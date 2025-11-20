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
