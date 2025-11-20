#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(Number),   // holds the new Number enum
    Ident(String), // for sin cos tan or anything i wanna add really xd
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Comma,
    Modulo,
    LParen,
    RParen,
    EOF,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();

                // needed to be while, not if gng
                while let Some(&d) = chars.peek() {
                    // alphanumeric bro...
                    if d.is_ascii_alphanumeric() {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Ident(ident));
            }

            '0'..='9' => {
                let mut num_str = String::new();
                let mut is_float = false;

                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) {
                        num_str.push(d);
                        chars.next();
                    } else if d == '.' {
                        if is_float {
                            break;
                        }
                        is_float = true;
                        num_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if is_float {
                    match num_str.parse::<f64>() {
                        Ok(f) => tokens.push(Token::Num(Number::Float(f))),
                        Err(_) => return Err(format!("invalid float: {}", num_str)),
                    }
                } else {
                    match num_str.parse::<i64>() {
                        Ok(i) => tokens.push(Token::Num(Number::Int(i))),
                        Err(_) => return Err(format!("invalid integer: {}", num_str)),
                    }
                }
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Power);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Modulo);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ' ' | '\t' => {
                chars.next();
            }
            _ => return Err(format!("unexpected character: {}", c)),
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn apply_function(name: &str, args: Vec<Number>) -> Number {
        let _ensure_arity = |n: usize| {
            if args.len() != n {
                panic!(
                    "Function '{}' expects {} arguments, got {}",
                    name,
                    n,
                    args.len()
                );
            }
        };

        match name {
            "min" => {
                if args.is_empty() {
                    panic!("min requires at least 1 argument");
                }
                let min_val = args
                    .iter()
                    .map(|n| n.to_float())
                    .fold(f64::INFINITY, |a, b| a.min(b));
                Number::Float(min_val)
            }
            "max" => {
                if args.is_empty() {
                    panic!("max requires at least 1 argument");
                }
                let max_val = args
                    .iter()
                    .map(|n| n.to_float())
                    .fold(f64::NEG_INFINITY, |a, b| a.max(b));
                Number::Float(max_val)
            }

            "atan2" => {
                if args.len() != 2 {
                    panic!("atan2 requires 2 arguments");
                }
                let y = args[0].to_float();
                let x = args[1].to_float();
                Number::Float(y.atan2(x))
            }
            "hypot" => {
                if args.len() != 2 {
                    panic!("hypot requires 2 arguments");
                }
                let x = args[0].to_float();
                let y = args[1].to_float();
                Number::Float(x.hypot(y))
            }

            "pow" => {
                if args.len() != 2 {
                    panic!("pow requires 2 arguments");
                }

                let x = args[0].to_int();
                let y = args[1].to_int();
                Number::Int(x.pow(y as u32)) // hopefully it doesnt overflow lmfao
            }

            _ => {
                if args.len() != 1 {
                    panic!("Function '{}' expects 1 argument, got {}", name, args.len());
                }

                let x = args[0].to_float();

                match name {
                    "exp" => Number::Float(x.exp()),
                    "log" => Number::Float(x.ln()), // natural log just incase i forgor
                    "log10" => Number::Float(x.log10()),
                    "log2" => Number::Float(x.log2()),
                    "sqrt" => Number::Float(x.sqrt()),
                    "abs" => Number::Float(x.abs()),
                    "floor" => Number::Float(x.floor()),
                    "ceil" => Number::Float(x.ceil()),
                    "round" => Number::Float(x.round()),
                    "sin" => Number::Float(x.sin()),
                    "cos" => Number::Float(x.cos()),
                    "tan" => Number::Float(x.tan()),
                    "asin" => Number::Float(x.asin()),
                    "acos" => Number::Float(x.acos()),
                    "atan" => Number::Float(x.atan()),
                    "sinh" => Number::Float(x.sinh()),
                    "cosh" => Number::Float(x.cosh()),
                    "tanh" => Number::Float(x.tanh()),

                    _ => panic!("Unknown function: {}", name),
                }
            }
        }
    }

    fn binding_power(token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 10,
            Token::Star | Token::Slash | Token::Modulo => 20,
            Token::Power => 30,
            _ => 0,
        }
    }

    fn nud(&mut self) -> Number {
        let token = self.current().clone();
        self.advance();

        match token {
            Token::Num(val) => val,
            Token::Minus => {
                let val = self.expression(100);
                match val {
                    Number::Int(i) => Number::Int(-i),
                    Number::Float(f) => Number::Float(-f),
                }
            }
            Token::LParen => {
                let val = self.expression(0);
                if let Token::RParen = self.current() {
                    self.advance();
                    val
                } else {
                    panic!("Expected ')'");
                }
            }
            Token::Ident(name) => {
                if let Token::LParen = self.current() {
                    self.advance(); // eat (

                    let mut args = Vec::new();

                    if !matches!(self.current(), Token::RParen) {
                        // check if its rparen
                        loop {
                            args.push(self.expression(0)); // eval the expr

                            // if current token is a comma, we eat the comma and loop again,
                            // if not we break
                            if let Token::Comma = self.current() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    if let Token::RParen = self.current() {
                        self.advance(); // eat rparen )
                    } else {
                        panic!("Expected ')' after function arguments");
                    }

                    return Parser::apply_function(&name, args);
                } else {
                    match name.as_str() {
                        "pi" | "PI" => Number::Float(std::f64::consts::PI),
                        "e" | "E" => Number::Float(std::f64::consts::E),
                        "tau" | "TAU" => Number::Float(std::f64::consts::TAU),
                        _ => panic!("Unknown variable or function: {}", name),
                    }
                }
            }
            _ => panic!("Unexpected token at start: {:?}", token),
        }
    }

    fn led(&mut self, left: Number, op: Token) -> Number {
        let bp = Parser::binding_power(&op);

        // cuz need right associativity ....
        let right_bp = if let Token::Power = op { bp - 1 } else { bp };

        let right = self.expression(right_bp);
        left.operate(&op, right)
    }

    pub fn expression(&mut self, rbp: u8) -> Number {
        let mut left = self.nud();

        while rbp < Parser::binding_power(self.current()) {
            let op = self.current().clone();
            self.advance();
            left = self.led(left, op);
        }
        left
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    fn operate(self, op: &Token, other: Number) -> Number {
        use Number::*;

        let as_float = |n: Number| match n {
            Int(i) => i as f64,
            Float(f) => f,
        };

        match op {
            Token::Plus => match (self, other) {
                (Int(a), Int(b)) => Int(a + b),
                _ => Float(as_float(self) + as_float(other)),
            },
            Token::Minus => match (self, other) {
                (Int(a), Int(b)) => Int(a - b),
                _ => Float(as_float(self) - as_float(other)),
            },
            Token::Star => match (self, other) {
                (Int(a), Int(b)) => Int(a * b),
                _ => Float(as_float(self) * as_float(other)),
            },
            Token::Slash => {
                let a = as_float(self);
                let b = as_float(other);
                Float(a / b)
            }
            Token::Power => {
                let base = as_float(self);
                let exponent = as_float(other);
                Number::Float(base.powf(exponent))
            }
            Token::Modulo => match (self, other) {
                (Int(a), Int(b)) => {
                    if b == 0 {
                        panic!("Modulo by zero");
                    }
                    Int(a % b)
                }
                _ => {
                    let a = as_float(self);
                    let b = as_float(other);
                    Float(a % b)
                }
            },
            _ => panic!("Invalid operator"),
        }
    }

    pub fn to_float(self) -> f64 {
        match self {
            Number::Float(f) => f,
            Number::Int(i) => i as f64,
        }
    }

    pub fn to_int(self) -> i64 {
        match self {
            Number::Int(i) => i,
            Number::Float(f) => f as i64,
        }
    }
}

use std::fmt;

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(val) => write!(f, "{}", val),
            Number::Float(val) => {
                if val.fract() == 0.0 {
                    write!(f, "{}", *val as i64)
                } else {
                    write!(f, "{}", val)
                }
            }
        }
    }
}
