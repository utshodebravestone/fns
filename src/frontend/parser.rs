use super::{
    ast::{
        AssignmentExpression, BinaryExpression, ConstStatement, Expression, IdentifierExpression,
        LetStatement, NoneLiteralExpression, NumericLiteralExpression, Program, Statement,
    },
    token::{Token, TokenKind},
    utils::Error,
};

pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut program = vec![];
    let mut current_token_index = 0;

    while current_token_index < tokens.len() {
        if tokens[current_token_index].kind == TokenKind::Eof {
            break;
        }
        let (statement, consumed_until) = parse_statement(&tokens, current_token_index)?;
        current_token_index = consumed_until;
        program.push(statement);
    }

    Ok(program)
}

fn parse_statement(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Statement, usize), Error> {
    match tokens[current_token_index].kind {
        TokenKind::Let => parse_let_statement(tokens, current_token_index),
        TokenKind::Const => parse_const_statement(tokens, current_token_index),
        _ => {
            let (expression, current_token_index) = parse_expression(tokens, current_token_index)?;
            Ok((Statement::Expression(expression), current_token_index))
        }
    }
}

fn parse_let_statement(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Statement, usize), Error> {
    let (keyword, current_token_index) =
        expect_to_match(tokens, current_token_index, TokenKind::Let)?;
    let (identifier, current_token_index) =
        expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
    let (_, current_token_index) = expect_to_match(tokens, current_token_index, TokenKind::Equal)?;
    let (expression, current_token_index) = parse_expression(tokens, current_token_index)?;
    Ok((
        Statement::Let(LetStatement::new(keyword, identifier, expression)),
        current_token_index,
    ))
}

fn parse_const_statement(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Statement, usize), Error> {
    let (keyword, current_token_index) =
        expect_to_match(tokens, current_token_index, TokenKind::Const)?;
    let (identifier, current_token_index) =
        expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
    let (_, current_token_index) = expect_to_match(tokens, current_token_index, TokenKind::Equal)?;
    let (expression, current_token_index) = parse_expression(tokens, current_token_index)?;
    Ok((
        Statement::Const(ConstStatement::new(keyword, identifier, expression)),
        current_token_index,
    ))
}

fn parse_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    parse_assignment_expression(tokens, current_token_index)
}

fn parse_assignment_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    if tokens.get(current_token_index + 1).is_some()
        && tokens[current_token_index].kind == TokenKind::Identifier
        && tokens[current_token_index + 1].kind == TokenKind::Equal
    {
        let (identifier, current_token_index) =
            expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
        let (_, current_token_index) =
            expect_to_match(tokens, current_token_index, TokenKind::Equal)?;
        let (expression, current_token_index) =
            parse_assignment_expression(tokens, current_token_index)?;
        Ok((
            Expression::Assignment(AssignmentExpression::new(identifier, expression)),
            current_token_index,
        ))
    } else {
        parse_binary_expression(tokens, current_token_index)
    }
}

fn parse_binary_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    parse_binary_additive_expression(tokens, current_token_index)
}

fn parse_binary_additive_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    let mut current_token_index = current_token_index;
    let (mut left, consumed_until) =
        parse_binary_multiplicative_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        tokens[current_token_index].kind.clone(),
        &[TokenKind::Plus, TokenKind::Minus],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) = parse_binary_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
}

fn parse_binary_multiplicative_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    let mut current_token_index = current_token_index;
    let (mut left, consumed_until) = parse_primary_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        tokens[current_token_index].kind.clone(),
        &[TokenKind::Asterisk, TokenKind::Slash],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) = parse_binary_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
}

fn parse_primary_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    match tokens[current_token_index].kind {
        TokenKind::OpenParen => {
            let (_, current_token_index) =
                expect_to_match(tokens, current_token_index, TokenKind::OpenParen)?;
            let (expression, current_token_index) = parse_expression(tokens, current_token_index)?;
            let (_, current_token_index) =
                expect_to_match(tokens, current_token_index, TokenKind::CloseParen)?;
            Ok((expression, current_token_index))
        }
        TokenKind::None => Ok((
            Expression::None(NoneLiteralExpression::new(
                tokens[current_token_index].clone(),
            )),
            current_token_index + 1,
        )),
        TokenKind::Number => Ok((
            Expression::Numeric(NumericLiteralExpression::new(
                tokens[current_token_index].clone(),
                tokens[current_token_index].lexeme.parse().unwrap(),
            )),
            current_token_index + 1,
        )),
        TokenKind::Identifier => Ok((
            Expression::Identifier(IdentifierExpression::new(
                tokens[current_token_index].clone(),
            )),
            current_token_index + 1,
        )),
        _ => Err(Error::new(
            format!("Unexpected token '{}'", tokens[current_token_index].lexeme),
            tokens[current_token_index].text_span.clone(),
        )),
    }
}

fn token_matches(token: TokenKind, expected_to_be_in: &[TokenKind]) -> bool {
    expected_to_be_in.contains(&token)
}

