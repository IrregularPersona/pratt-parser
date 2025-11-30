use crate::core::lexer::Token;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Base Types
    Char,
    Int,
    Uint,
    Float,
    Bool,

    // Explicit Types
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,

    // Others
    Str,      // String
    Void,     // return type
    NoneType, // Null types

    // Complex Types
    Array(Box<Type>, usize), // [int; 10]
    Slice(Box<Type>),        // [int]
    Option(Box<Type>),       // Optionals for variable assignment or return types
    Custom(String),          // Classes and Structs for like Person
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralValue),
    Variable(String),
    Binary {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
    Range {
        // for 0 .. 1 syntax
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
        //op: Option<Token> will need to include op for Gt or Lt
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Str(String),
    None, // None keyword
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        name: String,
        var_ty: Option<Type>, // Keep this as var_ty
        value: Expr,
        is_mut: bool,
    },

    Function {
        name: String,
        params: Vec<(String, Type)>,
        ret_type: Type,
        body: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Foreach {
        variable: String,
        var_ty: Option<Type>,
        range: Expr,
        step: Option<Expr>, // 'by' clause
        body: Vec<Stmt>,
    },
    Return(Option<Expr>), // explicit and implicit returns
    Expression(Expr),
}
