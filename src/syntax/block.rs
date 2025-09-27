use super::statement::Statement;

pub enum Block {
  Statement(BlockStatement)
} 

pub struct BlockStatement {
  pub statements: Vec<Statement>
}