fn expect_to_match(
    tokens: &[Token],
    current_token_index: usize,
    expected: TokenKind,
) -> Result<(Token, usize), Error> {
    if tokens[current_token_index].kind == expected {
        Ok((tokens[current_token_index].clone(), current_token_index + 1))
    } else {
        Err(Error::new(
            format!(
                "Unexpected token '{}', expected '{}'",
                tokens[current_token_index].lexeme, expected
            ),
            tokens[current_token_index].text_span.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::{
        ast::{
            AssignmentExpression, BinaryExpression, ConstStatement, Expression,
            IdentifierExpression, LetStatement, NumericLiteralExpression, Statement,
        },
        parser::{
            parse_assignment_expression, parse_binary_expression, parse_const_statement,
            parse_let_statement, parse_primary_expression,
        },
        token::{Token, TokenKind},
        tokenizer::tokenize,
        utils::TextSpan,
    };

    #[test]
    fn test_parse_let_statement() {
        let source_code = "let a = 2.5";
        let expected_output = (
            Statement::Let(LetStatement::new(
                Token::new(TokenKind::Let, "let".to_string(), TextSpan::new(0, 3)),
                Token::new(TokenKind::Identifier, "a".to_string(), TextSpan::new(4, 5)),
                Expression::Numeric(NumericLiteralExpression::new(
                    Token::new(TokenKind::Number, "2.5".to_string(), TextSpan::new(8, 11)),
                    2.5,
                )),
            )),
            4,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_let_statement(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_const_statement() {
        let source_code = "const PI = 3.14159";
        let expected_output = (
            Statement::Const(ConstStatement::new(
                Token::new(TokenKind::Const, "const".to_string(), TextSpan::new(0, 5)),
                Token::new(TokenKind::Identifier, "PI".to_string(), TextSpan::new(6, 8)),
                Expression::Numeric(NumericLiteralExpression::new(
                    Token::new(
                        TokenKind::Number,
                        "3.14159".to_string(),
                        TextSpan::new(11, 18),
                    ),
                    3.14159,
                )),
            )),
            4,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_const_statement(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_assignment_expression() {
        let source_code = "a = 2.5";
        let expected_output = (
            Expression::Assignment(AssignmentExpression::new(
                Token::new(TokenKind::Identifier, "a".to_string(), TextSpan::new(0, 1)),
                Expression::Numeric(NumericLiteralExpression::new(
                    Token::new(TokenKind::Number, "2.5".to_string(), TextSpan::new(4, 7)),
                    2.5,
                )),
            )),
            3,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_assignment_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_binary_expression() {
        let source_code = "a+b-c*d/e";
        let expected_output = (
            Expression::Binary(BinaryExpression::new(
                Expression::Identifier(IdentifierExpression::new(Token::new(
                    TokenKind::Identifier,
                    "a".to_string(),
                    TextSpan::new(0, 1),
                ))),
                Token::new(TokenKind::Plus, "+".to_string(), TextSpan::new(1, 2)),
                Expression::Binary(BinaryExpression::new(
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "b".to_string(),
                        TextSpan::new(2, 3),
                    ))),
                    Token::new(TokenKind::Minus, "-".to_string(), TextSpan::new(3, 4)),
                    Expression::Binary(BinaryExpression::new(
                        Expression::Identifier(IdentifierExpression::new(Token::new(
                            TokenKind::Identifier,
                            "c".to_string(),
                            TextSpan::new(4, 5),
                        ))),
                        Token::new(TokenKind::Asterisk, "*".to_string(), TextSpan::new(5, 6)),
                        Expression::Binary(BinaryExpression::new(
                            Expression::Identifier(IdentifierExpression::new(Token::new(
                                TokenKind::Identifier,
                                "d".to_string(),
                                TextSpan::new(6, 7),
                            ))),
                            Token::new(TokenKind::Slash, "/".to_string(), TextSpan::new(7, 8)),
                            Expression::Identifier(IdentifierExpression::new(Token::new(
                                TokenKind::Identifier,
                                "e".to_string(),
                                TextSpan::new(8, 9),
                            ))),
                        )),
                    )),
                )),
            )),
            9,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_binary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_primary_parenthesized_expression() {
        let source_code = "(a)";
        let expected_output = (
            Expression::Identifier(IdentifierExpression::new(Token::new(
                TokenKind::Identifier,
                "a".to_string(),
                TextSpan::new(1, 2),
            ))),
            3,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_primary_numeric_expression() {
        let source_code = "2.5";
        let expected_output = (
            Expression::Numeric(NumericLiteralExpression::new(
                Token::new(TokenKind::Number, "2.5".to_string(), TextSpan::new(0, 3)),
                2.5,
            )),
            1,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_primary_identifier_expression() {
        let source_code = "a";
        let expected_output = (
            Expression::Identifier(IdentifierExpression::new(Token::new(
                TokenKind::Identifier,
                "a".to_string(),
                TextSpan::new(0, 1),
            ))),
            1,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }
}
