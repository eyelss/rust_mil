use super::block::Block;
use super::expression::Expression;
use super::program::Program;
use super::statement::Statement;

pub enum ASTree {
    Root(ASTNode)
}

pub enum ASTNode {
    If {
        condition: Expression,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Box<ASTNode>>
    },
    Each {
        iter: Expression,
        body: Vec<ASTNode>
    },
    Prgm(Program),
    Expr(Expression),
    Grup(Expression),
    Stmt(Statement),
    Blck(Block)
}
