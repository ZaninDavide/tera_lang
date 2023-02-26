mod lexer;
use lexer::Lexer;

mod ast;

fn main() {
    let mut lexer = Lexer::new();
    lexer.text = String::from("write(max(9, 1/2) == (3^3)/3, 1/0, 2, 3)");
    lexer.lex();
    println!("{}\n", &lexer.text);
    // lexer.print();
    let ast = ast::ast(&lexer.lexems);
    println!("\n\n{} = {}", lexer.text, ast.eval());
}

// lexer.text = String::from("(-5 + 0.01)|km| + 3alpha ± 2m == sin(4) + 5|m/s| and 1 or 2 <=0< 1");
// lexer.text = String::from("20.32^((5.4 + 2) * (3 - 1)) + 2^2^2");
// lexer.text = String::from("!1 and !!(2*3) and 23?? or (3+3)? and 4?");
// lexer.text = String::from("2-(2 * !-3)");
// lexer.text = String::from("2023.32/(5.4^2.1 * (3 - 1)) - 2^2^2");
// lexer.text = String::from("( (1 and !0) or 0 ) + 2");
// lexer.text = String::from("2 * (3+1) + sin(-1) + 1 + exp(1)");
// lexer.text = String::from("( floor(2 * asin(1) + pow(3, 0)) + (1 > 2) ) / 3");