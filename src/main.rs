#![feature(slice_patterns)]

pub mod lexer;
pub mod token;
pub mod repl;
pub mod ast;
pub mod parser;

fn main() {
    repl::start();
}
