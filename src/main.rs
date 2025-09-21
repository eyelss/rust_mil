use std::iter;

use crate::tokenizer::tokenizer::{Token, Tokenizer};

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
    {{if !user.departed and user.name != \"unclose\" skip}}
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