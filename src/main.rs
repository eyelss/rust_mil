use std::iter;

use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::tokens::Token;

mod config;
mod source;
mod errors;
mod parser;
mod shared;
mod tokenizer;

fn main() {
    let line = 
"
This is very important document!
{{each users as user}}
    {{if !user.departed & user.name != \"unclose\" skip}}
        {{user.name}} is available!
    {{else}}
        {{user.name}} is departed!
    {{endif}}
{{endeach}}
";
    let tokenizer = Tokenizer::new(String::from(line));

    let tokens: Vec<Token> = tokenizer.collect();
    for token in tokens {
        println!("{:?}", token);
    }
    // print!("{:?}", tokens);
}