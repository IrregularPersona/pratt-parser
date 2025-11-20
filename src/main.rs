mod core;

use crate::core::arithmetic_parser::{Parser, lex};

fn main() {
    let inputs = vec![
        "min(10, 2)",         // 2
        "max(1, 5, 20, 3)",   // 20
        "max(1, 2.5)",        // 2.5
        "pow(2, 3)",          // 8
        "sin(0)",             // 0
        "min(sqrt(16), 100)", // 4
    ];

    for input in inputs {
        let tokens = lex(input).unwrap();
        let mut parser = Parser::new(tokens);
        println!("{} = {}", input, parser.expression(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::arithmetic_parser::Number;

    fn solve(input: &str) -> Number {
        let tokens = lex(input).expect("Lexing failed");
        let mut parser = Parser::new(tokens);
        parser.expression(0)
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(solve("1 + 2"), Number::Int(3));
        assert_eq!(solve("10 - 5"), Number::Int(5));
        assert_eq!(solve("2 * 3"), Number::Int(6));
        assert_eq!(solve("10 / 2"), Number::Float(5.0));
    }

    #[test]
    fn test_floats() {
        assert_eq!(solve("1.5 + 2.5"), Number::Float(4.0));
        assert_eq!(solve("10 * 0.5"), Number::Float(5.0));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(solve("2 + 3 * 4"), Number::Int(14));
        assert_eq!(solve("(2 + 3) * 4"), Number::Int(20));
    }

    #[test]
    fn test_new_operators() {
        assert_eq!(solve("10 % 3"), Number::Int(1));
        assert_eq!(solve("10.5 % 3"), Number::Float(1.5));

        assert_eq!(solve("2 ^ 3 ^ 2"), Number::Float(512.0));
    }

    #[test]
    fn test_functions() {
        assert_eq!(solve("min(1, 10)"), Number::Int(1)); // apparently this returns an Number::Float() instead
        assert_eq!(solve("max(5, 2, 8)"), Number::Int(8)); // so does this
        assert_eq!(solve("sqrt(16)"), Number::Float(4.0));
        assert_eq!(solve("abs(-50)"), Number::Float(50.0));
    }

    #[test]
    fn test_nested_logic() {
        assert_eq!(solve("max(10, sqrt(100) + 5)"), Number::Int(15));
        assert_eq!(solve("min(10 % 3, 5)"), Number::Int(1));
    }
}
