use super::expression::Expression;
use super::program::Program;
use super::statement::Statement;
use super::block::Block;

pub enum ASTNode {
  Prgm(Program),
  Expr(Expression),
  Grup(Expression),
  Stmt(Statement),
  Blck(Block)
}