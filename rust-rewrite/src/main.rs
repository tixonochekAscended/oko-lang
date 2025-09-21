use std::{collections::HashMap, fs};

use parser::Nodeable;


mod lexer;
mod parser;
mod executor;

fn main() {

    let source = fs::read_to_string("prg/fib.oko").unwrap();
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
