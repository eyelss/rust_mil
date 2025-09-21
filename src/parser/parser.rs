use crate::source::data_source::DataSource;
use crate::tokenizer::tokens::{Token};
use crate::tokenizer::tokenizer::Tokenizer;

pub struct TemplateParser;

pub struct ParseTree;

pub struct ASTree {

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