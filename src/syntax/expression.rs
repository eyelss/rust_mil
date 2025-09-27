use crate::shared::data_types::PrimitiveType;
use crate::lexer::tokens::Token;

pub enum Expression {
  Inner(CalculatableExpression),
  Leaf(PrimitiveType)
}

pub enum CalculatableExpression {
  Binary(BinaryExpression),
  Unary(UnaryExpression),
}

pub struct BinaryExpression {
  pub operation: Token,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
}

pub struct UnaryExpression {
  pub operation: Token,
  pub argument: Box<Expression>,
}