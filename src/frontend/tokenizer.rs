use super::{
    token::{Token, TokenKind},
    utils::{Error, TextSpan},
};

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let source_code: Vec<char> = format!("{source_code}\0").chars().collect();
    let mut starting_index = 0;
    let mut current_index = 0;

    while current_index < source_code.len() {
        let current_char = source_code[current_index];
        current_index += 1;

        match current_char {
            ' ' | '\t' | '\n' | '\r' => {}

            '\0' => tokens.push(Token::new(
                TokenKind::Eof,
                "\0".to_string(),
                TextSpan::new(starting_index, current_index),
            )),

            ':' => tokens.push(Token::new(
                TokenKind::Colon,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            ',' => tokens.push(Token::new(
                TokenKind::Comma,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            '.' => tokens.push(Token::new(
                TokenKind::Dot,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),

            '(' => tokens.push(Token::new(
                TokenKind::OpenParen,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            ')' => tokens.push(Token::new(
                TokenKind::CloseParen,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            '{' => tokens.push(Token::new(
                TokenKind::OpenBrace,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            '}' => tokens.push(Token::new(
                TokenKind::CloseBrace,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),

            '+' => tokens.push(Token::new(
                TokenKind::Plus,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            '-' => tokens.push(Token::new(
                TokenKind::Minus,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),
            '*' => tokens.push(Token::new(
                TokenKind::Asterisk,
                source_code[starting_index..current_index].iter().collect(),
                TextSpan::new(starting_index, current_index),
            )),

            '/' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '/' {
                    while current_index < source_code.len()
                        && source_code[current_index] != '\n'
                        && source_code[current_index] != '\0'
                    {
                        current_index += 1;
                    }
                } else {
                    tokens.push(Token::new(
                        TokenKind::Slash,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '&' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '&' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::DoubleAmpersand,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Ampersand,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '|' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '|' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::DoublePipe,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Pipe,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '=' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '=' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::DoubleEqual,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Equal,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '!' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '=' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::BangEqual,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Bang,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '>' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '=' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::GreaterOrEqual,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Greater,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
            '<' => {
                if source_code.get(current_index).is_some() && source_code[current_index] == '=' {
                    current_index += 1;
                    tokens.push(Token::new(
                        TokenKind::LesserOrEqual,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    tokens.push(Token::new(
                        TokenKind::Lesser,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }

            '"' => {
                loop {
                    if source_code[current_index] == '"' {
                        current_index += 1;
                        break;
                    }
                    if source_code[current_index] == '\0' {
                        return Err(Error::new(
                            "Unterminated string".to_string(),
                            TextSpan::new(starting_index, current_index),
                        ));
                    }

                    current_index += 1;
                }
                tokens.push(Token::new(
                    TokenKind::String,
                    source_code[starting_index + 1..current_index - 1]
                        .iter()
                        .collect(),
                    TextSpan::new(starting_index, current_index),
                ));
            }

            _ => {
                if current_char.is_alphabetic() || current_char == '_' {
                    while source_code[current_index].is_alphanumeric()
                        || source_code[current_index] == '_'
                    {
                        current_index += 1;
                    }
                    let lexeme: String =
                        source_code[starting_index..current_index].iter().collect();
                    tokens.push(Token::new(
                        TokenKind::get_lexeme_type(&lexeme),
                        lexeme,
                        TextSpan::new(starting_index, current_index),
                    ));
                } else if current_char.is_ascii_digit() {
                    while source_code[current_index].is_ascii_digit()
                        || source_code[current_index] == '.'
                    {
                        current_index += 1;
                    }
                    tokens.push(Token::new(
                        TokenKind::Number,
                        source_code[starting_index..current_index].iter().collect(),
                        TextSpan::new(starting_index, current_index),
                    ));
                } else {
                    return Err(Error::new(
                        format!("Unexpected character '{current_char}'"),
                        TextSpan::new(starting_index, current_index),
                    ));
                }
            }
        }
        starting_index = current_index;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::frontend::{
        token::{Token, TokenKind},
        utils::TextSpan,
    };

    use super::tokenize;

    #[test]
    fn test_tokenize() {
        let source_code = "let number = 2.5";
        let expected_tokens = vec![
            Token::new(TokenKind::Let, "let".to_string(), TextSpan::new(0, 3)),
            Token::new(
                TokenKind::Identifier,
                "number".to_string(),
                TextSpan::new(4, 10),
            ),
            Token::new(TokenKind::Equal, "=".to_string(), TextSpan::new(11, 12)),
            Token::new(TokenKind::Number, "2.5".to_string(), TextSpan::new(13, 16)),
            Token::new(TokenKind::Eof, "\0".to_string(), TextSpan::new(16, 17)),
        ];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_keywords() {
        let source_code = "let const true false none";
        let expected_tokens = vec![
            Token::new(TokenKind::Let, "let".to_string(), TextSpan::new(0, 3)),
            Token::new(TokenKind::Const, "const".to_string(), TextSpan::new(4, 9)),
            Token::new(TokenKind::True, "true".to_string(), TextSpan::new(10, 14)),
            Token::new(TokenKind::False, "false".to_string(), TextSpan::new(15, 20)),
            Token::new(TokenKind::None, "none".to_string(), TextSpan::new(21, 25)),
            Token::new(TokenKind::Eof, "\0".to_string(), TextSpan::new(25, 26)),
        ];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_string_token() {
        let source_code = "\"hello, world!\"";
        let expected_tokens = vec![
            Token::new(
                TokenKind::String,
                "hello, world!".to_string(),
                TextSpan::new(0, 15),
            ),
            Token::new(TokenKind::Eof, "\0".to_string(), TextSpan::new(15, 16)),
        ];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index + 1..token.text_span.ending_index - 1]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_single_and_double_character_tokens() {
        let source_code = "=(+-*/)!&&&|||><>=<===!={}:,.";
        let expected_tokens = vec![
            Token::new(TokenKind::Equal, "=".to_string(), TextSpan::new(0, 1)),
            Token::new(TokenKind::OpenParen, "(".to_string(), TextSpan::new(1, 2)),
            Token::new(TokenKind::Plus, "+".to_string(), TextSpan::new(2, 3)),
            Token::new(TokenKind::Minus, "-".to_string(), TextSpan::new(3, 4)),
            Token::new(TokenKind::Asterisk, "*".to_string(), TextSpan::new(4, 5)),
            Token::new(TokenKind::Slash, "/".to_string(), TextSpan::new(5, 6)),
            Token::new(TokenKind::CloseParen, ")".to_string(), TextSpan::new(6, 7)),
            Token::new(TokenKind::Bang, "!".to_string(), TextSpan::new(7, 8)),
            Token::new(
                TokenKind::DoubleAmpersand,
                "&&".to_string(),
                TextSpan::new(8, 10),
            ),
            Token::new(TokenKind::Ampersand, "&".to_string(), TextSpan::new(10, 11)),
            Token::new(
                TokenKind::DoublePipe,
                "||".to_string(),
                TextSpan::new(11, 13),
            ),
            Token::new(TokenKind::Pipe, "|".to_string(), TextSpan::new(13, 14)),
            Token::new(TokenKind::Greater, ">".to_string(), TextSpan::new(14, 15)),
            Token::new(TokenKind::Lesser, "<".to_string(), TextSpan::new(15, 16)),
            Token::new(
                TokenKind::GreaterOrEqual,
                ">=".to_string(),
                TextSpan::new(16, 18),
            ),
            Token::new(
                TokenKind::LesserOrEqual,
                "<=".to_string(),
                TextSpan::new(18, 20),
            ),
            Token::new(
                TokenKind::DoubleEqual,
                "==".to_string(),
                TextSpan::new(20, 22),
            ),
            Token::new(
                TokenKind::BangEqual,
                "!=".to_string(),
                TextSpan::new(22, 24),
            ),
            Token::new(TokenKind::OpenBrace, "{".to_string(), TextSpan::new(24, 25)),
            Token::new(
                TokenKind::CloseBrace,
                "}".to_string(),
                TextSpan::new(25, 26),
            ),
            Token::new(TokenKind::Colon, ":".to_string(), TextSpan::new(26, 27)),
            Token::new(TokenKind::Comma, ",".to_string(), TextSpan::new(27, 28)),
            Token::new(TokenKind::Dot, ".".to_string(), TextSpan::new(28, 29)),
            Token::new(TokenKind::Eof, "\0".to_string(), TextSpan::new(29, 30)),
        ];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_empty_input() {
        let source_code = "";
        let expected_tokens = vec![Token::new(
            TokenKind::Eof,
            "\0".to_string(),
            TextSpan::new(0, 1),
        )];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let source_code = "  \t \n \r";
        let expected_tokens = vec![Token::new(
            TokenKind::Eof,
            "\0".to_string(),
            TextSpan::new(7, 8),
        )];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }

    #[test]
    fn test_tokenize_with_comment() {
        let source_code = "// this is a comment";
        let expected_tokens = vec![Token::new(
            TokenKind::Eof,
            "\0".to_string(),
            TextSpan::new(20, 21),
        )];
        let tokens = tokenize(source_code).unwrap();
        assert_eq!(tokens, expected_tokens);
        for i in 0..expected_tokens.len() - 1 {
            let token = &tokens[i];
            assert_eq!(
                token.lexeme,
                source_code[token.text_span.starting_index..token.text_span.ending_index]
                    .to_string()
            );
        }
    }
}
