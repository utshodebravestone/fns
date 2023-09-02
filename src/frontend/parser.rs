use super::{
    ast::{
        AccessExpression, AssignmentExpression, BinaryExpression, BooleanLiteralExpression,
        ConstStatement, Expression, IdentifierExpression, KeyValuePair, LetStatement,
        NoneLiteralExpression, NumericLiteralExpression, ObjectLiteralExpression, Program,
        Statement, StringLiteralExpression, UnaryExpression,
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
    parse_binary_logical_expression(tokens, current_token_index)
}

fn parse_binary_logical_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    let mut current_token_index = current_token_index;
    let (mut left, consumed_until) = parse_binary_equality_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        &tokens[current_token_index].kind,
        &[TokenKind::DoubleAmpersand, TokenKind::DoublePipe],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) = parse_binary_logical_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
}

fn parse_binary_equality_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    let mut current_token_index = current_token_index;
    let (mut left, consumed_until) =
        parse_binary_comparison_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        &tokens[current_token_index].kind,
        &[TokenKind::DoubleEqual, TokenKind::BangEqual],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) =
            parse_binary_equality_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
}

fn parse_binary_comparison_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    let mut current_token_index = current_token_index;
    let (mut left, consumed_until) = parse_binary_additive_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        &tokens[current_token_index].kind,
        &[
            TokenKind::Greater,
            TokenKind::Lesser,
            TokenKind::GreaterOrEqual,
            TokenKind::LesserOrEqual,
        ],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) =
            parse_binary_comparison_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
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
        &tokens[current_token_index].kind,
        &[TokenKind::Plus, TokenKind::Minus],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) =
            parse_binary_additive_expression(tokens, current_token_index)?;
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
    let (mut left, consumed_until) = parse_unary_expression(tokens, current_token_index)?;
    current_token_index = consumed_until;
    while token_matches(
        &tokens[current_token_index].kind,
        &[TokenKind::Asterisk, TokenKind::Slash],
    ) {
        let operator = tokens[current_token_index].clone();
        current_token_index += 1;
        let (right, consumed_until) =
            parse_binary_multiplicative_expression(tokens, current_token_index)?;
        current_token_index = consumed_until;
        left = Expression::Binary(BinaryExpression::new(left, operator, right));
    }

    Ok((left, current_token_index))
}

