use std::collections::HashMap;

use crate::frontend::{
    ast::{ConstStatement, Expression, LetStatement, Program, Statement},
    token::TokenKind,
    utils::Error,
};

use super::{environment::Environment, types::Value};

pub fn evaluate(
    program: Program,
    parent: Option<Environment>,
) -> Result<(Value, Environment), Error> {
    let mut value = Value::None;
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
        Expression::Boolean(b) => Ok(Value::Boolean(b.value)),
        Expression::Numeric(n) => Ok(Value::Number(n.value)),
        Expression::String(s) => Ok(Value::String(s.value.clone())),
        Expression::Object(o) => {
            let mut pairs = vec![];
            for pair in &o.pairs {
                pairs.push((
                    pair.key.lexeme.clone(),
                    Box::new(evaluate_expression(&pair.value, environment)?),
                ));
            }
            Ok(Value::Object(HashMap::from_iter(pairs)))
        }
        Expression::Access(a) => {
            let value = evaluate_expression(&a.object, environment)?;
            if let Value::Object(object) = value {
                if let Some(value) = object.get(&a.property.lexeme) {
                    Ok(*value.clone())
                } else {
                    Err(Error::new(
                        format!(
                            "Can't access the property '{}' as it's not defined",
                            a.property.lexeme
                        ),
                        a.text_span(),
                    ))
                }
            } else {
                Err(Error::new(
                    format!(
                        "Can't access property of '{}' as it's not accessible",
                        value
                    ),
                    a.text_span(),
                ))
            }
        }
        Expression::Identifier(i) => {
            if let Some(value) = environment.access(&i.identifier.lexeme) {
                Ok(value)
            } else {
                Err(Error::new(
                    format!(
                        "Can't access the variable '{}' as it's not defined",
                        i.identifier.lexeme
                    ),
                    i.text_span(),
                ))
            }
        }
        Expression::Unary(u) => {
            let right = evaluate_expression(&u.right, environment)?;
            match (u.operator.kind.clone(), right) {
                (TokenKind::Bang, Value::Boolean(a)) => Ok(Value::Boolean(!a)),

                (TokenKind::Plus, Value::Number(a)) => Ok(Value::Number(a)),
                (TokenKind::Minus, Value::Number(a)) => Ok(Value::Number(-a)),

                (operator, right) => Err(Error::new(
                    format!("Can't use '{operator}' with '{right}'"),
                    u.text_span(),
                )),
            }
        }
        Expression::Binary(b) => {
            let left = evaluate_expression(&b.left, environment)?;
            let right = evaluate_expression(&b.right, environment)?;
            match (b.operator.kind.clone(), left, right) {
                (TokenKind::Plus, Value::String(left), Value::String(right)) => {
                    Ok(Value::String(left + &right))
                }

                (TokenKind::Plus, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left + right))
                }
                (TokenKind::Minus, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left - right))
                }
                (TokenKind::Asterisk, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Number(left * right))
                }
                (TokenKind::Slash, Value::Number(left), Value::Number(right)) => {
                    if right == 0. {
                        Err(Error::new("Can't divide by 0".to_string(), b.text_span()))
                    } else {
                        Ok(Value::Number(left / right))
                    }
                }

                (TokenKind::Greater, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Boolean(left > right))
                }
                (TokenKind::Lesser, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Boolean(left < right))
                }
                (TokenKind::GreaterOrEqual, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Boolean(left >= right))
                }
                (TokenKind::LesserOrEqual, Value::Number(left), Value::Number(right)) => {
                    Ok(Value::Boolean(left <= right))
                }

                (TokenKind::BangEqual, left, right) => Ok(Value::Boolean(left != right)),
                (TokenKind::DoubleEqual, left, right) => Ok(Value::Boolean(left == right)),

                (TokenKind::DoubleAmpersand, Value::Boolean(left), Value::Boolean(right)) => {
                    Ok(Value::Boolean(left && right))
                }
                (TokenKind::DoublePipe, Value::Boolean(left), Value::Boolean(right)) => {
                    Ok(Value::Boolean(left || right))
                }

                (operator, left, right) => Err(Error::new(
                    format!("Can't use '{operator}' with '{left}' and '{right}'"),
                    b.text_span(),
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
    use std::collections::HashMap;

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
    fn test_evaluate_binary_arithmetic_expression() {
        let src = "5 + 5 * 2 / 5 - 2";
        let expected_value = Value::Number(5.);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_binary_comparison_expression() {
        let src = "5 > 5";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "5 < 5";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "5 >= 5";
        let expected_value = Value::Boolean(true);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "5 <= 5";
        let expected_value = Value::Boolean(true);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_binary_equality_expression() {
        let src = "5 == 5 != 5";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_binary_logical_expression() {
        let src = "true && false || !true";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_string_concatenation_expression() {
        let src = "\"hello, \" + \"world!\"";
        let expected_value = Value::String("hello, world!".to_string());
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_unary_expression() {
        let src = "--+-5";
        let expected_value = Value::Number(-5.);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);

        let src = "!true";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_access_expression() {
        let src = "{wip: true}.wip";
        let expected_value = Value::Boolean(true);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_object_expression() {
        let src = "{name: \"fns\", paradigm: \"functional\", wip: true}";
        let expected_value = Value::Object(HashMap::from_iter(vec![
            (
                "name".to_string(),
                Box::new(Value::String("fns".to_string())),
            ),
            (
                "paradigm".to_string(),
                Box::new(Value::String("functional".to_string())),
            ),
            ("wip".to_string(), Box::new(Value::Boolean(true))),
        ]));
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_string_expression() {
        let src = "\"hello, world!\"";
        let expected_value = Value::String("hello, world!".to_string());
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

    #[test]
    fn test_evaluate_boolean_true_expression() {
        let src = "true";
        let expected_value = Value::Boolean(true);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_boolean_false_expression() {
        let src = "false";
        let expected_value = Value::Boolean(false);
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }

    #[test]
    fn test_evaluate_none_expression() {
        let src = "none";
        let expected_value = Value::None;
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        let (val, _) = evaluate(program, None).unwrap();
        assert_eq!(val, expected_value);
    }
}
