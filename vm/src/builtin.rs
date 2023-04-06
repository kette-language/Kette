use crate::lexer::Word;

pub enum Number {
    Uint(u64),
    Int(i64),
    Ratio(i64, i64),
    Float(f64),
    Uint32(u32),
    Int32(i32),
    Ratio32(i32, i32),
    Float32(f32),
}

impl Number {
    pub fn parse(word: &str) -> Option<Self> {
        if let Ok(n) = word.parse::<u32>() {
            return Some(Number::Uint32(n));
        }
        if let Ok(n) = word.parse::<u64>() {
            return Some(Number::Uint(n));
        }
        if let Ok(n) = word.parse::<i32>() {
            return Some(Number::Int32(n));
        }
        if let Ok(n) = word.parse::<i64>() {
            return Some(Number::Int(n));
        }
        if let Ok(n) = word.parse::<f32>() {
            return Some(Number::Float32(n));
        }
        if let Ok(n) = word.parse::<f64>() {
            return Some(Number::Float(n));
        }
        None
    }
}

pub enum Builtin {
    Number(Number),
    Add,
    Sub,
    Mul,
    Div,
    DotPrint,
}

impl Builtin {
    pub fn from_str(word: &str) -> Option<Builtin> {
        let builtin = match word {
            "+" => Some(Builtin::Add),
            "-" => Some(Builtin::Sub),
            "*" => Some(Builtin::Mul),
            "/" => Some(Builtin::Div),
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
