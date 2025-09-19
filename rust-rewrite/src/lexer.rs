


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


fn do_escape_sequences(mut seq: String) -> String
{
    for map in ESCAPE_SEQUENCES.iter() {
        seq = seq.replace(map[0], map[1]);
    }

    return seq;
}


#[derive(PartialEq, Debug)]
pub enum TokenClass {
    Operator(String),
    String(String),
    Integer(i64),
    Float(f64),
    Identifier(String),
    Keyword(String),
    EndOfStatement, // ;
    Comma,
    Define, // :=
    Assign, // =
    AssignOp(String), // +=, -=, etc. 
    ParenOpen, ParenClose,
    CurlyOpen, CurlyClose,
    BracketOpen, BracketClose,
    Namespace, // ::
}


#[derive(Debug)]
pub struct Token {
    pub data: TokenClass,
    pub line_index: u32,
}

#[derive(Debug)]
pub struct Stream {
    tokens: Vec<Token>,
    index: usize,
}

impl Stream {
    fn push (&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn has(&self) -> bool {
        return self.index < self.tokens.len();
    }

    pub fn peek(&self) -> Option<&Token> {
        if !self.has() { return None }
        
        return Some(&self.tokens[self.index]);
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<&Token> {
        if !self.has() { return None }

        let i = self.index;
        self.next();
        return Some(&self.tokens[i]);
    }

}




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
    Format, //newlines, spaces, etc.
}


fn get_char_state(char: char) -> CharType {
    return match char {
        x if x.is_alphabetic() => CharType::Alpha,
        x if x.is_numeric() => CharType::Num,
        '.' => CharType::Num,
        '"' => CharType::Quote,
        '(' => CharType::ParenOpen,   ')' => CharType::ParenClose,
        '{' => CharType::CurlyOpen,   '}' => CharType::CurlyClose,
        '[' => CharType::BracketOpen, ']' => CharType::BracketClose,
        ' ' | '\n' | '\t' => CharType::Format,
        _ => CharType::Symbol
    }
}





fn push_token(out: &mut Stream, state: &CharType, buffer: &String, line_index: u32) {
    let buf_ref: &str = buffer.as_str();

    let data: TokenClass =  match *state {
        CharType::Invalid   => { return; },
        CharType::Format    => { return; },
        CharType::Alpha     => {
            let content: String = do_escape_sequences(buffer.clone());
            match buf_ref {
                x if KEYWORDS.contains(&x) => TokenClass::Keyword(content),
                _                          => TokenClass::Identifier(content),
            }
        },
        CharType::Num => match buf_ref {
            x if x.parse::<i64>().is_ok() => TokenClass::Integer(x.parse().unwrap()),
            x if x.parse::<f64>().is_ok() => TokenClass::Float  (x.parse().unwrap()),
            x => panic!("Error: Token '{}' looks like a number, but cannot be parsed.", x)
        },
        CharType::Quote     => TokenClass::String(buffer.trim_matches('"').to_string()),
        CharType::ParenOpen   => TokenClass::ParenOpen,   CharType::ParenClose   => TokenClass::ParenClose,
        CharType::CurlyOpen   => TokenClass::CurlyOpen,   CharType::CurlyClose   => TokenClass::CurlyClose,
        CharType::BracketOpen => TokenClass::BracketOpen, CharType::BracketClose => TokenClass::BracketClose,
        CharType::Symbol    => match buf_ref {
            "=" => TokenClass::Assign, ":=" => TokenClass::Define, 
            "+=" | "-=" | "*=" | "/=" => TokenClass::AssignOp(buffer.clone()),
            "," => TokenClass::Comma,
            ";" => TokenClass::EndOfStatement,
            "::" => TokenClass::Namespace,
            x if OPERATORS.contains(&x) => TokenClass::Operator(buffer.clone()),
            x => panic!("Error: Symbol '{}' cannot be categorized.", x),
        },
    };

    out.push(Token {
        data,
        line_index,
    }); 

}

pub fn lex(source: &String) -> Stream {
    let mut out = Stream { tokens: vec![], index: 0 };

    let mut buffer: String = Default::default();
    let mut last  = CharType::Invalid;
    let mut state;

    let mut line_index: u32 = 0;

    let mut in_comment: bool = false;
    let mut in_string : bool = false;

    for char in source.chars() {
        state = get_char_state(char);

        if buffer == "//" { in_comment = true; buffer.clear() }
        if last == CharType::Quote { in_string = !in_string; }

        if state != last && !in_comment && !in_string {
            push_token(&mut out, &last, &buffer, line_index);
            buffer.clear();
        }

        if char   == '\n' { in_comment = false; line_index += 1; }

        if !in_comment {
            buffer.push(char);
        }
        last = state;
    }

    return out;
}










