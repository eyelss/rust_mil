use crate::source::data_source::DataSource;
use crate::tokenizer::tokenizer::{Token, Tokenizer};

pub struct TemplateParser;

pub struct ParseTree;

pub struct ASTree {

}

pub enum ASTNode {
  Root,
  Expr(Expr),
  Stmt,
}

pub enum Expr {
  Binary,
  Unary,

  Logic,
  Compare,
}

pub struct Condition {

}

impl TemplateParser {
  fn parse<T: DataSource>(
    template: String,
    source: T
  ) -> ParseTree {
    let tokenizer = Tokenizer::new(template);
    
    let tokens: Vec<Token> = tokenizer.collect();
    // source.get("da")
    ParseTree
  }
}