/// This library is used to create a parser for YARA language
/// It should provide also token for whitespaces
/// as we want full fidelity and error resilience.;
use std::{env::args, fs, path::Path};

use rowan_test::GreenNode;
use syntax::SourceFile;

use crate::{
    lexer::tokenize,
    parser::SyntaxKind,
    syntax::{
        syntax_error::SyntaxError,
        syntax_node::{SyntaxNode, SyntaxToken},
        text_token_source::TextTokenSource,
        text_tree_sink::TextTreeSink,
    },
};

// use only for tests
#[cfg(test)]
use crate::syntax::syntax_node::SyntaxElement;
#[cfg(test)]
use rowan_test::NodeOrToken;
#[cfg(test)]
use std::io::Write;

mod lexer;
mod parser;
mod syntax;

fn main() {
    // Take file as an input and parse it into tokens
    let arg = args().nth(1).expect("No pathname given");
    let path = Path::new(&arg);
    let input = fs::read_to_string(path).unwrap();

    let parse = SourceFile::parse(input.as_str());

    let file: SourceFile = parse.tree();
    print!("{:#?}", file.syntax);
}

fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, lexer_errors) = tokenize(text);
    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    parser::parse(&mut token_source, &mut tree_sink);
    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(lexer_errors);

    (tree, parser_errors)
}

#[cfg(test)]
fn print(indent: usize, element: SyntaxElement) -> String {
    let mut result = String::new();
    let kind: SyntaxKind = element.kind();
    result.push_str(&format!("{:indent$}", "", indent = indent));
    match element {
        NodeOrToken::Node(node) => {
            result.push_str(&format!("- {:?}\n", kind));
            for child in node.children_with_tokens() {
                result.push_str(&print(indent + 2, child));
            }
        }

        NodeOrToken::Token(token) => {
            result.push_str(&format!("- {:?} {:?}\n", token.text(), kind));
        }
    }
    result
}

#[test]
fn test_parse_text() {
    let mut mint = goldenfile::Mint::new(".");

    for entry in globwalk::glob("tests/*.in").unwrap().flatten() {
        // Path to the .in.zip file.
        let path = entry.into_path();
        let display_path = path.display();

        let input = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read input file {:?}", display_path));

        let (tree, errors) = parse_text(&input);

        let out_path = path.with_extension("").with_extension("out");
        let syntax_tree = SyntaxNode::new_root(tree.clone());

        let output = print(0, syntax_tree.into());

        let mut output_file = mint.new_goldenfile(out_path).unwrap();

        write!(output_file, "{}", output).unwrap();

        // Check errors
        let err_path = path.with_extension("").with_extension("err");
        if err_path.exists() {
            let expected_errors = fs::read_to_string(&err_path)
                .unwrap_or_else(|_| panic!("Failed to read error file {:?}", err_path.display()));
            let actual_errors =
                errors.iter().map(|error| format!("{:?}", error)).collect::<Vec<_>>().join("\n");
            assert_eq!(actual_errors, expected_errors);
        } else {
            assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
        }
    }
}
