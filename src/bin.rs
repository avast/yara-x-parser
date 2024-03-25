use std::fs;
use yara_parser::SourceFile;

fn main() {
    let filename = "example.yar";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let ast2 = SourceFile::parse(&contents);
    println!("{:#?}", ast2.tree());
    println!("{:?}", ast2.errors());
}
