
use core::fmt;
use std::rc::Rc;

use crate::lexer::{self, Stream};
use crate::executor;
type Streaming<'a> = &'a mut lexer::Stream;

const UNARY_OPS: [&str; 2] = ["!", "-"];

fn get_op_precedence(op: &str) -> u32 {
    match op {
    	"||" => 1,
    	"&&" => 2,

    	"!=" => 3,
    	"==" => 3,
    	">" =>  3,
    	"<" =>  3,
    	">=" => 3,
    	"<=" => 3,

    	"+" => 5,
    	"-" => 5,
    	"*" => 6,
    	"/" => 6,
    	"%" => 6,

    	"^" => 7,

        _ => 0,
    }
}




pub trait Nodeable: fmt::Debug {
    fn eval(&self, scope: &mut executor::Scope) -> executor::Obj;
}

pub type Node = Box<dyn Nodeable>;



#[derive(Debug)] pub struct StatSeq          { pub nodes: Vec<Node> } //program is just sequence of statements
#[derive(Debug)] pub struct ImportStat       { pub mod_name: String }
#[derive(Debug)] pub struct VariableAssign   { pub var_name: String, pub op: lexer::TokenClass, pub expr: Node }
#[derive(Debug)] pub struct BinaryExpr       { pub op: String, pub left: Node, pub right: Node }
#[derive(Debug)] pub struct UnaryExpr        { pub op: String, pub operand: Node }
#[derive(Debug)] pub struct IntLiteral       { pub value: u64 }
#[derive(Debug)] pub struct FloatLiteral     { pub value: f64 }
#[derive(Debug)] pub struct StrLiteral       { pub value: String }
#[derive(Debug)] pub struct Variable         { pub name:  String } 
#[derive(Debug)] pub struct FunctionCall     { pub name: String, pub args: Vec<Node> }
#[derive(Debug)] pub struct ModAccess        { pub mod_name: String, pub member: FunctionCall }
#[derive(Debug)] pub struct ArrayLiteral     { pub elem: Vec<Node> }
#[derive(Debug)] pub struct ReturnStat       { pub expr: Option<Node> }
#[derive(Debug)] pub struct FunctionDeclare  { pub name: String, pub args: Vec<String>, pub body: Rc<StatSeq> }
#[derive(Debug)] pub struct ExprStat         { pub expr: Node }
#[derive(Debug)] pub struct IfStat           { pub condition: Node, pub if_block: StatSeq, pub else_block: Option<Node> }
#[derive(Debug)] pub struct WhileStat        { pub condition: Node, pub body: StatSeq }
#[derive(Debug)] pub struct ForStat          { pub elem_name: String, pub array: Node, pub body: StatSeq }


fn parse_block(stream: Streaming) -> StatSeq {
    stream.expect(lexer::TokenClass::CurlyOpen);
    let block = StatSeq::parse(stream);
    stream.expect(lexer::TokenClass::CurlyClose);
    return block;
}

fn parse_condition(stream: Streaming) -> Node {
    stream.expect(lexer::TokenClass::ParenOpen);
    let expr = parse_expr(stream);
    stream.expect(lexer::TokenClass::ParenClose);
    return expr;
}



impl IfStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("if".to_string()));
        let condition: Node  = parse_condition(stream);
        let if_block: StatSeq = parse_block(stream);

        let else_block = match stream.peek() {
            None => None,
            Some(ref x) => {
                match x.data {
                    lexer::TokenClass::Keyword(ref x) if x == "elif" => 
                        { stream.next(); Some(Box::new(IfStat::parse(stream)) as Node) },
                    lexer::TokenClass::Keyword(ref x) if x == "else" => 
                        { stream.next(); Some(Box::new(parse_block(stream)) as Node)   },
                    _ => None,
            }},
        };

        IfStat { condition, if_block, else_block }
    }
}



fn lookhead_fn_call(stream: &Stream) -> bool {
    match stream.lookhead(1) {
        Some(lexer::TokenClass::ParenOpen) => true,
        _ => false,
    }
}

fn lookhead_mod(stream: &Stream) -> bool {
    match stream.lookhead(1) {
        Some(lexer::TokenClass::Namespace) => true,
        _ => false,
    }
}




