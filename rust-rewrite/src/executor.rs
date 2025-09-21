

use std::fmt::{Display, Pointer};
use std::{collections::HashMap, fmt::Debug};
use std::rc::Rc;
use crate::{lexer, parser};


#[derive(Clone)]
pub struct Fun {
    args: Vec<String>,
    body: Rc<parser::StatSeq>,
}


#[derive(Clone)]
pub struct Scope {
    pub vars: HashMap<String, Obj>,
    pub funs: HashMap<String, Fun>,
    pub ret_val: Obj,
    pub ret_flag: bool,
}

#[derive(Clone)]
pub enum Obj {
    Invalid,
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Obj>),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::Invalid    => error("Cannot render invalid object.".to_string()),
            Obj::Nil        => write!(f, "Nil"),
            Obj::Int(x)     => write!(f, "{}", x),
            Obj::Float(x)   => write!(f, "{}", x),
            Obj::String(x)  => write!(f, "{}", x),
            Obj::Bool(x)    => write!(f, "{}", x),
            Obj::Array(x)   => {
                write!(f, "[")?;
                for elem in x {
                    elem.fmt(f)?;
                    write!(f, ", ")?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}


fn error(msg: String) -> ! {
    eprintln!("Runtime Error: {}", msg);
    std::process::exit(1);
}


//this is formulated as a function for performance reasons
fn apply_binary_op(lhs: &Obj, rhs: &Obj, op: &str) -> Obj {
    if let Obj::Nil = lhs { return Obj::Nil }
    if let Obj::Nil = rhs { return Obj::Nil }

    match op {
        "+" => match (lhs, rhs) {
            (Obj::Int(x),    Obj::Int(y)   ) => Obj::Int   (x         + y),
            (Obj::String(x), Obj::String(y)) => Obj::String(x.clone() + y),
            (Obj::Float(x),  Obj::Float(y) ) => Obj::Float (x         + y),
            _ => error("Unable to perform addition of divergent types.".to_string()),
        },
        "-" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Int  (x - y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Float(x - y),
            _ => error("Unable to perform subtraction of divergent types.".to_string()),
        },
        "*" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Int  (x * y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Float(x * y),
            _ => error("Unable to perform multiplication of divergent types.".to_string()),
        },
        "/" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Int  (x / y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Float(x / y),
            _ => error("Unable to perform division of divergent types.".to_string()),
        },
        "^" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Int  (x.pow(*y as u32)),
            (Obj::Float(x), Obj::Float(y)) => Obj::Float(x.powf(*y)),
            _ => error("Unable to perform exponentiation of divergent types.".to_string()),
        },
        "%" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Int  (x % y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Float(x % y),
            _ => error("Unable to perform modulo of divergent types.".to_string()),
        },
        ">" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x > y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x > y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        "<" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x < y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x < y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        ">=" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x >= y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x >= y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        "<=" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x <= y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x <= y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        "==" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x == y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x == y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        "!=" => match (lhs, rhs) {
            (Obj::Int(x),   Obj::Int(y)  ) => Obj::Bool(x != y),
            (Obj::Float(x), Obj::Float(y)) => Obj::Bool(x != y),
            _ => error("Unable to perform comparision of divergent types.".to_string()),
        },
        "&&" => match (lhs, rhs) {
            (Obj::Bool(x), Obj::Bool(y)) => Obj::Bool(*x && *y),
            _ => error("Unable to perform boolean and of non-boolean types.".to_string()),
        },
        "||" => match (lhs, rhs) {
            (Obj::Bool(x), Obj::Bool(y)) => Obj::Bool(*x || *y),
            _ => error("Unable to perform boolean or of non-boolean types.".to_string()),
        },
        _ => Obj::Invalid,
    }

}


//these should all makes sense.
//rust doesn't have truthy values, because it's an actually programming language.
//(unlike *cough* JS *cough*, jk ofc :3)
fn truthiness(obj: Obj) -> bool {
    match obj {
        Obj::Bool(x)       => x,
        Obj::Nil           => false,
        Obj::Invalid       => false,
        Obj::String(ref x) => x.len() > 0,
        Obj::Int(x)        => x != 0,
        Obj::Float(x)      => x != 0.0,
        Obj::Array(x)      => x.len() > 0,
    }
}




impl parser::Nodeable for parser::StatSeq {
    fn eval(&self, scope: &mut Scope) -> Obj {
        for node in &self.nodes {
            node.eval(scope);

            if scope.ret_flag { break; }
        }

        Obj::Invalid
    }
}

impl parser::Nodeable for parser::ImportStat {
    fn eval(&self, _: &mut Scope) -> Obj {
        //built-in's are compiled into execute,
        //so "importing" doesn't have semantic meaning.
        //there is not to "load" into memory.
        Obj::Invalid 
    }
}

