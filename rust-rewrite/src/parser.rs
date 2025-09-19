
use crate::lexer;
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



pub trait Nodeable {
}
type Node = Box<dyn Nodeable>;

struct StatSeq          { nodes: Vec<Node> } //program is just sequence of statements
struct ImportStat       { }
struct VariableAssign   { }
struct BinaryExpr       { op: String, left: Node, right: Node }
struct UnaryExpr        { op: String, operand: Node }
struct IntLiteral       { value: u64 }
struct FloatLiteral     { value: f64 }
struct StrLiteral       { value: String }
struct Variable         { name:  String } 
struct FunctionCall     { }
struct ModAccess        { }
struct ArrayLiteral     { }
struct ReturnStat       { }
struct FunctionDeclare  { }
struct ExprStat         { }
struct IfStat           { condition: Node, if_block: StatSeq, else_block: Option<Node> }
struct WhileStat        { condition: Node, body: StatSeq }
struct ForStat          { elem_name: String, array: Node, body: StatSeq }


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
        let condition: Node  = parse_condition(stream);
        let if_block: StatSeq = parse_block(stream);

        let else_block = match stream.peek().map(|x| x.data.clone()) {
            Some(lexer::TokenClass::Keyword(x)) if ["else", "elif"].contains(&x.as_str()) => {
                stream.next();
                Some(match x.as_str() {
                    "elif" => Box::new(IfStat::parse(stream)) as Node,
                    "else" => Box::new(parse_block(stream))   as Node,
                    _ => unreachable!(),
                })
            },
            _ => None,
        };

        IfStat { condition, if_block, else_block }
    }
}

impl Nodeable for IfStat {}
impl Nodeable for UnaryExpr {}
impl Nodeable for BinaryExpr {}


fn parse_primary_expr(stream: Streaming) -> Node {
    let Some(token) = stream.peek() else { stream.error("End of token stream while parsing primary expression."); }
    
    match token.data {
        lexer::TokenClass::BracketOpen  => ArrayLiteral::parse(stream),
        lexer::TokenClass::Operator(op) if UNARY_OPS.contains(&x.as_str()) => {
            stream.next();
            let operand = parse_expr_prec(stream, 9);
            Box::new(UnaryExpr { operand, op }) as Node
        },
        lexer::TokenClass::Integer(x) => IntLiteral   { value: x },
        lexer::TokenClass::Float(x)   => FloatLiteral { value: x },
        lexer::TokenClass::String(x)  => StrLiteral   { value: x.clone() },
        lexer::TokenClass::Identifier(_) if lookhead_fn_call(stream) => FunctionCall::parse(stream),
        lexer::TokenClass::Identifier(_) if lookhead_mod(stream)     => ModAccess::parse(stream),
        lexer::TokenClass::Identifier(x) => Variable  { name: x.clone() },
        lexer::TokenClass::ParenOpen      => {
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
    let Some(token) = stream.peek() else { stream.error("End of token stream while parsing precedented expression."); };
 
    if let lexer::TokenClass::Operator(op) = token.data {
        while precedence <= get_op_precedence(op.as_str()) {
            let right = parse_expr_prec(stream, get_op_precedence(op.as_str()));

            left = Box::new(BinaryExpr { 
                op, left, right
            }) as Node;
        }
    }

    left
}


fn parse_func_args(stream: Streaming) -> Vec<String> {
    let out: Vec<String> = vec![];
    stream.expect(lexer::TokenClass::ParenOpen);

    while let Some(x) = stream.peek() {
        match x.data {
            lexer::TokenClass::Identifier(x) => {
                stream.next();
                stream.maybe(",");
                out.push(x);
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
        let condition = parse_condition(stream);
        let body = parse_block(stream);
        WhileStat { condition, body }
    }
}

impl ForStat {
    fn parse(stream: Streaming) -> Self {
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




fn lookhead_assign(stream: Streaming) -> bool {

}


fn parse_statement(stream: Streaming) -> Option<Node> {
    let Some(token) = stream.peek() else { return None };

    //keyword have to be pre-advanced
    if let lexer::TokenClass::Keyword(_) = token.data {
        stream.next();
    }
    
    Some(match token.data {
        lexer::TokenClass::Keyword(x) if x == "if"      => Box::new(IfStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(x) if x == "while"   => Box::new(WhileStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(x) if x == "for"     => Box::new(ForStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(x) if x == "fun"     => Box::new(FunctionDeclare::parse(stream)) as Node,
        lexer::TokenClass::Keyword(x) if x == "return"  => Box::new(ReturnStat::parse(stream)) as Node,
        lexer::TokenClass::Keyword(x) if x == "import"  => Box::new(ImportStat::parse(stream)) as Node,
        lexer::TokenClass::Identifier(x) if lookhead_assign(stream) => Box::new(VariableAssign::parse(stream)) as Node,
        lexer::TokenClass::ParenOpen | 
            lexer::TokenClass::Integer(_) | lexer::TokenClass::Float(_) | lexer::TokenClass::Integer(_) |
            lexer::TokenClass::String(_) | lexer::TokenClass::Identifier(_) | lexer::TokenClass::Operator(_)
            => Box::new(ExprStat::parse(stream)) as Node,
    
        _ => stream.error("Invalid syntax")
    })

}








impl StatSeq {
    fn parse(stream: Streaming) -> Self {
        let mut nodes: Vec<Node> = vec![];

        loop {
            let Some(node) = stream.peek() else { break };
            if lexer::TokenClass::CurlyClose == node.data { break };

            let Some(node) = parse_statement(stream) else { break };
            nodes.push(node);
        }
        StatSeq { nodes }
    }
}







