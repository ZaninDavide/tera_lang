mod lexer;
use lexer::Lexer;

mod ast;

use std::fs;
use std::time::{Instant};

fn main() {
    let code = fs::read_to_string("sample.tr").expect("Unable to read the source file");

    let iterations = 100;
    let now = Instant::now();

    for _ in 1..=iterations {
        let mut lexer = Lexer::new();
        lexer.text = code.clone();
        lexer.lex();
        // println!("{}\n", &lexer.text);
        // lexer.print();
        let mut evaluator = ast::eval::Evaluator::from_tree(ast::ast(&lexer.lexems));
        evaluator.eval();
        // println!("\n\n{} = {}", lexer.text, evaluator.eval());   
    }

    let elapsed_time = now.elapsed();
    let time = elapsed_time.as_nanos() as f64 / 1e9;
    println!("Running took {}s which is {}s per iteration.", time, time / iterations as f64);


    println!("Finish")
}

// lexer.text = String::from("(-5 + 0.01)|km| + 3alpha ± 2m == sin(4) + 5|m/s| and 1 or 2 <=0< 1");
// lexer.text = String::from("20.32^((5.4 + 2) * (3 - 1)) + 2^2^2");
// lexer.text = String::from("!1 and !!(2*3) and 23?? or (3+3)? and 4?");
// lexer.text = String::from("2-(2 * !-3)");
// lexer.text = String::from("2023.32/(5.4^2.1 * (3 - 1)) - 2^2^2");
// lexer.text = String::from("( (1 and !0) or 0 ) + 2");
// lexer.text = String::from("2 * (3+1) + sin(-1) + 1 + exp(1)");
// lexer.text = String::from("( floor(2 * asin(1) + pow(3, 0)) + (1 > 2) ) / 3");