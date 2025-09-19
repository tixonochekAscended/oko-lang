
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
struct ImportStat       { mod_name: String }
struct VariableAssign   { var_name: String, op: String, expr: Node }
struct BinaryExpr       { op: String, left: Node, right: Node }
struct UnaryExpr        { op: String, operand: Node }
struct IntLiteral       { value: u64 }
struct FloatLiteral     { value: f64 }
struct StrLiteral       { value: String }
struct Variable         { name:  String } 
struct FunctionCall     { name: String, args: Vec<Node> }
struct ModAccess        { mod_name: String, member: String }
struct ArrayLiteral     { elem: Vec<Node> }
struct ReturnStat       { expr: Option<Node> }
struct FunctionDeclare  { name: String, args: Vec<String>, body: StatSeq }
struct ExprStat         { expr: Node }
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
        stream.maybe("if");
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
impl Nodeable for WhileStat {}
impl Nodeable for ForStat {}


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
        stream.maybe("while");
        let condition = parse_condition(stream);
        let body = parse_block(stream);
        WhileStat { condition, body }
    }
}

impl ForStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe("for");
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
        stream.maybe("return");
        let Some(token) = stream.pop() else { stream.error("End of token stream while parsing return statement."); };

        let expr: Option<Node> = match token.data {
            lexer::TokenClass::EndOfStatement => Some(parse_expr(stream)),
            _ => None,
        };

        stream.expect(lexer::TokenClass::EndOfStatement);

        ReturnStat { expr }
    }
}

impl FunctionDeclare {
    fn parse(stream: Streaming) -> Self {
        stream.maybe("fun");
        let Some(token) = stream.pop() else { stream.error("End of token stream while parsing function delcaration."); };
        let lexer::TokenClass::Identifier(ref name_ref) = token.data else { stream.error("Expected identifier."); };
        let name = name_ref.clone();
        
        let args = parse_func_args(stream);
        let body = parse_block(stream);

        FunctionDeclare { name, args, body }
    }
}


fn parse_call_args(stream: Streaming) -> Vec<Node> {
    let out: Vec<Node> = vec![];
    stream.expect(lexer::TokenClass::ParenOpen);

    while let Some(x) = stream.peek() {
        out.push(parse_expr(stream));
        stream.maybe(",");
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

        let Some(member_token) = stream.pop() else { stream.error("End of token stream while parsing module access."); };
        let lexer::TokenClass::Identifier(ref member_ref) = member_token.data else { stream.error("Expected member name."); };
        let member = member_ref.clone();

        ModAccess { mod_name, member }

    }
}

impl ImportStat {
    fn parse(stream: Streaming) -> Self {
        stream.maybe("import")
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
        let lexer::TokenClass::Operator(ref op_ref) = op_token.data else { stream.error("Expected assignment operator."); };
        let op = op_ref.clone();

        let expr = parse_expr(stream);

        stream.expect(lexer::TokenClass::EndOfStatement);

        VariableAssign { var_name, op, expr }
    }
}

impl ArrayLiteral {
    fn parse(stream: Streaming) -> Self {
        stream.expect(lexer::TokenClass::BracketOpen);
        
        let elem: Vec<Node> = vec![];

        while let Some(token) = stream.peek() {
            if let lexer::TokenClass::BracketClose = token.data { break; }
            elem.push(parse_expr(stream));
            stream.maybe(",");
        }

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

impl Nodeable for FunctionDeclare {}
impl Nodeable for ReturnStat {}
impl Nodeable for ImportStat {}
impl Nodeable for VariableAssign {}
impl Nodeable for ExprStat {}


fn lookhead_assign(stream: &lexer::Stream) -> bool {

}


fn parse_statement(stream: Streaming) -> Option<Node> {
    let Some(ref token_ref) = stream.peek() else { return None };
    let token = token_ref.clone();


    //keyword have to be pre-advanced
    /*
    if let lexer::TokenClass::Keyword(_) = token.data {
        stream.next();
    }*/
    
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







