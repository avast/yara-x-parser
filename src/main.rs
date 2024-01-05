use lexer::LogosToken;
use logos::Logos;
/// This library is used to create a parser for YARA language
/// It should provide also token for whitespaces
/// as we want full fidelity and error resilience.;
use std::{env::args, fs, path::Path};

use crate::lexer::tokenize;

mod lexer;
mod parser;
mod syntax;

fn main() {
    // Take file as an input and parse it into tokens
    let arg = args().nth(1).expect("No pathname given");
    let path = Path::new(&arg);
    let input = fs::read_to_string(path).unwrap();

    let tokens = tokenize(&input);
    println!("{:?}", tokens);
}
