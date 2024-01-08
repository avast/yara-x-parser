/// This library is used to create a parser for YARA language
/// It should provide also token for whitespaces
/// as we want full fidelity and error resilience.;
use std::{env::args, fs, path::Path};

use rowan_test::GreenNode;

use crate::lexer::tokenize;
use crate::parser::{TokenSource, TreeSink};
use crate::syntax::{
    syntax_error::SyntaxError, text_token_source::TextTokenSource, text_tree_sink::TextTreeSink,
};

mod lexer;
mod parser;
mod syntax;

fn main() {
    // Take file as an input and parse it into tokens
    let arg = args().nth(1).expect("No pathname given");
    let path = Path::new(&arg);
    let input = fs::read_to_string(path).unwrap();

    parse_text(&input);
}

fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, lexer_errors) = tokenize(text);
    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);
    println!("{:?}", tokens);

    parser::parse(&mut token_source, &mut tree_sink);
    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(lexer_errors);

    (tree, parser_errors)
}
