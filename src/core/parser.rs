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
            Token::KwFnc => self.func_declaration(),
            Token::KwMut => self.var_declaration(true),
            Token::KwIf => self.if_statement(),
            Token::Ident(_) => {
                if self.peek_token() == &Token::ColonAssign {
                    self.var_declaration(false)
                } else {
                    let expr = self.expression(0);
                    Stmt::Expression(expr)
                }
            }
            Token::KwRet => {
                self.advance();

                if self.current() == &Token::Newline || self.current() == &Token::Dedent {
                    Stmt::Return(None)
                } else {
                    let expr = self.expression(0);
                    Stmt::Return(Some(expr))
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

    fn func_declaration(&mut self) -> Stmt {
        self.consume(Token::KwFnc);
        let name = self.consume_ident();

        self.consume(Token::LParen);
        let mut params = Vec::new();

        self.skip_newlines();

        if self.current() != &Token::RParen {
            loop {
                let p_name = self.consume_ident();
                self.consume(Token::Colon);
                let p_type = self.parse_type();
                params.push((p_name, p_type));

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

        // handle newline before if block starts?
        if self.current() == &Token::Newline {
            self.advance();
        }

        let then_branch = self.parse_block();
        let mut else_branch = None; // wont always exist right

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
            then_branch: then_branch,
            else_branch: else_branch,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(Token::Indent);
        let mut stmts = Vec::new();

        while self.current() != &Token::Dedent && self.current() != &Token::EOF {
            // Handle empty lines within the block
            if self.current() == &Token::Newline {
                self.advance();
                continue;
            }
            stmts.push(self.declaration());
        }

        if self.current() == &Token::Dedent {
            self.advance();
        }

        stmts
    }

    fn parse_type(&mut self) -> Type {
        let mut val_type = match self.current() {
            Token::Ident(t) => {
                let t = match t.as_str() {
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "bool" => Type::Bool,
                    "str" => Type::Str,
                    "i8" => Type::I8,
                    "i16" => Type::I16,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "i128" => Type::I128,
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
            Token::EqEq | Token::NotEq | Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => 5,

            Token::Plus | Token::Minus => 10,
            Token::Star | Token::Slash => 20,
            Token::Power => 30,
            _ => 0,
        }
    }

    fn nud(&mut self) -> Expr {
        let token = self.current().clone();
        self.advance();

        match token {
            Token::NumInt(i) => Expr::Literal(LiteralValue::Int(i)),
            Token::NumFloat(f) => Expr::Literal(LiteralValue::Float(f)),
            Token::Ident(name) => {
                if self.current() == &Token::LParen {
                    self.advance(); // eat '('
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
                    self.skip_newlines();
                    self.consume(Token::RParen);

                    Expr::Call { func: name, args }
                } else {
                    Expr::Variable(name)
                }
            }
            Token::Minus => {
                let right = self.expression(100);
                Expr::Unary {
                    op: Token::Minus,
                    expr: Box::new(right),
                }
            }
            Token::LParen => {
                self.skip_newlines();
                let expr = self.expression(0);
                self.skip_newlines();
                self.consume(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token in nud: {:?}", token),
        }
    }

    fn led(&mut self, left: Expr, op: Token) -> Expr {
        let bp = Parser::binding_power(&op);
        let right_bp = if op == Token::Power { bp - 1 } else { bp };

        let right = self.expression(right_bp);

        Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
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
