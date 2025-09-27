use super::expression::Expression;
use super::block::Block;
use crate::Token;

pub enum Statement {
  If(BranchStatemenet),
  Each(CycleStatement),
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