fn parse_unary_expression(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(Expression, usize), Error> {
    if token_matches(
        &tokens[current_token_index].kind,
        &[TokenKind::Bang, TokenKind::Plus, TokenKind::Minus],
    ) {
        let (operator, current_token_index) = eat_token(tokens, current_token_index);
        let (right, current_token_index) = parse_unary_expression(tokens, current_token_index)?;
        return Ok((
            Expression::Unary(UnaryExpression::new(operator, right)),
            current_token_index,
        ));
    }
    parse_primary_expression(tokens, current_token_index)
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
        TokenKind::True => Ok((
            Expression::Boolean(BooleanLiteralExpression::new(
                tokens[current_token_index].clone(),
                true,
            )),
            current_token_index + 1,
        )),
        TokenKind::False => Ok((
            Expression::Boolean(BooleanLiteralExpression::new(
                tokens[current_token_index].clone(),
                false,
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
        TokenKind::String => Ok((
            Expression::String(StringLiteralExpression::new(
                tokens[current_token_index].clone(),
                tokens[current_token_index].lexeme.parse().unwrap(),
            )),
            current_token_index + 1,
        )),
        TokenKind::OpenBrace => {
            let (open_brace, current_token_index) =
                expect_to_match(tokens, current_token_index, TokenKind::OpenBrace)?;
            let mut pairs = vec![];
            let mut global_current_token_index = current_token_index;
            while tokens[current_token_index].kind != TokenKind::CloseBrace {
                let current_token_index = global_current_token_index;
                let (pair, current_token_index) =
                    parse_key_value_pair(tokens, current_token_index)?;
                pairs.push(pair);
                if tokens[current_token_index].kind == TokenKind::CloseBrace {
                    global_current_token_index = current_token_index;
                    break;
                }
                let (_, current_token_index) =
                    expect_to_match(tokens, current_token_index, TokenKind::Comma)?;
                global_current_token_index = current_token_index;
            }
            let (close_brace, current_token_index) =
                expect_to_match(tokens, global_current_token_index, TokenKind::CloseBrace)?;
            Ok((
                Expression::Object(ObjectLiteralExpression::new(open_brace, pairs, close_brace)),
                current_token_index,
            ))
        }
        TokenKind::Identifier => {
            if tokens.get(current_token_index + 1).is_some()
                && tokens[current_token_index + 1].kind == TokenKind::Dot
            {
                let (object, current_token_index) =
                    expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
                let (_, current_token_index) =
                    expect_to_match(tokens, current_token_index, TokenKind::Dot)?;
                let (property, current_token_index) =
                    expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
                Ok((
                    Expression::Access(AccessExpression::new(object, property)),
                    current_token_index,
                ))
            } else {
                Ok((
                    Expression::Identifier(IdentifierExpression::new(
                        tokens[current_token_index].clone(),
                    )),
                    current_token_index + 1,
                ))
            }
        }
        _ => Err(Error::new(
            format!("Unexpected token '{}'", tokens[current_token_index].lexeme),
            tokens[current_token_index].text_span.clone(),
        )),
    }
}

fn parse_key_value_pair(
    tokens: &[Token],
    current_token_index: usize,
) -> Result<(KeyValuePair, usize), Error> {
    let (key, current_token_index) =
        expect_to_match(tokens, current_token_index, TokenKind::Identifier)?;
    let (_, current_token_index) = expect_to_match(tokens, current_token_index, TokenKind::Colon)?;
    let (value, current_token_index) = parse_expression(tokens, current_token_index)?;
    Ok((KeyValuePair::new(key, value), current_token_index))
}

fn token_matches(token_kind: &TokenKind, expected_to_be_in: &[TokenKind]) -> bool {
    expected_to_be_in.contains(token_kind)
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

fn eat_token(tokens: &[Token], current_token_index: usize) -> (Token, usize) {
    (tokens[current_token_index].clone(), current_token_index + 1)
}

#[cfg(test)]
mod tests {
    use crate::frontend::{
        ast::{
            AccessExpression, AssignmentExpression, BinaryExpression, BooleanLiteralExpression,
            ConstStatement, Expression, IdentifierExpression, KeyValuePair, LetStatement,
            NumericLiteralExpression, ObjectLiteralExpression, Statement, StringLiteralExpression,
            UnaryExpression,
        },
        parser::{
            parse_assignment_expression, parse_binary_expression, parse_const_statement,
            parse_key_value_pair, parse_let_statement, parse_primary_expression,
            parse_unary_expression,
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
    fn test_parse_binary_logical_expression() {
        let source_code = "a&&b||c";
        let expected_output = (
            Expression::Binary(BinaryExpression::new(
                Expression::Identifier(IdentifierExpression::new(Token::new(
                    TokenKind::Identifier,
                    "a".to_string(),
                    TextSpan::new(0, 1),
                ))),
                Token::new(
                    TokenKind::DoubleAmpersand,
                    "&&".to_string(),
                    TextSpan::new(1, 3),
                ),
                Expression::Binary(BinaryExpression::new(
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "b".to_string(),
                        TextSpan::new(3, 4),
                    ))),
                    Token::new(TokenKind::DoublePipe, "||".to_string(), TextSpan::new(4, 6)),
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "c".to_string(),
                        TextSpan::new(6, 7),
                    ))),
                )),
            )),
            5,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_binary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_binary_equality_expression() {
        let source_code = "a==b!=c";
        let expected_output = (
            Expression::Binary(BinaryExpression::new(
                Expression::Identifier(IdentifierExpression::new(Token::new(
                    TokenKind::Identifier,
                    "a".to_string(),
                    TextSpan::new(0, 1),
                ))),
                Token::new(
                    TokenKind::DoubleEqual,
                    "==".to_string(),
                    TextSpan::new(1, 3),
                ),
                Expression::Binary(BinaryExpression::new(
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "b".to_string(),
                        TextSpan::new(3, 4),
                    ))),
                    Token::new(TokenKind::BangEqual, "!=".to_string(), TextSpan::new(4, 6)),
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "c".to_string(),
                        TextSpan::new(6, 7),
                    ))),
                )),
            )),
            5,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_binary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_binary_comparison_expression() {
        let source_code = "a>b<c>=d<=e";
        let expected_output = (
            Expression::Binary(BinaryExpression::new(
                Expression::Identifier(IdentifierExpression::new(Token::new(
                    TokenKind::Identifier,
                    "a".to_string(),
                    TextSpan::new(0, 1),
                ))),
                Token::new(TokenKind::Greater, ">".to_string(), TextSpan::new(1, 2)),
                Expression::Binary(BinaryExpression::new(
                    Expression::Identifier(IdentifierExpression::new(Token::new(
                        TokenKind::Identifier,
                        "b".to_string(),
                        TextSpan::new(2, 3),
                    ))),
                    Token::new(TokenKind::Lesser, "<".to_string(), TextSpan::new(3, 4)),
                    Expression::Binary(BinaryExpression::new(
                        Expression::Identifier(IdentifierExpression::new(Token::new(
                            TokenKind::Identifier,
                            "c".to_string(),
                            TextSpan::new(4, 5),
                        ))),
                        Token::new(
                            TokenKind::GreaterOrEqual,
                            ">=".to_string(),
                            TextSpan::new(5, 7),
                        ),
                        Expression::Binary(BinaryExpression::new(
                            Expression::Identifier(IdentifierExpression::new(Token::new(
                                TokenKind::Identifier,
                                "d".to_string(),
                                TextSpan::new(7, 8),
                            ))),
                            Token::new(
                                TokenKind::LesserOrEqual,
                                "<=".to_string(),
                                TextSpan::new(8, 10),
                            ),
                            Expression::Identifier(IdentifierExpression::new(Token::new(
                                TokenKind::Identifier,
                                "e".to_string(),
                                TextSpan::new(10, 11),
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
    fn test_parse_binary_additive_and_multiplicative_expression() {
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
    fn test_parse_unary_expression() {
        let source_code = "!+-2.5";
        let expected_output = (
            Expression::Unary(UnaryExpression::new(
                Token::new(TokenKind::Bang, "!".to_string(), TextSpan::new(0, 1)),
                Expression::Unary(UnaryExpression::new(
                    Token::new(TokenKind::Plus, "+".to_string(), TextSpan::new(1, 2)),
                    Expression::Unary(UnaryExpression::new(
                        Token::new(TokenKind::Minus, "-".to_string(), TextSpan::new(2, 3)),
                        Expression::Numeric(NumericLiteralExpression::new(
                            Token::new(TokenKind::Number, "2.5".to_string(), TextSpan::new(3, 6)),
                            2.5,
                        )),
                    )),
                )),
            )),
            4,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_unary_expression(&tokens, 0).unwrap();
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
    fn test_parse_primary_boolean_true_expression() {
        let source_code = "true";
        let expected_output = (
            Expression::Boolean(BooleanLiteralExpression::new(
                Token::new(TokenKind::True, "true".to_string(), TextSpan::new(0, 4)),
                true,
            )),
            1,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_primary_boolean_false_expression() {
        let source_code = "false";
        let expected_output = (
            Expression::Boolean(BooleanLiteralExpression::new(
                Token::new(TokenKind::False, "false".to_string(), TextSpan::new(0, 5)),
                false,
            )),
            1,
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
    fn test_parse_primary_string_expression() {
        let source_code = "\"hello, world\"";
        let expected_output = (
            Expression::String(StringLiteralExpression::new(
                Token::new(
                    TokenKind::String,
                    "hello, world".to_string(),
                    TextSpan::new(0, 14),
                ),
                "hello, world".to_string(),
            )),
            1,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_primary_object_expression() {
        let source_code = "{name: \"fns\", works: true}";
        let expected_output = (
            Expression::Object(ObjectLiteralExpression::new(
                Token::new(TokenKind::OpenBrace, "{".to_string(), TextSpan::new(0, 1)),
                vec![
                    KeyValuePair::new(
                        Token::new(
                            TokenKind::Identifier,
                            "name".to_string(),
                            TextSpan::new(1, 5),
                        ),
                        Expression::String(StringLiteralExpression::new(
                            Token::new(TokenKind::String, "fns".to_string(), TextSpan::new(7, 12)),
                            "fns".to_string(),
                        )),
                    ),
                    KeyValuePair::new(
                        Token::new(
                            TokenKind::Identifier,
                            "works".to_string(),
                            TextSpan::new(14, 19),
                        ),
                        Expression::Boolean(BooleanLiteralExpression::new(
                            Token::new(TokenKind::True, "true".to_string(), TextSpan::new(21, 25)),
                            true,
                        )),
                    ),
                ],
                Token::new(
                    TokenKind::CloseBrace,
                    "}".to_string(),
                    TextSpan::new(25, 26),
                ),
            )),
            9,
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

    #[test]
    fn test_parse_access_expression() {
        let source_code = "lang.name";
        let expected_output = (
            Expression::Access(AccessExpression::new(
                Token::new(
                    TokenKind::Identifier,
                    "lang".to_string(),
                    TextSpan::new(0, 4),
                ),
                Token::new(
                    TokenKind::Identifier,
                    "name".to_string(),
                    TextSpan::new(5, 9),
                ),
            )),
            3,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_primary_expression(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_parse_key_value_pair() {
        let source_code = "works: true";
        let expected_output = (
            KeyValuePair::new(
                Token::new(
                    TokenKind::Identifier,
                    "works".to_string(),
                    TextSpan::new(0, 5),
                ),
                Expression::Boolean(BooleanLiteralExpression::new(
                    Token::new(TokenKind::True, "true".to_string(), TextSpan::new(7, 11)),
                    true,
                )),
            ),
            3,
        );
        let tokens = tokenize(source_code).unwrap();
        let output = parse_key_value_pair(&tokens, 0).unwrap();
        assert_eq!(expected_output, output);
    }
}
