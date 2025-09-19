
use crate::lexer;
type Streaming = &mut lexer::Stream;

pub trait Nodeable {
    //size known at compile-time because constructor
    fn parse(stream: Streaming) -> Self where Self: Sized; 
}
type Node = Box<dyn Nodeable>;

struct StatSeq          { nodes: Vec<Node> } //program is just sequence of statements
struct ImportStat       { }
struct VariableAssign   { }
struct BinaryExpr       { }
struct UnaryExpr        { }
struct NumLiteral       { }
struct StrLiteral       { }
struct Identifier       { } //maybe variable
struct FunctionCall     { }
struct ModAccess        { }
struct ArrayLiteral     { }
struct ReturnStat       { }
struct FunctionDeclare  { }
struct ExprStat         { }
struct IfStat           { condition: Node, if_block: StatSeq, else_block: Option<Node> }
struct WhileStat        { }
struct ForStat          { }


fn parse_condition(stream: Streaming) -> Node {
    stream.expect(lexer::TokenClass::ParenOpen);
    let expr = parse_expr(stream);
    stream.expect(lexer::TokenClass::ParenClose);
    return expr;
}

fn parse_block(stream: Streaming) -> StatSeq {
    stream.expect(lexer::TokenClass::CurlyOpen);
    let block = StatSeq::parse(stream);
    stream.expect(lexer::TokenClass::CurlyClose);
    return block;
}



impl Nodeable for IfStat {
    fn parse(stream: Streaming) -> Self {
        let condition: Node  = parse_condition(stream);
        let if_block: StatSeq = parse_block(stream);

        let else_block = match stream.peek().map(|x| x.data) {
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






fn lookhead_assign(stream: Streaming) -> bool {

}


fn parse_statement(stream: Streaming) -> Option<Node> {
    let Some(token) = stream.peek() else { return None };

    //keyword have to be pre-advanced
    if let lexer::TokenClass::Keyword(_) = token.data {
        stream.next();
    }
    
    match token.data {
        lexer::TokenClass::Keyword(x) if x == "if"      => IfStat::parse(stream),
        lexer::TokenClass::Keyword(x) if x == "while"   => WhileStat::parse(stream),
        lexer::TokenClass::Keyword(x) if x == "for"     => ForStat::parse(stream),
        lexer::TokenClass::Keyword(x) if ["elif", "else"].contains(&&x.as_str()) 
            => Stream.error("Either \"elif\" or \"else\" was used without it being preceeded by an if statement."),
        lexer::TokenClass::Keyword(x) if x == "fun"     => FunctionDeclare::parse(stream),
        lexer::TokenClass::Keyword(x) if x == "return"  => ReturnStat::parse(stream),
        lexer::TokenClass::Keyword(x) if x == "import"  => ImportStat::parse(stream),
        lexer::TokenClass::Identifier(x) if lookhead_assign(stream) => VariableAssign::parse(stream),
        lexer::TokenClass::ParenOpen | 
            lexer::TokenClass::Integer(_) | lexer::TokenClass::Float(_) | lexer::TokenClass::Integer(_) |
            lexer::TokenClass::String(_) | lexer::TokenClass::Identifier(_) | lexer::TokenClass::Operator(_)
            => ExprStat::parse(stream),
    }

}




impl Nodeable for StatSeq {
    fn parse(stream: Streaming) -> Self {
        let mut nodes: Vec<Node> = vec![];

        while let Some(node) = parse_statement(stream) {
            nodes.push(node);
        }
        StatSeq { nodes }
    }
}







