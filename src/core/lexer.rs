#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    NumInt(i64),
    NumFloat(f64),
    Ident(String),
    StrLit(String),
    BoolLit(bool),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Modulo,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Question,
    Dot,
    DoubleColon,

    // Comparison Ops
    EqEq,  // ==
    NotEq, // !=
    Lt,    // <
    Gt,    // >
    LtEq,  // <=
    GtEq,  // >=

    // Assign
    ColonAssign, // :=
    Arrow,       // ->
    Assign,      // =

    // Keywords
    KwFnc,
    KwRet,
    KwMut,
    KwIf,
    KwElse,
    KwNone,
    KwAnd,
    KwOr,
    KwNot,
    KwForeach,
    KwBy,
    KwAs,
    KwStruct,
    KwTrue,
    KwFalse,
    KwContinue,

    DotDot,   // ..
    DotDotEq, // ..=

    Indent,
    Dedent,
    Newline,

    EOF,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut indent_stack = vec![0];

    for (line_num, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let indent_level = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();

        if indent_level > *indent_stack.last().unwrap() {
            indent_stack.push(indent_level);
            tokens.push(Token::Indent);
        } else if indent_level < *indent_stack.last().unwrap() {
            while indent_stack.len() > 1 && indent_level < *indent_stack.last().unwrap() {
                indent_stack.pop();
                tokens.push(Token::Dedent);
            }

            if indent_level != *indent_stack.last().unwrap() {
                return Err(format!(
                    "Line {}: Invalid indentation level {}",
                    line_num + 1,
                    indent_level
                ));
            }
        }

        let mut chars = trimmed.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_alphanumeric() || d == '_' {
                            ident.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    // Check keywords
                    let token = match ident.as_str() {
                        "fnc" => Token::KwFnc,
                        "ret" => Token::KwRet,
                        "mut" => Token::KwMut,
                        "if" => Token::KwIf,
                        "else" => Token::KwElse,
                        "none" => Token::KwNone,
                        "and" => Token::KwAnd,
                        "or" => Token::KwOr,
                        "not" => Token::KwNot,
                        "foreach" => Token::KwForeach,
                        "by" => Token::KwBy,
                        "as" => Token::KwAs,
                        "struct" => Token::KwStruct,
                        "true" => Token::BoolLit(true),
                        "false" => Token::BoolLit(false),
                        _ => Token::Ident(ident),
                    };
                    tokens.push(token);
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    let mut has_dot = false;

                    while let Some(&d) = chars.peek() {
                        if d.is_digit(10) {
                            num_str.push(d);
                            chars.next();
                        } else if d == '.' && !has_dot {
                            let mut temp_chars = chars.clone();
                            temp_chars.next();
                            if let Some(&next) = temp_chars.peek() {
                                if next == '.' {
                                    break;
                                }
                            }

                            has_dot = true;
                            num_str.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if has_dot {
                        tokens.push(Token::NumFloat(num_str.parse().unwrap()));
                    } else {
                        tokens.push(Token::NumInt(num_str.parse().unwrap()));
                    }
                }
                '"' => {
                    chars.next();
                    let mut string = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next();
                            break;
                        }

                        string.push(c);
                        chars.next();
                    }

                    tokens.push(Token::StrLit(string));
                }
                ':' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        match next {
                            '=' => {
                                chars.next();
                                tokens.push(Token::ColonAssign); // :=
                            }
                            ':' => {
                                chars.next();
                                tokens.push(Token::DoubleColon); // ::
                            }
                            _ => {
                                chars.next();
                                tokens.push(Token::Colon);
                            }
                        }
                    } else {
                        tokens.push(Token::Colon);
                    }
                }
                '.' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '.' {
                            chars.next();
                            if let Some(&next2) = chars.peek() {
                                if next2 == '=' {
                                    chars.next();
                                    tokens.push(Token::DotDotEq);
                                } else {
                                    tokens.push(Token::DotDot);
                                }
                            } else {
                                tokens.push(Token::DotDot);
                            }
                        } else {
                            tokens.push(Token::Dot);
                        }
                    } else {
                        tokens.push(Token::Dot);
                    }
                }
                '-' => {
                    chars.next();
                    if let Some('>') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Arrow); // ->
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '=' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::EqEq);
                    } else {
                        tokens.push(Token::Assign);
                    }
                }

                '!' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::NotEq);
                    } else {
                        return Err(format!("Unexpected char: !"));
                    }
                }

                '<' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::LtEq);
                    } else {
                        tokens.push(Token::Lt);
                    }
                }

                '>' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::GtEq);
                    } else {
                        tokens.push(Token::Gt);
                    }
                }

                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Star);
                    chars.next();
                }
                '/' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            break;
                        } else {
                            tokens.push(Token::Slash);
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '^' => {
                    tokens.push(Token::Power);
                    chars.next();
                }
                '%' => {
                    tokens.push(Token::Modulo);
                    chars.next();
                }
                '[' => {
                    tokens.push(Token::LBracket);
                    chars.next();
                }
                ']' => {
                    tokens.push(Token::RBracket);
                    chars.next();
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    chars.next();
                }
                '}' => {
                    tokens.push(Token::RBrace);
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
                '?' => {
                    tokens.push(Token::Question);
                    chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    chars.next();
                }
                ' ' | '\t' => {
                    chars.next();
                } // skip whitespace
                _ => return Err(format!("Unexpected char: {}", c)),
            }
        }
        tokens.push(Token::Newline);
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token::Dedent);
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}
