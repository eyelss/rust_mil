use super::block::Block;
use super::expression::Expression;
use super::program::Program;
use super::statement::Statement;

pub enum ASTree {
    Root(ASTNode)
}

pub enum ASTNode {
    Prgm(Program),
    Expr(Expression),
    Grup(Expression),
    Stmt(Statement),
    Blck(Block)
}