fn parse_primary_expr(stream: Streaming) -> Node {
    let Some(token) = stream.peek() else { stream.error("End of token stream while parsing primary expression."); };
    
    match token.data.clone() {
        lexer::TokenClass::Operator(ref op) if UNARY_OPS.contains(&op.as_str()) => {
            stream.next();
            let operand = parse_expr_prec(stream, 9);
            Box::new(UnaryExpr { operand, op: op.clone() }) as Node
        },
        lexer::TokenClass::BracketOpen                               => {                Box::new(ArrayLiteral::parse(stream))       as Node },
        lexer::TokenClass::Integer(x)                                => { stream.next(); Box::new(IntLiteral   { value: x         }) as Node },
        lexer::TokenClass::Float(x)                                  => { stream.next(); Box::new(FloatLiteral { value: x         }) as Node },
        lexer::TokenClass::String(ref x)                             => { stream.next(); Box::new(StrLiteral   { value: x.clone() }) as Node },
        lexer::TokenClass::Identifier(_) if lookhead_fn_call(stream) => {                Box::new(FunctionCall::parse(stream))       as Node },
        lexer::TokenClass::Identifier(_) if lookhead_mod(stream)     => {                Box::new(ModAccess::parse(stream))          as Node },
        lexer::TokenClass::Identifier(ref x)                         => { stream.next(); Box::new(Variable     { name: x.clone()  }) as Node },
        lexer::TokenClass::ParenOpen     => {
            stream.next();
            let expr = parse_expr(stream);
            stream.expect(lexer::TokenClass::ParenClose);
            expr
        },
        _ => stream.error("Invalid Syntax while parsing primary expression.")
    }
    
}

fn parse_expr(stream: Streaming) -> Node {
    parse_expr_prec(stream, 0)
}


fn parse_expr_prec(stream: Streaming, precedence: u32) -> Node {
    let mut left = parse_primary_expr(stream);
 
    loop {
        let Some(token) = stream.peek() else { break; };
        let lexer::TokenClass::Operator(ref op_ref) = token.data else { break; };
        if precedence > get_op_precedence(op_ref.as_str()) { break; }
        let op = op_ref.clone();
        stream.next();

        let right = parse_expr_prec(stream, get_op_precedence(op.as_str()));

        left = Box::new(BinaryExpr { 
            op, left, right
        }) as Node;
    }

    left
}


fn parse_func_args(stream: Streaming) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    stream.expect(lexer::TokenClass::ParenOpen);

    while let Some(x) = stream.peek() {
        match x.data {
            lexer::TokenClass::Identifier(ref x) => {
                let arg = x.clone();
                stream.next();
                stream.maybe(lexer::TokenClass::Comma);
                out.push(arg);
            },
            lexer::TokenClass::ParenClose => { break; }
            _ => stream.error("Expected identifier or closing parenthesis")
        }
    }

    stream.expect(lexer::TokenClass::ParenClose);
    out
}

impl WhileStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("while".to_string()));
        let condition = parse_condition(stream);
        let body = parse_block(stream);
        WhileStat { condition, body }
    }
}

impl ForStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("for".to_string()));
        stream.expect(lexer::TokenClass::ParenOpen);
        let Some(token) = stream.pop() else { stream.error("End of token stream while parsing for loop."); };
        let lexer::TokenClass::Identifier(ref elem_name_ref) = token.data else { stream.error("Expected identifier for element name"); };
        let elem_name = elem_name_ref.clone();
        stream.expect(lexer::TokenClass::ParenClose);

        stream.expect(lexer::TokenClass::ParenOpen);
        let array = parse_expr(stream);
        stream.expect(lexer::TokenClass::ParenClose);

        let body = parse_block(stream);

        ForStat { elem_name, array, body }
    }
}

impl ReturnStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("return".to_string()));
        let Some(token) = stream.peek() else { stream.error("End of token stream while parsing return statement."); };

        let expr: Option<Node> = match token.data {
            lexer::TokenClass::EndOfStatement => None,
            _ => Some(parse_expr(stream)),
        };

        stream.expect(lexer::TokenClass::EndOfStatement);

        ReturnStat { expr }
    }
}

impl FunctionDeclare {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("fun".to_string()));
        let Some(token) = stream.pop() else { stream.error("End of token stream while parsing function delcaration."); };
        let lexer::TokenClass::Identifier(ref name_ref) = token.data else { stream.error("Expected identifier."); };
        let name = name_ref.clone();
        
        let args = parse_func_args(stream);
        let body = parse_block(stream);

        FunctionDeclare { name, args, body: Rc::new(body) }
    }
}


fn parse_call_args(stream: Streaming) -> Vec<Node> {
    let mut out: Vec<Node> = vec![];
    stream.expect(lexer::TokenClass::ParenOpen);

    while let Some(x) = stream.peek() {
        if lexer::TokenClass::ParenClose == x.data { break; };
        out.push(parse_expr(stream));
        stream.maybe(lexer::TokenClass::Comma);
    }

    stream.expect(lexer::TokenClass::ParenClose);
    out
}

