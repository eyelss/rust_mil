use crate::lexer::tokens::Token;
use crate::shared::data_types::PrimitiveType;

pub enum Expression {
    Inner(CalculatableExpression),
    Leaf(LeafedExpression),
}

pub enum LeafedExpression {
    Literal(PrimitiveType),
    Identifier(Vec<String>)
}

pub enum CalculatableExpression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
}

pub struct BinaryExpression {
    pub operation: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt
}

pub struct UnaryExpression {
    pub operation: Token,
    pub argument: Box<Expression>,
}
