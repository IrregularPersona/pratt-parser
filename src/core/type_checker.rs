use crate::core::ast::{Expr, Stmt, Type};
use crate::core::lexer::Token;
use std::collections::HashMap;

struct TypeEnvironment {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnvironment {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, val_type: Type) -> Result<(), String> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!(
                    "Variable '{}' already defined in this scope!",
                    name
                ));
            }

            scope.insert(name, val_type);
            Ok(())
        } else {
            Err("Internal Compiler Error: No scope found?".to_string())
        }
    }

    fn resolve(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ.clone());
            }
        }

        None
    }
}

pub struct TypeChecker {
    env: TypeEnvironment,
    current_fn_ret_type: Option<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnvironment::new(),
            current_fn_ret_type: None,
        }
    }

    pub fn check_prog(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.check_stmt(stmt)?;
        }

        Ok(())
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::VarDecl {
                name,
                var_ty,
                value,
                is_mut: _,
            } => {
                let val_type = self.check_expr(value)?;

                let final_type = if let Some(explicit_typ) = var_ty {
                    if !self.types_match(explicit_typ, &val_type) {
                        return Err(format!(
                            "Type Mismatched: Variable '{}' declared as {:?} but assigned as {:?}",
                            name, explicit_typ, val_type
                        ));
                    }

                    explicit_typ.clone()
                } else {
                    val_type
                };

                self.env.define(name.clone(), final_type)?;
            }

            Stmt::Function {
                name,
                params,
                ret_type,
                body,
            } => {
                // enter func
                self.env.enter_scope();

                // get params
                for (p_name, p_type) in params {
                    self.env.define(p_name.clone(), p_type.clone())?;
                }

                let prev_ret = self.current_fn_ret_type.clone();
                self.current_fn_ret_type = Some(ret_type.clone());

                for s in body {
                    self.check_stmt(s)?;
                }

                // cleanup gng
                self.current_fn_ret_type = prev_ret;
                self.env.exit_scope();
            }
            Stmt::Expression(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_typ = self.check_expr(cond)?;
                if cond_typ != Type::Bool {
                    return Err(format!("If condition must be Bool, found {:?}", cond_typ));

                    self.env.enter_scope();
                    for s in then_branch {
                        self.check_stmt(s)?;
                    }
                    self.env.exit_scope();

                    if let Some(else_stmt) = else_branch {
                        self.env.enter_scope();
                        for s in else_stmt {
                            self.check_stmt(s)?;
                        }
                        self.env.exit_scope();
                    }
                }
            }
            Stmt::Return(opt_expr) => {
                let ret_typ = match opt_expr {
                    Some(e) => self.check_expr(e)?,
                    None => Type::Void,
                };

                if let Some(expected) = &self.current_fn_ret_type {
                    if !self.types_match(expected, &ret_typ) {
                        return Err(format!(
                            "Expected return type {:?}, got {:?}",
                            expected, ret_typ
                        ));
                    }
                } else {
                    return Err("Return statement outside of function".to_string());
                }
            }
        }

        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Literal(lit) => match lit {
                crate::core::ast::LiteralValue::Int(_) => Ok(Type::Int),
                crate::core::ast::LiteralValue::Float(_) => Ok(Type::Float),
                crate::core::ast::LiteralValue::Str(_) => Ok(Type::Str),
                crate::core::ast::LiteralValue::None => Ok(Type::NoneType),
            },

            Expr::Variable(name) => self
                .env
                .resolve(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),

            Expr::Binary { op, left, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;
                match op {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                        if left_ty == Type::Int && right_ty == Type::Int {
                            Ok(Type::Int)
                        } else if left_ty == Type::Float && right_ty == Type::Float {
                            Ok(Type::Float)
                        } else {
                            Err(format!(
                                "Cannot apply operation {:?} to {:?} and {:?}",
                                op, left_ty, right_ty
                            ))
                        }
                    }

                    Token::EqEq | Token::NotEq | Token::Gt | Token::Lt => {
                        if !self.types_match(&left_ty, &right_ty) {
                            Err(format!(
                                "Cannot compare different types {:?} and {:?}",
                                left_ty, right_ty
                            ))
                        } else {
                            Ok(Type::Bool)
                        }
                    }

                    _ => Err(format!("Unknown binary operator {:?}", op)),
                }
            }
            _ => Ok(Type::Int),
        }
    }
    fn types_match(&self, t1: &Type, t2: &Type) -> bool {
        t1 == t2
    }
}