impl FunctionCall {
    fn parse(stream: Streaming) -> Self {
        let Some(token) = stream.pop() else { stream.error("End of token stream while parsing function call."); };
        let lexer::TokenClass::Identifier(ref name_ref) = token.data else { stream.error("Expected identifier for function name."); };

        let name = name_ref.clone();
        let args = parse_call_args(stream);

        FunctionCall { name, args }
    }
}

impl ModAccess {
    fn parse(stream: Streaming) -> Self {
        let Some(mod_token) = stream.pop() else { stream.error("End of token stream while parsing module access."); };
        let lexer::TokenClass::Identifier(ref mod_ref) = mod_token.data else { stream.error("Expected module identifier."); };
        let mod_name = mod_ref.clone();

        stream.maybe(lexer::TokenClass::Namespace);

        let member = FunctionCall::parse(stream);

        ModAccess { mod_name, member }

    }
}

impl ImportStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe(lexer::TokenClass::Keyword("import".to_string()));
        let Some(mod_token) = stream.pop() else { stream.error("End of token stream while parsing module import."); };
        let lexer::TokenClass::Identifier(ref mod_ref) = mod_token.data else { stream.error("Expected import identifier."); };
        let mod_name = mod_ref.clone();

        stream.expect(lexer::TokenClass::EndOfStatement);

        ImportStat { mod_name }

    }
    
}

impl VariableAssign {
    fn parse(stream: Streaming) -> Self {
        let Some(var_token) = stream.pop() else { stream.error("End of token stream while parsing variable assignment."); };
        let lexer::TokenClass::Identifier(ref var_ref) = var_token.data else { stream.error("Expected variable name."); };
        let var_name = var_ref.clone();

        let Some(op_token) = stream.pop() else { stream.error("End of token stream while parsing variable assignment."); };
        let op_data = op_token.data.to_owned();
        let op = match op_data {
            lexer::TokenClass::AssignOp(_) => op_data,
            lexer::TokenClass::Assign => op_data,
            lexer::TokenClass::Define => op_data,
            _ => stream.error("Expected assignment operator."),
        };

        let expr = parse_expr(stream);

        stream.expect(lexer::TokenClass::EndOfStatement);

        VariableAssign { var_name, op, expr }
    }
}

impl ArrayLiteral {
    fn parse(stream: Streaming) -> Self {
        stream.expect(lexer::TokenClass::BracketOpen);
        
        let mut elem: Vec<Node> = vec![];

        while let Some(token) = stream.peek() {
            if let lexer::TokenClass::BracketClose = token.data { break; }
            elem.push(parse_expr(stream));
            stream.maybe(lexer::TokenClass::Comma);
        }

        stream.expect(lexer::TokenClass::BracketClose);
        ArrayLiteral { elem }
    }
}


impl ExprStat {
    fn parse(stream: Streaming) -> Self {
        let expr = parse_expr(stream);
        stream.expect(lexer::TokenClass::EndOfStatement);
        ExprStat { expr }
    }
}


fn lookhead_assign(stream: &lexer::Stream) -> bool {
    match stream.lookhead(1) {
        Some(lexer::TokenClass::AssignOp(_)) => true,
        Some(lexer::TokenClass::Assign)      => true,
        Some(lexer::TokenClass::Define)      => true,
        _ => false
    }
}


fn parse_statement(stream: Streaming) -> Option<Node> {
    let Some(ref token) = stream.peek() else { return None };

    Some(match token.data {
        lexer::TokenClass::Keyword(ref x) if x == "if"      => Box::new(IfStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(ref x) if x == "while"   => Box::new(WhileStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(ref x) if x == "for"     => Box::new(ForStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(ref x) if x == "fun"     => Box::new(FunctionDeclare::parse(stream)) as Node,
        lexer::TokenClass::Keyword(ref x) if x == "return"  => Box::new(ReturnStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(ref x) if x == "import"  => Box::new(ImportStat::parse(stream)) as Node,
        lexer::TokenClass::Identifier(_) if lookhead_assign(stream) => Box::new(VariableAssign::parse(stream)) as Node,
        lexer::TokenClass::ParenOpen | 
            lexer::TokenClass::Integer(_) | lexer::TokenClass::Float(_) | 
            lexer::TokenClass::String(_) | lexer::TokenClass::Identifier(_) | lexer::TokenClass::Operator(_)
            => Box::new(ExprStat::parse(stream)) as Node,
    
        _ => stream.error("Invalid syntax")
    })

}








impl StatSeq {
    pub fn parse(stream: Streaming) -> Self {
        let mut nodes: Vec<Node> = vec![];

        loop {
            let Some(node) = stream.peek() else { break };
            if let lexer::TokenClass::CurlyClose = node.data { break };

            let Some(node) = parse_statement(stream) else { break };
            nodes.push(node);
        }
        StatSeq { nodes }
    }
}







