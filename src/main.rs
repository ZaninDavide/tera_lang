mod lexer;
use lexer::Lexer;

mod ast;
mod quantity;

use std::fs;
use std::time::{Instant};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code;
    if args.len() > 1 {
        code = fs::read_to_string(&(args[1])[..]);
    }else{
        panic!("Source code path missing");
    }
    let code = code.expect("Unable to read the source file");

    let mut lexer = Lexer::new();
    lexer.text = code.clone();
    lexer.lex();

    let abst = ast::ast(&lexer.lexems);
    let mut evaluator = ast::eval::Evaluator::from_tree(abst);

    let iterations = 1;
    let now = Instant::now();
    
    for _ in 1..=iterations {
        let _res = evaluator.eval();
        // println!("\n\n{} = {}", lexer.text, res);   
    }

    let elapsed_time = now.elapsed();
    let time = elapsed_time.as_nanos() as f64 / 1e3;
    println!("Running took {}µs which is {}µs per iteration.", time, time / iterations as f64);

    /*
    let x = Quantity{re: 1.0, im: 0.0, vre: 0.1*0.1, vim: 0.0, unit: quantity::Unit::unitless()};
    let y = Quantity{re: 2.0, im: 1.0, vre: 0.1*0.1, vim: 0.0, unit: quantity::Unit::unitless()};
    let mut z = x * y;
    z.unit.metre = 2;
    z.unit.second = -2;
    println!("{}", z);

    let theta = Quantity{re: 3.14/4.0, im: 0.0, vre: 3.14/40.0, vim: 0.0,unit: quantity::Unit::unitless()};
    z = theta.sin();
    z.unit.metre = 1;
    println!("{}", z);
    */
}

// lexer.text = String::from("(-5 + 0.01)|km| + 3alpha ± 2m == sin(4) + 5|m/s| and 1 or 2 <=0< 1");
// lexer.text = String::from("20.32^((5.4 + 2) * (3 - 1)) + 2^2^2");
// lexer.text = String::from("!1 and !!(2*3) and 23?? or (3+3)? and 4?");
// lexer.text = String::from("2-(2 * !-3)");
// lexer.text = String::from("2023.32/(5.4^2.1 * (3 - 1)) - 2^2^2");
// lexer.text = String::from("( (1 and !0) or 0 ) + 2");
// lexer.text = String::from("2 * (3+1) + sin(-1) + 1 + exp(1)");
// lexer.text = String::from("( floor(2 * asin(1) + pow(3, 0)) + (1 > 2) ) / 3");