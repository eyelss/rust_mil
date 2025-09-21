use crate::shared::data_types::SupportedDataType;
use crate::tokenizer::tokens::{Keyword, Token};

pub enum ASTNode {
  Prgm(Program),
  Expr(Expression),
  Stmt(Statement),
  Blck(Block)
}

pub enum Expression {
  Inner(CalculatableExpression),
  Leaf(SupportedDataType)
}

pub enum CalculatableExpression {
  Binary(BinaryExpression),
  Unary(UnaryExpression),
}

pub struct BinaryExpression {
  pub operation: Keyword,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
}

pub struct UnaryExpression {
  pub operation: Keyword,
  pub argument: Box<Expression>,
}

pub enum Statement {
  If(BranchStatemenet),
  Each(CycleStatement),
}

pub enum Block {
  Statement(BlockStatement)
} 

pub struct BranchStatemenet {
  pub condition: Expression,
  pub consequent: Block,
  pub alternative: Option<Block>,
}

pub struct CycleStatement {
  pub variable: Token, // Identifier: array variable
  pub iterable: Expression,
  pub index: Option<Token>, // Identifier: array[idx] variable
  pub body: Block,
}

pub struct BlockStatement {
  pub statements: Vec<Statement>
}

pub struct Program {
  pub statements: Vec<Statement>
}