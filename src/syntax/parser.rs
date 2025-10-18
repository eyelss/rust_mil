use crate::lexer::tokenizer::Tokenizer;
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
  fn new(
    tokens: Vec<Token>
  ) -> TemplateParser {
    TemplateParser {
      tokens,
      ptr: 0,
    }
  }

  fn is_at_end(&self) -> bool {
    self.tokens.len() == self.ptr
  }

  fn peek(&self) -> &Token {
    self.tokens.get(self.ptr).unwrap()
  }

  fn prev(&self) -> &Token {
    self.tokens.get(self.ptr - 1).unwrap()
  }

  fn next(&mut self) -> &Token {
    if !self.is_at_end() {
      self.ptr += 1;
    }

    self.prev()
  }

  fn check(&self, compare_token: Token) -> bool {
    if !self.is_at_end() {
      false
    } else {
      matches!(self.peek(), compare_token)
    }
  }

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
}