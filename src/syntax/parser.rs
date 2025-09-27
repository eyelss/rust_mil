use crate::source::data_source::DataSource;
use crate::lexer::tokens::{Token};
use super::ast::ASTNode;
use super::expression::Expression;
use super::visitor::{Visitable, Visitor};

pub struct TemplateParser {
  tokens: Vec<Token>,
  ptr: usize,
}

impl Visitable for Expression {
  fn accept(&self, v: &Visitor) -> String {
    v.visit(self)
  }
}

impl TemplateParser {
  fn parse<T: DataSource>(
    &mut self,
    source: T
  ) -> ASTNode {
    // for token in &self.tokens {
    //   match token {
    //       Token::Identifier(identifier) => {}
    //   }
    // }
    todo!()
  }

  fn new(
    tokens: Vec<Token>
  ) -> TemplateParser {
    TemplateParser { 
      tokens: tokens,
      ptr: 0,
    }
  }
}