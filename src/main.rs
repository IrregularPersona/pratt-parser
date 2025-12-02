mod core;

#[cfg(test)]
mod unit_tests;

use core::lexer;
use core::parser;
use core::type_checker;

fn main() {
    let source_code = r#"
struct Calculator:
  mut total: f64
  
  fnc new() -> Calculator:
    ret Calculator { total: 0.0 }
  
  fnc add(mut self, value: f64):
    self.total = self.total + value
    print("${self.total}")
  
  fnc subtract(mut self, value: f64):
    self.total = self.total - value
    print("${self.total}")
  
  fnc get_result(self) -> f64:
    ret self.total

fnc calculate_average(numbers: [f64]) -> f64?:
  if numbers.length() == 0:
    ret none
  
  mut sum: f64 = 0.0
  foreach i; 0..numbers.length():
    sum = sum + numbers[i]
  
  ret sum / (numbers.length() as f64)

fnc main():
  print("Calculator Demo")
  
  mut calc := Calculator::new()
  
  calc.add(10.0)
  calc.add(5.0)
  calc.subtract(3.0)
  
  result := calc.get_result()
  print("Final result: ${result}")
  
  // Calculate average
  scores: [f64] = [85.5, 92.0, 78.5, 95.0, 88.0]
  avg: f64? = calculate_average(scores)
  
  if avg != none:
    print("Average score: ${avg}")
  else:
    print("No scores to average")
  
  // Loop demo
  print("Counting:")
  foreach i; 1..=5:
    if i == 3:
      continue
    print(i)
"#;

    println!("=== COMPILATION PIPELINE ===\n");
    println!("--- Source Code ---");
    println!("{}", source_code);
    println!("-------------------\n");

    // 1. Lexing
    println!("Phase 1: Lexing...");
    let tokens = match lexer::lex(source_code) {
        Ok(tokens) => {
            println!(
                "✓ Lexing completed successfully ({} tokens)\n",
                tokens.len()
            );
            tokens
        }
        Err(e) => {
            eprintln!("✗ Lexer Error: {}\n", e);
            return;
        }
    };

    // 2. Parsing
    println!("Phase 2: Parsing...");
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse();
    println!(
        "✓ Parsing completed successfully ({} top-level statements)\n",
        statements.len()
    );

    // 3. Type Checking
    println!("Phase 3: Type Checking...");
    let mut type_checker = type_checker::TypeChecker::new();
    match type_checker.check_prog(&statements) {
        Ok(_) => {
            println!("✓ Type checking passed!\n");

            println!("=== AST (Abstract Syntax Tree) ===");
            for (i, stmt) in statements.iter().enumerate() {
                println!("\nStatement {}:", i + 1);
                println!("{:#?}", stmt);
            }

            println!("\n=== COMPILATION SUCCESSFUL ===");
        }
        Err(e) => {
            println!("✗ Type checking failed!\n");
            eprintln!("{}", e);
        }
    }
}
