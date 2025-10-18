use std::io;

use crate::lexer::tokenizer::Tokenizer;
use crate::lexer::tokens::Token;

mod config;
mod source;
mod errors;
mod syntax;
mod shared;
mod lexer;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokenizer = Tokenizer::new(input);

    for (idx, token) in tokenizer.enumerate() {
        println!("{}.\t{:?}", idx, token);
    }
}