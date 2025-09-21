use crate::shared::data_types::PrimitiveType;
use crate::source::data_source::DataSource;
use crate::lexer::tokenizer::Tokenizer;
use crate::lexer::tokens::{Token};
use crate::syntax::types::{ASTNode, CalculatableExpression, Expression};
use crate::syntax::visitor::{Visitable, Visitor};

pub struct TemplateParser;

impl Visitable for Expression {
  fn accept(&self, v: &Visitor) -> String {
    v.visit(self)
  }
}

impl TemplateParser {
  fn parse<T: DataSource>(
    template: String,
    source: T
  ) -> ASTNode {
    // let tokenizer = Tokenizer::new(template);
    // let tokens: Vec<Token> = tokenizer.collect();
    // source.get("da")
    todo!()
  }
}