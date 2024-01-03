/// This library is used to create a parser for YARA language
/// It should provide also token for whitespaces
/// as we want full fidelity and error resilience.;
use std::{env::args, fs, path::Path};
use lexer::Token;
use logos::Logos; 

mod lexer;

fn main() {
    // Take file as an input and parse it into tokens
    let arg = args().nth(1).expect("No pathname given");
    let path = Path::new(&arg);
    let input = fs::read_to_string(path).unwrap();

    // print all tokens
    for result in Token::lexer(&input) {
        match result {
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{:?}", err),
        }
    }
}
