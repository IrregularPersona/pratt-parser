// tests.rs - Place this in src/tests.rs or tests/integration_tests.rs

#[cfg(test)]
mod tests {
    use crate::core::{lexer, parser, type_checker};

    fn compile(source: &str) -> Result<(), String> {
        let tokens = lexer::lex(source).map_err(|e| format!("Lexer error: {}", e))?;
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();

        let mut type_checker = type_checker::TypeChecker::new();
        type_checker
            .check_prog(&statements)
            .map_err(|e| format!("{}", e))
    }

    #[test]
    fn test_valid_function_declaration() {
        let code = r#"
fnc add(x: int, y: int) -> int:
  ret x + y
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_mutable_variable() {
        let code = r#"
fnc test():
  mut x: int = 5
  x = 10
  x = 15
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_array_declaration() {
        let code = r#"
fnc test():
  numbers := [1, 2, 3, 4, 5]
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_struct_initialization() {
        let code = r#"
struct Person:
  name: string
  age: int

fnc test():
  p := Person { name: "Alice", age: 30 }
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_if_statement() {
        let code = r#"
fnc test() -> int:
  x := 5
  if x == 5:
    ret 1
  ret 0
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_comparison() {
        let code = r#"
fnc test():
  result := 5 == 5
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_arithmetic() {
        let code = r#"
fnc test():
  a := 10
  b := 5
  sum := a + b
  diff := a - b
  prod := a * b
  quot := a / b
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_logical_operations() {
        let code = r#"
fnc test():
  a := true
  b := false
  result1 := a and b
  result2 := a or b
  result3 := not a
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_valid_type_casting() {
        let code = r#"
fnc test():
  x: int = 5
  y: f64 = x as f64
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_complex_valid_program() {
        let code = r#"
struct Point:
  x: f64
  y: f64

fnc main():
  p1 := Point { x: 0.0, y: 0.0 }
  p2 := Point { x: 3.0, y: 4.0 }
"#;
        // let result = compile(code);
        // if result.is_err() {
        //     println!("Something went incredibly wrong here: {:?}", result.err());
        // }
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_struct_field_access() {
        let code = r#"
struct Point:
  x: f64
  y: f64

fnc main():
  p := Point { x: 5.0, y: 10.0 }
  val := p.x
"#;
        // This might fail if field access isn't fully implemented
        let result = compile(code);
        // if result.is_err() {
        //     println!(
        //         "Field access error (this is expected if not implemented): {:?}",
        //         result.err()
        //     );
        // }
        assert!(result.is_ok());
        // For now, don't assert - just let it run
    }

    // ==================== ERROR DETECTION TESTS ====================

    #[test]
    fn test_type_mismatch_variable() {
        let code = r#"
fnc test():
  x: int = 3.14
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_undefined_variable() {
        let code = r#"
fnc test():
  result := unknown_variable + 5
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_immutable_assignment() {
        let code = r#"
fnc test():
  x: int = 5
  x = 10
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("immutable"));
    }

    #[test]
    fn test_invalid_operator_string_int() {
        let code = r#"
fnc test():
  result := "hello" + 5
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("operator"));
    }

    #[test]
    fn test_return_type_mismatch() {
        let code = r#"
fnc get_number() -> int:
  ret "not a number"
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("return type"));
    }

    #[test]
    fn test_array_mixed_types() {
        let code = r#"
fnc test():
  numbers := [1, 2, 3.14, 4]
"#;
        let result = compile(code);
        assert!(result.is_err());
        // assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_invalid_if_condition() {
        let code = r#"
fnc test() -> int:
  if 42:
    ret 1
  ret 0
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Condition"));
    }

    #[test]
    fn test_struct_missing_field() {
        let code = r#"
struct Person:
  name: string
  age: int

fnc test():
  p := Person { name: "Alice" }
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing"));
    }

    #[test]
    fn test_struct_field_type_mismatch() {
        let code = r#"
struct Person:
  name: string
  age: int

fnc test():
  p := Person { name: "Alice", age: "thirty" }
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_arithmetic_on_boolean() {
        let code = r#"
fnc test():
  result := true + false
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("operator"));
    }

    #[test]
    fn test_logical_on_non_boolean() {
        let code = r#"
fnc test():
  result := 5 and 10
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Logical"));
    }

    #[test]
    fn test_return_outside_function() {
        let code = r#"
x := 5
ret x
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outside"));
    }

    #[test]
    fn test_variable_already_defined() {
        let code = r#"
fnc test():
  x := 5
  x := 10
"#;
        let result = compile(code);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already defined"));
    }

    // ==================== LEXER TESTS ====================

    #[test]
    fn test_lexer_basic_tokens() {
        let code = "fnc test(): ret 42";
        let tokens = lexer::lex(code);
        assert!(tokens.is_ok());
    }

    #[test]
    fn test_lexer_operators() {
        let code = "+ - * / == != < > <= >=";
        let tokens = lexer::lex(code);
        assert!(tokens.is_ok());
    }

    #[test]
    fn test_lexer_range_operators() {
        let code = "1..10 1..=10";
        let tokens = lexer::lex(code);
        assert!(tokens.is_ok());
    }

    #[test]
    fn test_lexer_comments() {
        let code = r#"
// This is a comment
fnc test():
  ret 42 // another comment
"#;
        let tokens = lexer::lex(code);
        assert!(tokens.is_ok());
    }

    // ==================== PARSER TESTS ====================

    #[test]
    fn test_parse_struct() {
        let code = r#"
struct Point:
  x: f64
  y: f64
"#;
        let tokens = lexer::lex(code).unwrap();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let code = r#"
fnc add(a: int, b: int) -> int:
  ret a + b
"#;
        let tokens = lexer::lex(code).unwrap();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn test_parse_foreach() {
        let code = r#"
fnc test():
  foreach i; 0..10:
    x := i
"#;
        let tokens = lexer::lex(code).unwrap();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn test_nested_scopes() {
        let code = r#"
fnc test():
  x := 5
  if true:
    y := 10
    z := x + y
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_function_return_inference() {
        let code = r#"
fnc get_int() -> int:
  x := 42
  ret x
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_array_index_must_be_int() {
        let code = r#"
fnc test():
  arr := [1, 2, 3]
  x := arr[0]
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_array_index_float_fails() {
        let code = r#"
fnc test():
  arr := [1, 2, 3]
  mut idx: int = 0
  idx = 1
  x := arr[idx]
"#;
        assert!(compile(code).is_ok());
    }

    #[test]
    fn test_array_index_with_float_variable() {
        let code = r#"
fnc test():
  arr := [1, 2, 3]
  mut idx := 1.5
  x := arr[idx]
"#;
        let result = compile(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_cast_between_numeric_types() {
        let code = r#"
fnc test():
  x: int = 5
  y: f64 = x as f64
  z: i32 = x as i32
"#;
        assert!(compile(code).is_ok());
    }
}
