use crate::lexer::{Lexer, Word};

#[derive(Debug)]
pub enum Number {
    Uint(u64),
    Int(i64),
    Ratio(i64, i64),
    Float(f64),
}

impl Number {
    pub fn parse(word: &str) -> Option<Self> {
        if let Ok(n) = word.parse::<u64>() {
            return Some(Number::Uint(n));
        }
        if let Ok(n) = word.parse::<i64>() {
            return Some(Number::Int(n));
        }
        if let Ok(n) = word.parse::<f64>() {
            return Some(Number::Float(n));
        }
        None
    }
}

#[derive(Debug)]
pub enum Builtin {
    Number(Number),
    Add,
    Sub,
    Mul,
    Div,
    DotPrint,
    Drop,
}

impl Builtin {
    pub fn from_str(word: &str) -> Option<Builtin> {
        let builtin = match word {
            "+" => Some(Builtin::Add),
            "-" => Some(Builtin::Sub),
            "*" => Some(Builtin::Mul),
            "/" => Some(Builtin::Div),
            "." => Some(Builtin::DotPrint),
            "drop" => Some(Builtin::Drop),
            _ => None,
        };
        if builtin.is_some() {
            return builtin;
        }
        if let Some(number) = Number::parse(word) {
            return Some(Builtin::Number(number));
        }
        return None;
    }
}