impl parser::Nodeable for parser::VariableAssign {
    fn eval(&self, scope: &mut Scope) -> Obj {
        match self.op {
            lexer::TokenClass::Define => {
                if scope.vars.contains_key(&self.var_name) {
                    error(format!("Variable of name {} is already defined in scope.", self.var_name));
                }
                let expr = self.expr.eval(scope);
                scope.vars.insert(self.var_name.clone(), expr);
            },
            lexer::TokenClass::Assign => {
                if let None = scope.vars.remove(&self.var_name) {
                    error(format!("Variable of name {} is not defined in scope.", self.var_name)); 
                }
                let expr = self.expr.eval(scope);
                scope.vars.insert(self.var_name.clone(), expr);
            }
            lexer::TokenClass::AssignOp(ref x) => {
                let expr = &self.expr.eval(scope);
                let Some(ref var) = scope.vars.remove(&self.var_name) else { 
                    error(format!("Variable of name {} is not defined in scope.", self.var_name)); 
                };
                let new = match x.as_str() {
                    "+=" => apply_binary_op(var, expr, "+"),
                    "-=" => apply_binary_op(var, expr, "-"),
                    "*=" => apply_binary_op(var, expr, "*"),
                    "/=" => apply_binary_op(var, expr, "/"),
                    _ => unreachable!(),
                };
                scope.vars.insert(self.var_name.clone(), new);
            },
            _ => unreachable!(),
        }

        Obj::Invalid
    }
}

impl parser::Nodeable for parser::BinaryExpr {
    fn eval(&self, scope: &mut Scope) -> Obj {
        let left  = self.left.eval(scope);
        let right = self.right.eval(scope);

        apply_binary_op(&left, &right, self.op.as_str())
    }
}

impl parser::Nodeable for parser::UnaryExpr {
    fn eval(&self, scope: &mut Scope) -> self::Obj {
        let expr = self.operand.eval(scope);
        match (self.op.as_str(), expr) {
            ("!", Obj::Bool(x))   => Obj::Bool(!x),
            ("-", Obj::Int(x))    => Obj::Int(-x),
            ("-", Obj::Float(x))  => Obj::Float(-x),
            _ => error(format!("Unable to perform unary operator {} on given type.", self.op)),
        }
    }
}

impl parser::Nodeable for parser::IntLiteral {
    fn eval(&self, _: &mut Scope) -> Obj {
        //note that: int literal may only be unsigned,
        //while object ints can be signed
        Obj::Int(self.value as i64)
    }
}

impl parser::Nodeable for parser::FloatLiteral {
    fn eval(&self, _: &mut Scope) -> Obj {
        Obj::Float(self.value)
    }
}

impl parser::Nodeable for parser::StrLiteral {
    fn eval(&self, _: &mut Scope) -> Obj {
        Obj::String(self.value.clone())
    }
}

impl parser::Nodeable for parser::Variable {
    fn eval(&self, scope: &mut Scope) -> Obj {
        let vars = &scope.vars;
        let Some(value) = vars.get(&self.name) else {
            error(format!("Variable of name {} is not defined in scope.", self.name));
        };
        value.clone()
    }
}

impl parser::Nodeable for parser::FunctionCall {
    fn eval(&self, scope: &mut Scope) -> Obj {

        //pre-evaluate argument expressions
        let mut arg_vals: Vec<Obj> = vec![];
        for arg in &self.args {
            arg_vals.push(arg.eval(scope));
        }

        let funs = &scope.funs;
        let Some(fun) = funs.get(&self.name) else {
            error(format!("Function of name {} is not declared in scope.", self.name));
        };

        //explicit clone to enable scope teardown
        let mut inner_scope = scope.clone();

        //shouldn't be set anyways
        inner_scope.ret_val = Obj::Invalid;
        inner_scope.ret_flag = false; 

        //inject args
        for (arg_val, arg_name) in std::iter::zip(arg_vals, &fun.args) {
            inner_scope.vars.insert(arg_name.clone(), arg_val);
        }

        fun.body.eval(&mut inner_scope);

        inner_scope.ret_val
    }
}

impl parser::Nodeable for parser::ModAccess {
    fn eval(&self, scope: &mut Scope) -> Obj {
        match (self.mod_name.as_str(), self.member.name.as_str()) {
            ("io", "println") => println!("dbg out: {}", self.member.args[0].eval(scope)),
            _ => todo!(),
        };
        Obj::Invalid
    }
}

impl parser::Nodeable for parser::ArrayLiteral {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        Obj::Array(
            self.elem
                .iter()
                .map(|x| x.eval(scope))
                .collect()
        )
    }
}

impl parser::Nodeable for parser::ReturnStat {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        if let Some(expr) = &self.expr {
            scope.ret_val = expr.eval(scope);
        }
        scope.ret_flag = true;

        Obj::Invalid
    }
}

impl parser::Nodeable for parser::FunctionDeclare {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        scope.funs.insert(self.name.clone(), Fun {
            args: self.args.clone(),
            body: self.body.clone(),
        });

        Obj::Invalid
    }
}


impl parser::Nodeable for parser::ExprStat {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        self.expr.eval(scope);
        Obj::Invalid
    }
}

impl parser::Nodeable for parser::IfStat {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        let cond_val = self.condition.eval(scope);

        if truthiness(cond_val) {
            self.if_block.eval(scope)
        } else if let Some(else_block) = &self.else_block {
            else_block.eval(scope)
        } else {
            Obj::Invalid
        }
    }
}

impl parser::Nodeable for parser::WhileStat {
    fn eval(&self, scope: &mut self::Scope) -> Obj {
        while truthiness(self.condition.eval(scope)) {
            self.body.eval(scope);
        }
        
        Obj::Invalid
    }
}

impl parser::Nodeable for parser::ForStat {
    fn eval(&self, scope: &mut self::Scope) -> self::Obj {
        let Obj::Array(arr) = self.array.eval(scope) else {
            error("Unable to iterate non-array type.".to_string());
        };

        for elem in arr {
            let mut inner_scope = scope.clone();
            inner_scope.vars.insert(self.elem_name.clone(), elem);

            self.body.eval(&mut inner_scope);
        }

        Obj::Invalid
    }
}




