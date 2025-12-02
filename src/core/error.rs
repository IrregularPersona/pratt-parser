use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self {
            line,
            column,
            length,
        }
    }

    pub fn unknown() -> Self {
        Self {
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    // Lexer Errors
    UnexpectedCharacter(char),
    InvalidIndentation {
        expected: usize,
        found: usize,
    },
    UnterminatedString,
    InvalidNumber(String),

    // Parser Errors
    UnexpectedToken {
        expected: String,
        found: String,
    },
    UnexpectedEOF,
    InvalidType(String),
    InvalidArraySize,
    MissingIdentifier,
    MissingReturnType,

    // Type Checker Errors
    TypeMismatch {
        expected: String,
        found: String,
    },
    UndefinedVariable(String),
    AlreadyDefined(String),
    InvalidOperator {
        op: String,
        left: String,
        right: String,
    },
    InvalidComparison {
        left: String,
        right: String,
    },
    InvalidUnaryOperator {
        op: String,
        operand: String,
    },
    InvalidCondition {
        found: String,
    },
    ReturnTypeMismatch {
        expected: String,
        found: String,
    },
    ReturnOutsideFunction,
    InvalidAssignment {
        target: String,
        value: String,
    },
    UndefinedFunction(String),
    WrongNumberOfArguments {
        expected: usize,
        found: usize,
    },
    InvalidIndexType {
        found: String,
    },
    InvalidFieldAccess {
        object: String,
        field: String,
    },

    // Semantic Errors
    ImmutableAssignment(String),
    UseBeforeDefinition(String),
    InvalidBreakContext,
    InvalidContinueContext,
}

pub struct CompilerError {
    pub kind: ErrorKind,
    pub span: Span,
    pub source_line: Option<String>,
    pub note: Option<String>,
}

impl CompilerError {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self {
            kind,
            span,
            source_line: None,
            note: None,
        }
    }

    pub fn with_source(mut self, source_line: String) -> Self {
        self.source_line = Some(source_line);
        self
    }

    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type = match &self.kind {
            ErrorKind::UnexpectedCharacter(_)
            | ErrorKind::InvalidIndentation { .. }
            | ErrorKind::UnterminatedString
            | ErrorKind::InvalidNumber(_) => "Lexer Error",

            ErrorKind::UnexpectedToken { .. }
            | ErrorKind::UnexpectedEOF
            | ErrorKind::InvalidType(_)
            | ErrorKind::InvalidArraySize
            | ErrorKind::MissingIdentifier
            | ErrorKind::MissingReturnType => "Parser Error",

            _ => "Type Error",
        };

        writeln!(
            f,
            "\n{} at line {}:{}",
            error_type.to_uppercase(),
            self.span.line,
            self.span.column
        )?;

        writeln!(f, "{}", self.error_message())?;

        if let Some(source) = &self.source_line {
            writeln!(f, "\n{:4} | {}", self.span.line, source)?;
            write!(f, "     | ")?;
            for _ in 0..self.span.column.saturating_sub(1) {
                write!(f, " ")?;
            }
            for _ in 0..self.span.length.max(1) {
                write!(f, "^")?;
            }
            writeln!(f)?;
        }

        if let Some(note) = &self.note {
            writeln!(f, "\nNote: {}", note)?;
        }

        Ok(())
    }
}

