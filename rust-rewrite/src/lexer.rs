


const KEYWORDS: [&str; 6] = [
    "fun",
    "while",
    "if",
    "else",
    "import",
    "for",
];

const OPERATORS: [&str; 14] = [
    "+",
    "-",
    "*",
    "/",
    "%",
    "^",
    "!",
    ">",
    "<",
    ">=",
    "<=",
    "&&",
    "||",
    "==",
];

const ESCAPE_SEQUENCES: [[&str; 2]; 4] = [
    ["\\n", "\n"],
    ["\\e", "\x1B"],
    ["\\t", "\t"],
    ["\\\"", "\""],
];


fn do_escape_sequences(seq: &String)
{
    for map in ESCAPE_SEQUENCES.iter() {
        seq.replace(map[0], map[1]);
    }
}


#[derive(PartialEq, Debug)]
pub enum TokenClass {
    Operator(String),
    String(String),
    Number(i32),
    Identifier(String),
    Keyword(String),
    EndOfStatement, // ;
    Comma,
    Define, // :=
    Assign, // =
    AssignOp, // +=, -=, etc. 
    ParenOpen, ParenClose,
    CurlyOpen, CurlyClose,
    BracketOpen, BracketClose,
}


#[derive(Debug)]
pub struct Token {
    data: TokenClass,
    line_index: u32,
}
type Stream = Vec<Token>;



//this defines the state for the tokenizer state machine,
//which detects the boundaries between tokens
#[derive(PartialEq)]
enum CharType {
    Invalid,
    Alpha,
    Num,
    Quote,
    ParenOpen, ParenClose,      //()
    CurlyOpen, CurlyClose,      //{}
    BracketOpen, BracketClose,  //[]
    Symbol,
}


fn get_char_state(char: char) -> CharType {
    return match char {
        x if x.is_alphabetic() => CharType::Alpha,
        x if x.is_numeric() => CharType::Num,
        '-' => CharType::Num,
        '\'' => CharType::Quote,
        '(' => CharType::ParenOpen,   ')' => CharType::ParenClose,
        '{' => CharType::CurlyOpen,   '}' => CharType::CurlyClose,
        '[' => CharType::BracketOpen, ']' => CharType::BracketOpen,
        _ => CharType::Symbol
    }
}





fn push_token(out: &Stream, state: CharType, buffer: &String) {
    let buf_ref: &str = buffer.as_str();

    let data: TokenClass =  match state {
        CharType::Invalid   => { return; },
        CharType::Alpha     => {
            let content: String = buffer.clone();
            match buf_ref {
                x if KEYWORDS.contains(&x) => TokenClass::Keyword(content),
                _                          => TokenClass::Identifier(content),
            }
        },
        CharType::Num => match buf_ref {
            "-" => TokenClass::Operator(buffer.clone()), //dash might be operators or part of number
            _   => TokenClass::Number(buffer.parse::<i32>().unwrap()),
        },
        CharType::Quote     => TokenClass::String(buffer.trim_matches('"').to_string()),
        CharType::ParenOpen   => TokenClass::ParenOpen,   CharType::ParenClose   => TokenClass::ParenClose,
        CharType::CurlyOpen   => TokenClass::CurlyOpen,   CharType::CurlyClose   => TokenClass::CurlyClose,
        CharType::BracketOpen => TokenClass::BracketOpen, CharType::BracketClose => TokenClass::BracketClose,
        CharType::Symbol    => match buf_ref {
            "=" => TokenClass::Assign, ":=" => TokenClass::Define, 
            "+=" | "-=" | "*=" | "/=" => TokenClass::AssignOp,
            x if OPERATORS.contains(&x) => TokenClass::Operator(buffer.clone()),
            x => panic!("Symbol '{}' cannot be categorized", x),
        },
    };

    

}

pub fn lex(source: &String) -> Stream {
    let mut out: Stream  = vec![];

    let mut buffer: String = Default::default();
    let mut state = CharType::Invalid;
    let mut last  = CharType::Invalid;

    let mut line_index: u32 = 0;
    let mut line_text: String;


    for char in source.chars() {
        state = get_char_state(char);

        if state != last {
            //push_token();
            
            buffer.clear();
        }

        buffer.push(char);
        last = state;
    }

    return out;
}










