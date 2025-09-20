use core::fmt;




const KEYWORDS: [&str; 8] = [
    "fun",
    "while",
    "if",
    "elif",
    "else",
    "import",
    "for",
    "return",
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


#[derive(PartialEq, Debug, Clone)]
pub enum TokenClass {
    Operator(String),
    String(String),
    Integer(u64),
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

impl fmt::Display for TokenClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Operator(ref x)   => write!(f, "Operator({})", x),
            Self::String(ref x)     => write!(f, "String({})", x),
            Self::Integer(ref x)    => write!(f, "Integer({})", x),
            Self::Float(ref x)      => write!(f, "Float({})", x),
            Self::Identifier(ref x) => write!(f, "Identifier({})", x),
            Self::Keyword(ref x)    => write!(f, "Keyword({})", x),
            Self::EndOfStatement    => write!(f, "EndOfStatement(;)"),
            Self::Comma             => write!(f, "Comma"),
            Self::Define            => write!(f, "Define(:=)"),
            Self::Assign            => write!(f, "Assign(=)"),
            Self::AssignOp(ref x)   => write!(f, "AssignOp({})", x),
            Self::ParenOpen         => write!(f, "ParenOpen"),
            Self::ParenClose        => write!(f, "ParenClose"),
            Self::CurlyOpen         => write!(f, "CurlyOpen"),
            Self::CurlyClose        => write!(f, "CurlyClose"),
            Self::BracketOpen       => write!(f, "BracketOpen"),
            Self::BracketClose      => write!(f, "BracketClose"),
            Self::Namespace         => write!(f, "Namespace(::)")
        }
    }
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
    last_line_index: u32,
}

impl Stream {
    fn push (&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn has(&self) -> bool {
        return self.index < self.tokens.len();
    }

    fn get(&mut self, index: usize) -> &Token {
        let token = &self.tokens[index];
        self.last_line_index = token.line_index;
        token
    }

    pub fn peek(&self) -> Option<&Token> {
        dbg!("PEEK");
        if !self.has() { return None }
        dbg!(&self.tokens[self.index]);
        
        return Some(&self.tokens[self.index]);
    }

    pub fn next(&mut self) {
        dbg!("NEXT");
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<&Token> {
        dbg!("POP");
        dbg!(&self.tokens[self.index]);
        if !self.has() { return None }

        let i = self.index;
        self.next();
        return Some(self.get(i));
    }

    pub fn expect(&mut self, should: TokenClass) {
        dbg!("EXPECT");
        dbg!(&should);
        let Some(token) = self.pop() else { 
            self.error(format!("Expected {}, but end of token stream.", should).as_str()) 
        };
        let got = token.data.clone();
        if should != got {
            self.error(format!("Expected {}, but got {}.", should, got).as_str());
        }
    }

    pub fn error(&self, msg: &str) -> ! {
        eprintln!("Error at line {}: {}", self.last_line_index, msg);
        //std::process::exit(1);
        panic!("EVERYBODY PANIC!!!!")
    }

    pub fn maybe(&mut self, can: TokenClass) {
        dbg!("MAYBE");
        dbg!(&can);
        let Some(token) = self.peek() else { return; };
        let got = token.data.clone();
        if can == got { self.next(); }
    }

    pub fn lookhead(&self, offset: usize) -> Option<&TokenClass> {
        let at: usize = self.index + offset;
        if at >= self.tokens.len() { return None }
        
        return Some(&self.tokens[at].data);
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
            x if x.parse::<u64>().is_ok() => TokenClass::Integer(x.parse().unwrap()),
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

fn should_always_transition(state: &CharType) -> bool {
    match state {
        CharType::ParenOpen | CharType::ParenClose |
        CharType::CurlyOpen | CharType::CurlyClose |
        CharType::BracketOpen | CharType::BracketClose
          => true,
        _ => false
    }
}



pub fn lex(source: &String) -> Stream {
    let mut out = Stream { tokens: vec![], index: 0, last_line_index: 0};

    let mut buffer: String = Default::default();
    let mut last  = CharType::Invalid;
    let mut state;

    let mut line_index: u32 = 1;

    let mut in_comment: bool = false;
    let mut in_string : bool = false;

    for char in source.chars() {
        state = get_char_state(char);

        if buffer == "//" { in_comment = true; buffer.clear() }
        if last == CharType::Quote { in_string = !in_string; }

        //"transite" = (Lat.) "go over!" (imperative of "transire", "to transition")
        let transite = (state != last) || should_always_transition(&last);
        if transite && !in_comment && !in_string {
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










