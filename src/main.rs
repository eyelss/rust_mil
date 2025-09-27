use std::iter;

use crate::lexer::tokenizer::Tokenizer;
use crate::lexer::tokens::{Token, SingleChar};
use crate::syntax::expression::{*};
use crate::syntax::ast::{*};
use crate::shared::data_types::PrimitiveType;
use crate::syntax::visitor::Visitable;

mod config;
mod source;
mod errors;
mod syntax;
mod shared;
mod lexer;

fn main() {
    let line = 
"
This is very important document!
{{each users as user}}  
    {{if !user.departed & user.role != \"admin\" 100a1a1skip}}
        {{user.name}} is available!
    {{else}}
        {{user.name}} is departed!
    {{endif}}
{{endeach}}
";

    // let un_expr = Expression::Inner(CalculatableExpression::Unary(UnaryExpression {
    //     operation: Token::Single(lexer::tokens::SingleChar::Minus),
    //     argument: Box::new(Expression::Leaf(PrimitiveType::Integer(10))),
    // }));

    // println!("{}", un_expr.accept(&syntax::visitor::Visitor {}));

    // let bin_expr = Expression::Inner(CalculatableExpression::Binary(BinaryExpression {
    //     operation: Token::Single(SingleChar::Plus),
    //     left: Box::new(un_expr),
    //     right: Box::new(Expression::Inner(CalculatableExpression::Binary(BinaryExpression {
    //         operation: Token::Single(SingleChar::Plus),
    //         left: Box::new(Expression::Leaf(PrimitiveType::Integer(10))),
    //         right: Box::new(Expression::Leaf(PrimitiveType::Integer(15))),
    //     }))),
    // }));

    // println!("{}", bin_expr.accept(&syntax::visitor::Visitor {}));

    let tokenizer = Tokenizer::new(String::from(line));

    for (idx, token) in tokenizer.enumerate() {
        println!("{}.\t{:?}", idx, token);
    }
}