impl CompilerError {
    fn error_message(&self) -> String {
        match &self.kind {
            ErrorKind::UnexpectedCharacter(ch) => {
                format!("Unexpected character '{}'", ch)
            }
            ErrorKind::InvalidIndentation { expected, found } => {
                format!(
                    "Invalid indentation: expected {} spaces, found {}",
                    expected, found
                )
            }
            ErrorKind::UnterminatedString => "Unterminated string literal".to_string(),
            ErrorKind::InvalidNumber(num) => {
                format!("Invalid number format: '{}'", num)
            }
            ErrorKind::UnexpectedToken { expected, found } => {
                format!("Expected {}, found {}", expected, found)
            }
            ErrorKind::UnexpectedEOF => "Unexpected end of file".to_string(),
            ErrorKind::InvalidType(type_name) => {
                format!("Invalid type: '{}'", type_name)
            }
            ErrorKind::InvalidArraySize => "Array size must be a positive integer".to_string(),
            ErrorKind::MissingIdentifier => "Expected identifier".to_string(),
            ErrorKind::MissingReturnType => "Missing return type annotation".to_string(),
            ErrorKind::TypeMismatch { expected, found } => {
                format!("Type mismatch: expected '{}', found '{}'", expected, found)
            }
            ErrorKind::UndefinedVariable(name) => {
                format!("Undefined variable '{}'", name)
            }
            ErrorKind::AlreadyDefined(name) => {
                format!("Variable '{}' is already defined in this scope", name)
            }
            ErrorKind::InvalidOperator { op, left, right } => {
                format!(
                    "Cannot apply operator '{}' to types '{}' and '{}'",
                    op, left, right
                )
            }
            ErrorKind::InvalidComparison { left, right } => {
                format!("Cannot compare types '{}' and '{}'", left, right)
            }
            ErrorKind::InvalidUnaryOperator { op, operand } => {
                format!("Cannot apply unary operator '{}' to type '{}'", op, operand)
            }
            ErrorKind::InvalidCondition { found } => {
                format!("Condition must be of type 'bool', found '{}'", found)
            }
            ErrorKind::ReturnTypeMismatch { expected, found } => {
                format!(
                    "Function expects return type '{}', found '{}'",
                    expected, found
                )
            }
            ErrorKind::ReturnOutsideFunction => "Return statement outside of function".to_string(),
            ErrorKind::InvalidAssignment { target, value } => {
                format!("Cannot assign '{}' to '{}'", value, target)
            }
            ErrorKind::UndefinedFunction(name) => {
                format!("Undefined function '{}'", name)
            }
            ErrorKind::WrongNumberOfArguments { expected, found } => {
                format!("Expected {} arguments, found {}", expected, found)
            }
            ErrorKind::InvalidIndexType { found } => {
                format!("Array index must be an integer type, found '{}'", found)
            }
            ErrorKind::InvalidFieldAccess { object, field } => {
                format!("Type '{}' has no field '{}'", object, field)
            }
            ErrorKind::ImmutableAssignment(name) => {
                format!("Cannot assign to immutable variable '{}'", name)
            }
            ErrorKind::UseBeforeDefinition(name) => {
                format!("Variable '{}' used before definition", name)
            }
            ErrorKind::InvalidBreakContext => "Break statement outside of loop".to_string(),
            ErrorKind::InvalidContinueContext => "Continue statement outside of loop".to_string(),
        }
    }
}

// Helper type alias
pub type Result<T> = std::result::Result<T, CompilerError>;

// Error builder for convenience
pub struct ErrorBuilder {
    kind: ErrorKind,
    span: Span,
}

impl ErrorBuilder {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn build(self) -> CompilerError {
        CompilerError::new(self.kind, self.span)
    }

    pub fn with_source(self, source_line: String) -> CompilerError {
        CompilerError::new(self.kind, self.span).with_source(source_line)
    }

    pub fn with_note(self, source_line: String, note: String) -> CompilerError {
        CompilerError::new(self.kind, self.span)
            .with_source(source_line)
            .with_note(note)
    }
}

// Utility functions
pub fn format_type(t: &crate::core::ast::Type) -> String {
    use crate::core::ast::Type;
    match t {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Str => "string".to_string(),
        Type::Void => "void".to_string(),
        Type::NoneType => "none".to_string(),
        Type::I8 => "i8".to_string(),
        Type::I16 => "i16".to_string(),
        Type::I32 => "i32".to_string(),
        Type::I64 => "i64".to_string(),
        Type::I128 => "i128".to_string(),
        Type::U8 => "u8".to_string(),
        Type::U16 => "u16".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U64 => "u64".to_string(),
        Type::U128 => "u128".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Array(inner, size) => format!("[{}; {}]", format_type(inner), size),
        Type::Slice(inner) => format!("[{}]", format_type(inner)),
        Type::Option(inner) => format!("{}?", format_type(inner)),
        Type::Custom(name) => name.clone(),
        Type::Char => "char".to_string(),
        Type::Uint => "uint".to_string(),
    }
}

pub fn format_token(token: &crate::core::lexer::Token) -> String {
    use crate::core::lexer::Token;
    match token {
        Token::NumInt(i) => format!("integer '{}'", i),
        Token::NumFloat(f) => format!("float '{}'", f),
        Token::Ident(s) => format!("identifier '{}'", s),
        Token::StrLit(s) => format!("string \"{}\"", s),
        Token::BoolLit(b) => format!("boolean '{}'", b),
        Token::Plus => "'+'".to_string(),
        Token::Minus => "'-'".to_string(),
        Token::Star => "'*'".to_string(),
        Token::Slash => "'/'".to_string(),
        Token::Assign => "'='".to_string(),
        Token::ColonAssign => "':='".to_string(),
        Token::Arrow => "'->'".to_string(),
        Token::Colon => "':'".to_string(),
        Token::Semicolon => "';'".to_string(),
        Token::KwFnc => "keyword 'fnc'".to_string(),
        Token::KwRet => "keyword 'ret'".to_string(),
        Token::KwMut => "keyword 'mut'".to_string(),
        Token::KwIf => "keyword 'if'".to_string(),
        Token::KwElse => "keyword 'else'".to_string(),
        Token::EOF => "end of file".to_string(),
        _ => format!("{:?}", token),
    }
}
