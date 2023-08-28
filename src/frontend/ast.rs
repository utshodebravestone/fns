use super::{token::Token, utils::TextSpan};

pub type Program = Vec<Statement>;
pub type Number = f64;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Expression(Expression),
}

impl Statement {
    pub fn text_span(&self) -> TextSpan {
        match self {
            Statement::Let(ls) => {
                TextSpan::add(ls.keyword.text_span.clone(), ls.expression.text_span())
            }
            Statement::Expression(e) => e.text_span(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub keyword: Token,
    pub identifier: Token,
    pub expression: Expression,
}

impl LetStatement {
    pub fn new(keyword: Token, identifier: Token, expression: Expression) -> Self {
        Self {
            keyword,
            identifier,
            expression,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Expression {
    None(NoneLiteralExpression),
    Numeric(NumericLiteralExpression),
    Identifier(IdentifierExpression),
    Binary(BinaryExpression),
}

impl Expression {
    pub fn text_span(&self) -> TextSpan {
        match self {
            Expression::None(n) => n.none.text_span.clone(),
            Expression::Numeric(n) => n.number.text_span.clone(),
            Expression::Identifier(i) => i.identifier.text_span.clone(),
            Expression::Binary(b) => TextSpan::add(b.left.text_span(), b.right.text_span()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub identifier: Token,
}

impl IdentifierExpression {
    pub fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}

#[derive(Debug, PartialEq)]
pub struct NumericLiteralExpression {
    pub number: Token,
    pub value: Number,
}

impl NumericLiteralExpression {
    pub fn new(number: Token, value: Number) -> Self {
        Self { number, value }
    }
}

#[derive(Debug, PartialEq)]
pub struct NoneLiteralExpression {
    pub none: Token,
}

impl NoneLiteralExpression {
    pub fn new(none: Token) -> Self {
        Self { none }
    }
}
