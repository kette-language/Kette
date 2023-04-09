use crate::lexer::{Lexer, Word};
use crate::symbol::{SymbolId, SymbolKind, SymbolStorage};

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
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

    pub fn to_string(&self) -> String {
        match self {
            Builtin::Number(num) => {
                return match num {
                    Number::Uint(val) => val.to_string(),
                    Number::Int(val) => val.to_string(),
                    Number::Ratio(a, b) => format!("{}/{}", a.to_string(), b.to_string()),
                    Number::Float(val) => val.to_string(),
                }
            }
            Builtin::Add => "+",
            Builtin::Sub => "-",
            Builtin::Mul => "*",
            Builtin::Div => "/",
            Builtin::DotPrint => ".",
            Builtin::Drop => "drop",
        }.to_string()
    }

    pub fn add_symbol(self, symbols: &mut SymbolStorage) -> SymbolId {
        symbols.insert_or_get(self.to_string(), self.to_string(), SymbolKind::Builtin(self))
    }
}