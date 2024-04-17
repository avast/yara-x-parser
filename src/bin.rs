use std::fs;
use yara_parser::{AstNode, SourceFile};

fn main() {
    let filename = std::env::args().nth(1).expect("No arguments provided");
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong while reading the file");
    let ast = SourceFile::parse(&file_content);
    println!("AST:\n\n{:#?}", ast.tree().syntax());
    println!("Errors: {:?}", ast.errors());
}
