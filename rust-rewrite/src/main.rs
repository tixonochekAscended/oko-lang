use std::{collections::HashMap, fs};

use parser::Nodeable;


mod lexer;
mod parser;
mod executor;

fn error(msg: &str) -> ! {
    eprint!("Error: {}\n", msg);
    std::process::exit(1);
}


fn main() {

    let mut args = std::env::args();
    _ = args.next(); //first arg is exec path

    let Some(source_path) = args.next() else {
        error("No source path argument provided.");
    };

    let Ok(source) = fs::read_to_string(source_path) else {
        error("Unable to read source file.");
    };


    let mut stream = lexer::lex(&source);
    let root = parser::StatSeq::parse(&mut stream);
    
    let mut scope = executor::Scope {
        vars: HashMap::new(),
        funs: HashMap::new(),
        ret_val: executor::Obj::Invalid,
        ret_flag: false
    };

    root.eval(&mut scope);

}
