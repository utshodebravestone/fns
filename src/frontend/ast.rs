use super::{token::Token, utils::TextSpan};

pub type Program = Vec<Statement>;
pub type Number = f64;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Expression(Expression),
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
pub struct ConstStatement {
    pub keyword: Token,
    pub identifier: Token,
    pub expression: Expression,
}

impl ConstStatement {
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
    Boolean(BooleanLiteralExpression),
    Numeric(NumericLiteralExpression),
    String(StringLiteralExpression),
    Object(ObjectLiteralExpression),
    Identifier(IdentifierExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Assignment(AssignmentExpression),
}

impl Expression {
    pub fn text_span(&self) -> TextSpan {
        match self {
            Expression::None(n) => n.text_span(),
            Expression::Boolean(b) => b.text_span(),
            Expression::Numeric(n) => n.text_span(),
            Expression::String(s) => s.text_span(),
            Expression::Object(o) => o.text_span(),
            Expression::Identifier(i) => i.text_span(),
            Expression::Unary(u) => u.text_span(),
            Expression::Binary(b) => b.text_span(),
            Expression::Assignment(a) => a.text_span(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub identifier: Token,
    pub expression: Box<Expression>,
}

impl AssignmentExpression {
    pub fn new(identifier: Token, expression: Expression) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }

    pub fn text_span(&self) -> TextSpan {
        TextSpan::add(
            self.identifier.text_span.clone(),
            self.expression.text_span(),
        )
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

    pub fn text_span(&self) -> TextSpan {
        TextSpan::add(self.left.text_span(), self.right.text_span())
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn text_span(&self) -> TextSpan {
        TextSpan::add(self.operator.text_span.clone(), self.right.text_span())
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

    pub fn text_span(&self) -> TextSpan {
        self.identifier.text_span.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjectLiteralExpression {
    pub open_brace: Token,
    pub pairs: Vec<KeyValuePair>,
    pub close_brace: Token,
}

impl ObjectLiteralExpression {
    pub fn new(open_brace: Token, pairs: Vec<KeyValuePair>, close_brace: Token) -> Self {
        Self {
            open_brace,
            pairs,
            close_brace,
        }
    }

    pub fn text_span(&self) -> TextSpan {
        TextSpan::add(
            self.open_brace.text_span.clone(),
            self.close_brace.text_span.clone(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct StringLiteralExpression {
    pub string: Token,
    pub value: String,
}

impl StringLiteralExpression {
    pub fn new(string: Token, value: String) -> Self {
        Self { string, value }
    }

    pub fn text_span(&self) -> TextSpan {
        self.string.text_span.clone()
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

    pub fn text_span(&self) -> TextSpan {
        self.number.text_span.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanLiteralExpression {
    pub boolean: Token,
    pub value: bool,
}

impl BooleanLiteralExpression {
    pub fn new(boolean: Token, value: bool) -> Self {
        Self { boolean, value }
    }

    pub fn text_span(&self) -> TextSpan {
        self.boolean.text_span.clone()
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

    pub fn text_span(&self) -> TextSpan {
        self.none.text_span.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    pub key: Token,
    pub value: Expression,
}

impl KeyValuePair {
    pub fn new(key: Token, value: Expression) -> Self {
        Self { key, value }
    }
}
