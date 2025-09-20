use std::fs;


mod lexer;
mod parser;

fn main() {

    let source = fs::read_to_string("prg/processing_test.oko").unwrap();
    let mut stream = lexer::lex(&source);
    let root = parser::StatSeq::parse(&mut stream);

    dbg!(root);

}
