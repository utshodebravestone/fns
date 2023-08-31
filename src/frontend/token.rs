use std::fmt;

use super::utils::TextSpan;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof,

    Number,

    Identifier,

    Let,
    Const,
    True,
    False,
    None,

    OpenParen,
    CloseParen,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
}

impl TokenKind {
    pub fn get_lexeme_type(lexeme: &str) -> Self {
        match lexeme {
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "none" => TokenKind::None,
            _ => TokenKind::Identifier,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "\0"),

            TokenKind::Number => write!(f, "NUMBER"),

            TokenKind::Identifier => write!(f, "IDENTIFIER"),

            TokenKind::Let => write!(f, "let"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::None => write!(f, "none"),

            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),

            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),

            TokenKind::Equal => write!(f, "="),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub text_span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, text_span: TextSpan) -> Self {
        Self {
            kind,
            lexeme,
            text_span,
        }
    }
}
