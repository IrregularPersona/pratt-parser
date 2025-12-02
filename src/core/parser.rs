use crate::core::ast::{Expr, LiteralValue, Stmt, Type};
use crate::core::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        if self.pos >= self.tokens.len() {
            return &Token::EOF;
        }
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        } else {
            // Point to EOF
            self.pos = self.tokens.len();
        }
    }

    fn consume(&mut self, expected: Token) {
        if self.current() == &expected {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current());
        }
    }

    fn consume_ident(&mut self) -> String {
        if let Token::Ident(name) = self.current() {
            let n = name.clone();
            self.advance();
            n
        } else {
            panic!("Expected identifier, found {:?}", self.current());
        }
    }

    fn skip_newlines(&mut self) {
        while self.current() == &Token::Newline {
            self.advance();
        }
    }

    fn peek_token(&self) -> &Token {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1]
        } else {
            &Token::EOF
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.current() != &Token::EOF {
            if self.current() == &Token::Newline {
                self.advance();
                continue;
            }

            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        match self.current() {
            Token::KwStruct => self.struct_declaration(),
            Token::KwFnc => self.func_declaration(),
            Token::KwMut => self.var_declaration(true),
            Token::KwIf => self.if_statement(),
            Token::KwForeach => self.foreach_statement(),
            Token::KwRet => {
                self.advance();

                if self.current() == &Token::Newline || self.current() == &Token::Dedent {
                    Stmt::Return(None)
                } else {
                    let expr = self.expression(0);
                    Stmt::Return(Some(expr))
                }
            }
            Token::Ident(_) => {
                let next = if self.pos + 1 < self.tokens.len() {
                    &self.tokens[self.pos + 1]
                } else {
                    &Token::EOF
                };

                match next {
                    Token::ColonAssign | Token::Colon => self.var_declaration(false),
                    Token::Assign => {
                        let target = self.expression(50);
                        self.consume(Token::Assign);
                        let value = self.expression(0);
                        Stmt::Assign { target, value }
                    }
                    _ => {
                        let expr = self.expression(0);
                        if self.current() == &Token::Assign {
                            self.advance();
                            let val = self.expression(0);
                            Stmt::Assign {
                                target: expr,
                                value: val,
                            }
                        } else {
                            Stmt::Expression(expr)
                        }
                    }
                }
            }
            _ => {
                let expr = self.expression(0);
                Stmt::Expression(expr)
            }
        }
    }

    fn var_declaration(&mut self, explicit_mut: bool) -> Stmt {
        if explicit_mut {
            self.consume(Token::KwMut);
        }

        let name = self.consume_ident();
        let mut var_ty = None;

        if self.current() == &Token::Colon {
            self.advance();
            var_ty = Some(self.parse_type());
            self.consume(Token::Assign);
        } else if self.current() == &Token::ColonAssign {
            self.advance();
        } else {
            panic!("Expected ':=' or ':' after variable name");
        }

        let value = self.expression(0);

        Stmt::VarDecl {
            name,
            var_ty,
            value,
            is_mut: explicit_mut,
        }
    }

    fn struct_declaration(&mut self) -> Stmt {
        self.consume(Token::KwStruct);
        let name = self.consume_ident();
        self.consume(Token::Colon);
        self.skip_newlines();
        self.consume(Token::Indent);

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.current() != &Token::Dedent && self.current() != &Token::EOF {
            self.skip_newlines();
            if self.current() == &Token::Dedent {
                break;
            }

            match self.current() {
                Token::KwFnc => {
                    methods.push(self.func_declaration());
                }
                Token::KwMut => {
                    // Handle mutable fields
                    self.advance(); // consume 'mut'
                    let f_name = self.consume_ident();
                    self.consume(Token::Colon);
                    let f_type = self.parse_type();
                    fields.push((f_name, f_type));
                    if self.current() == &Token::Newline {
                        self.advance();
                    }
                }
                Token::Ident(_) => {
                    let f_name = self.consume_ident();
                    self.consume(Token::Colon);
                    let f_type = self.parse_type();
                    fields.push((f_name, f_type));
                    if self.current() == &Token::Newline {
                        self.advance();
                    }
                }
                _ => panic!("Unexpected token inside struct: {:?}", self.current()),
            }
        }

        self.consume(Token::Dedent);
        Stmt::Struct {
            name,
            fields,
            methods,
        }
    }

    fn func_declaration(&mut self) -> Stmt {
        self.consume(Token::KwFnc);
        let name = self.consume_ident();

        self.consume(Token::LParen);
        let mut params = Vec::new();

        self.skip_newlines();

        if self.current() != &Token::RParen {
            loop {
                // Handle 'mut' modifier for parameters
                if self.current() == &Token::KwMut {
                    self.advance(); // skip 'mut'
                }

                let p_name = self.consume_ident();
                
                // Check if there's a colon (for typed parameters)
                // or comma/rparen (for self parameter without type)
                if self.current() == &Token::Colon {
                    self.advance();
                    let p_type = self.parse_type();
                    params.push((p_name, p_type));
                } else {
                    // Assume it's 'self' or similar, treat as custom type
                    params.push((p_name.clone(), Type::Custom(p_name)));
                }

                if self.current() == &Token::Comma {
                    self.advance();
                    self.skip_newlines();
                } else {
                    break;
                }
            }
        }

        self.skip_newlines();
        self.consume(Token::RParen);

        let mut ret_type = Type::Void;
        if self.current() == &Token::Arrow {
            self.advance();
            ret_type = self.parse_type();
        }

        self.consume(Token::Colon);

        if self.current() == &Token::Newline {
            self.advance();
        }

        let body = self.parse_block();

        Stmt::Function {
            name,
            params,
            ret_type,
            body,
        }
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(Token::KwIf);
        let condition = self.expression(0);
        self.consume(Token::Colon);

        if self.current() == &Token::Newline {
            self.advance();
        }

        let then_branch = self.parse_block();
        let mut else_branch = None;

        if self.current() == &Token::KwElse {
            self.advance();
            self.consume(Token::Colon);

            if self.current() == &Token::Newline {
                self.advance();
            }

            else_branch = Some(self.parse_block());
        }

        Stmt::If {
            cond: condition,
            then_branch,
            else_branch,
        }
    }

    fn foreach_statement(&mut self) -> Stmt {
        self.consume(Token::KwForeach);
        let variable = self.consume_ident();

        let mut var_type = Type::NoneType;
        if self.current() == &Token::Colon {
            self.advance();
            var_type = self.parse_type();
        }

        self.consume(Token::Semicolon);
        
        // Parse the range expression directly
        let range_expr = self.expression(0);
        
        // Extract start, end, and inclusive from the range expression
        let (start, end, inclusive) = match range_expr {
            Expr::Range { start, end, inclusive } => (start, end, inclusive),
            _ => {
                // If it's not a range, parse it the old way
                let start = Box::new(range_expr);
                let (inclusive, _op) = match self.current() {
                    Token::DotDot => (false, Token::DotDot),
                    Token::DotDotEq => (true, Token::DotDotEq),
                    _ => panic!("Expected .. or ..= in foreach loop"),
                };
                self.advance();
                let end = Box::new(self.expression(0));
                (start, end, inclusive)
            }
        };

        let step = if self.current() == &Token::KwBy {
            self.advance();
            Some(self.expression(0))
        } else {
            None
        };

        self.consume(Token::Colon);
        
        if self.current() == &Token::Newline {
            self.advance();
        }
        
        let body = self.parse_block();

        Stmt::Foreach {
            variable,
            var_ty: Some(var_type),
            range: Expr::Range {
                start,
                end,
                inclusive,
            },
            step,
            body,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(Token::Indent);
        let mut stmts = Vec::new();

        while self.current() != &Token::Dedent && self.current() != &Token::EOF {
            if self.current() == &Token::Newline {
                self.advance();
                continue;
            }
            stmts.push(self.declaration());
        }

        self.consume(Token::Dedent);
        stmts
    }

    fn parse_type(&mut self) -> Type {
        if self.current() == &Token::LBracket {
            self.advance();
            let inner = self.parse_type();
            if self.current() == &Token::Semicolon {
                self.advance();
                if let Token::NumInt(size) = self.current() {
                    let s = *size as usize;
                    self.advance();
                    self.consume(Token::RBracket);
                    return Type::Array(Box::new(inner), s);
                } else {
                    panic!("Expected array size");
                }
            } else {
                self.consume(Token::RBracket);
                return Type::Slice(Box::new(inner));
            }
        }

        let mut val_type = match self.current() {
            Token::Ident(t) => {
                let t = match t.as_str() {
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "bool" => Type::Bool,
                    "string" => Type::Str,
                    "void" => Type::Void,
                    "i8" => Type::I8,
                    "i16" => Type::I16,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "i128" => Type::I128,
                    "u8" => Type::U8,
                    "u16" => Type::U16,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "u128" => Type::U128,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    _ => Type::Custom(t.clone()),
                };
                self.advance();
                t
            }
            _ => panic!("Expected type, found {:?}", self.current()),
        };

        while self.current() == &Token::Question {
            self.advance();
            val_type = Type::Option(Box::new(val_type));
        }
        val_type
    }

    fn binding_power(token: &Token) -> u8 {
        match token {
            Token::KwAs => 1, // Cast
            Token::KwOr => 3,
            Token::KwAnd => 4,
            Token::EqEq | Token::NotEq | Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => 5,
            Token::DotDot | Token::DotDotEq => 6, // Range
            Token::Plus | Token::Minus => 10,
            Token::Star | Token::Slash | Token::Modulo => 20,
            Token::Power => 30,
            Token::LParen | Token::LBracket | Token::Dot => 40, // Call, Index, Member
            Token::DoubleColon => 45,
            _ => 0,
        }
    }

    fn nud(&mut self) -> Expr {
        let token = self.current().clone();
        self.advance();

        match token {
            Token::NumInt(i) => Expr::Literal(LiteralValue::Int(i)),
            Token::NumFloat(f) => Expr::Literal(LiteralValue::Float(f)),
            Token::StrLit(s) => Expr::Literal(LiteralValue::Str(s)),
            Token::BoolLit(b) => Expr::Literal(LiteralValue::Bool(b)),
            Token::KwNone => Expr::Literal(LiteralValue::None),
            Token::LBracket => {
                let mut elements = Vec::new();
                if self.current() != &Token::RBracket {
                    loop {
                        elements.push(self.expression(0));
                        if self.current() == &Token::Comma {
                            self.advance();
                            self.skip_newlines();
                        } else {
                            break;
                        }
                    }
                }
                self.consume(Token::RBracket);
                Expr::Array { elements }
            }

            Token::LParen => {
                let expr = self.expression(0);
                self.consume(Token::RParen);
                expr
            }
            Token::Minus | Token::KwNot => {
                let right = self.expression(35);
                Expr::Unary {
                    op: token,
                    expr: Box::new(right),
                }
            }

            Token::Ident(name) => {
                if self.current() == &Token::LBrace {
                    self.advance(); // eat '{'
                    let mut fields = Vec::new();
                    self.skip_newlines();
                    if self.current() != &Token::RBrace {
                        loop {
                            let f_name = self.consume_ident();
                            self.consume(Token::Colon);
                            let f_val = self.expression(0);
                            fields.push((f_name, f_val));

                            if self.current() == &Token::Comma {
                                self.advance();
                                self.skip_newlines();
                            } else {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RBrace);
                    Expr::StructInit { name, fields }
                } else {
                    Expr::Variable(name)
                }
            }

            _ => panic!("Unexpected token in nud: {:?}", token),
        }
    }

    fn led(&mut self, left: Expr, op: Token) -> Expr {
        let bp = Parser::binding_power(&op);

        match op {
            Token::KwAs => {
                let target_type = self.parse_type();
                Expr::Cast {
                    expr: Box::new(left),
                    target_type,
                }
            }
            Token::LParen => {
                let mut args = Vec::new();
                self.skip_newlines();
                if self.current() != &Token::RParen {
                    loop {
                        args.push(self.expression(0));
                        if self.current() == &Token::Comma {
                            self.advance();
                            self.skip_newlines();
                        } else {
                            break;
                        }
                    }
                }
                self.consume(Token::RParen);
                Expr::Call {
                    func: Box::new(left),
                    args,
                }
            }
            Token::LBracket => {
                let index = self.expression(0);
                self.consume(Token::RBracket);
                Expr::Index {
                    callee: Box::new(left),
                    index: Box::new(index),
                }
            }
            Token::Dot => {
                let name_token = self.current().clone();
                if let Token::Ident(_) = name_token {
                    self.advance();
                    Expr::Get {
                        object: Box::new(left),
                        name: name_token,
                    }
                } else {
                    panic!("Expected field name after dot");
                }
            }
            Token::DotDot | Token::DotDotEq => {
                let right = self.expression(bp);
                Expr::Range {
                    start: Box::new(left),
                    end: Box::new(right),
                    inclusive: op == Token::DotDotEq,
                }
            }
            Token::DoubleColon => {
                let name_token = self.current().clone();
                if let Token::Ident(_) = name_token {
                    self.advance();
                    Expr::Get {
                        object: Box::new(left),
                        name: name_token,
                    }
                } else {
                    panic!("Expected identifier after ::");
                }
            }

            _ => {
                let right_bp = if op == Token::Power { bp - 1 } else { bp };
                let right = self.expression(right_bp);
                Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }

    pub fn expression(&mut self, rbp: u8) -> Expr {
        let mut left = self.nud();

        while rbp < Parser::binding_power(self.current()) {
            let op = self.current().clone();
            self.advance();
            left = self.led(left, op);
        }
        left
    }
}
