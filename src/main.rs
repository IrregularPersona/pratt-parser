mod core;

use core::lexer::lex;
use core::parser::Parser;
use core::type_checker::TypeChecker;

fn main() {
    let source = "
mut x: int = 42
y := 10

fnc add(a: int, b: int) -> int: 
    ret a + b

fnc max(a: i8, b: i8) -> bool:
    if a > b:
        ret a
    else:
        ret b

fnc testTypeChecker(a: f64, b: str) -> bool:
    if a < b:
        ret false
    else:
        ret false

z := add(x, y)";

    match lex(source) {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();

            println!("-- Generate AST Here --");
            // println!("{:#?}", ast);

            println!("\n-- Running Type Checker --");
            let mut checker = TypeChecker::new();
            match checker.check_prog(&ast) {
                Ok(_) => {
                    println!("Type check passed apparently");
                    // TODO:
                    // codegen here later on lol
                }
                Err(e) => {
                    eprintln!("Type Error: {}", e);
                }
            }
        }

        Err(e) => eprintln!("Lex error: {}", e),
    }
}
