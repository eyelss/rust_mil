use crate::source::data_source::DataSource;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::tokens::{Token};
use crate::parser::lexer::*;

pub struct TemplateParser;


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