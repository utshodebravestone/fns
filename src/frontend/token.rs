use std::fmt;

use super::utils::TextSpan;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Asterisk => write!(f, "*"),
            BinaryOperator::Slash => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof,

    Number,

    Identifier,

    Let,
    None,

    OpenParen,
    CloseParen,

    BinaryOperator(BinaryOperator),

    Equal,
}

impl TokenKind {
    pub fn get_lexeme_type(lexeme: &str) -> Self {
        match lexeme {
            "let" => TokenKind::Let,
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
            TokenKind::None => write!(f, "none"),

            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),

            TokenKind::BinaryOperator(b) => write!(f, "{b}"),

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
