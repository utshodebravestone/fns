use crate::frontend::{
    ast::{ConstStatement, Expression, LetStatement, Program, Statement},
    token::{BinaryOperator, TokenKind},
    utils::Error,
};

use super::{environment::Environment, types::Value};

pub fn evaluate(
    program: Program,
    parent: Option<Environment>,
) -> Result<(Value, Environment), Error> {
    let mut value = Value::Number(0.);
    let mut environment = Environment::new(parent);

    for statement in program {
        value = evaluate_statement(statement, &mut environment)?;
    }
    Ok((value, environment))
}

fn evaluate_statement(statement: Statement, environment: &mut Environment) -> Result<Value, Error> {
    match statement {
        Statement::Let(l) => evaluate_let_statement(l, environment),
        Statement::Const(c) => evaluate_const_statement(c, environment),
        Statement::Expression(e) => evaluate_expression(&e, environment),
    }
}

fn evaluate_let_statement(
    statement: LetStatement,
    environment: &mut Environment,
) -> Result<Value, Error> {
    let value = evaluate_expression(&statement.expression, environment)?;
    environment.define(statement.identifier.lexeme, value, false);
    Ok(Value::None)
}

fn evaluate_const_statement(
    statement: ConstStatement,
    environment: &mut Environment,
) -> Result<Value, Error> {
    let value = evaluate_expression(&statement.expression, environment)?;
    environment.define(statement.identifier.lexeme, value, true);
    Ok(Value::None)
}

fn evaluate_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Value, Error> {
    match expression {
        Expression::None(_) => Ok(Value::None),
        Expression::Numeric(n) => Ok(Value::Number(n.value)),
        Expression::Identifier(i) => {
            if let Some(value) = environment.access(&i.identifier.lexeme) {
                Ok(value)
            } else {
                Err(Error::new(
                    format!(
                        "Can't access the variable '{}' as it's not defined",
                        i.identifier.lexeme
                    ),
                    expression.text_span(),
                ))
            }
        }
        Expression::Binary(b) => {
            let left = evaluate_expression(&b.left, environment)?;
            let right = evaluate_expression(&b.right, environment)?;
            match (b.operator.kind.clone(), left, right) {
                (
                    TokenKind::BinaryOperator(BinaryOperator::Plus),
                    Value::Number(left),
                    Value::Number(right),
                ) => Ok(Value::Number(left + right)),
                (
                    TokenKind::BinaryOperator(BinaryOperator::Minus),
                    Value::Number(left),
                    Value::Number(right),
                ) => Ok(Value::Number(left - right)),
                (
                    TokenKind::BinaryOperator(BinaryOperator::Asterisk),
                    Value::Number(left),
                    Value::Number(right),
                ) => Ok(Value::Number(left * right)),
                (
                    TokenKind::BinaryOperator(BinaryOperator::Slash),
                    Value::Number(left),
                    Value::Number(right),
                ) => {
                    if right == 0. {
                        Err(Error::new(
                            "Can't divide by 0".to_string(),
                            expression.text_span(),
                        ))
                    } else {
                        Ok(Value::Number(left / right))
                    }
                }

                (operator, left, right) => Err(Error::new(
                    format!("Can't use '{operator}' with '{left}' and '{right}'"),
                    expression.text_span(),
                )),
            }
        }
        Expression::Assignment(a) => {
            if let Some(is_constant) = environment.is_constant(&a.identifier.lexeme) {
                if is_constant {
                    Err(Error::new(
                        format!(
                            "Can't assign the variable '{}' as it's a constant",
                            a.identifier.lexeme
                        ),
                        a.text_span(),
                    ))
                } else {
                    let value = evaluate_expression(&a.expression, environment)?;
                    environment.define(a.identifier.lexeme.clone(), value.clone(), false);
                    Ok(value)
                }
            } else {
                Err(Error::new(
                    format!(
                        "Can't assign to the variable '{}' as it's not defined",
                        a.identifier.lexeme
                    ),
                    a.text_span(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::{parser::parse, tokenizer::tokenize},
        runtime::types::Value,
    };

    use super::evaluate;

    #[test]
    fn test_evaluate_let_statement() {
        let src = "let a = 2.71";
        let expected_value = Value::None;
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, env) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "a";
        let expected_value = Value::Number(2.71);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, Some(env)).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_const_statement() {
        let src = "let E = 2.71";
        let expected_value = Value::None;
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, env) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "E";
        let expected_value = Value::Number(2.71);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, Some(env)).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_assignment_expression_with_let() {
        let src = "let a = 2.71";
        let expected_value = Value::None;
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, env) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "a = 5";
        let expected_value = Value::Number(5.);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, Some(env)).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_assignment_expression_with_const() {
        let src = "const a = 2.71";
        let expected_value = Value::None;
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, env) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "a = 5";
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        assert!(evaluate(program, Some(env)).is_err());
    }

    #[test]
    fn test_evaluate_binary_expression() {
        let src = "5 + 5";
        let expected_value = Value::Number(10.);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_numeric_expression() {
        let src = "5";
        let expected_value = Value::Number(5.);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }
}
