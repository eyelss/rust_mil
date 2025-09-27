
use crate::lexer::tokens::{Keyword, Token};
use crate::syntax::statement::Statement;

pub struct Program {
  pub statements: Vec<Statement>
}