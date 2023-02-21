mod lexer;
use lexer::Lexer;

mod ast;

fn main() {
    println!("------------ TERA ------------");
    let mut lexer = Lexer::new();
    // lexer.text = String::from("(-5 + 0.01)|km| + 3alpha ± 2m == sin(4) + 5|m/s| and 1 or 2 <=0< 1");
    // lexer.text = String::from("20.32^((5.4 + 2) * (3 - 1)) + 2^2^2");
    // lexer.text = String::from("!1 and !!(2*3) and 23?? or (3+3)? and 4?");
    lexer.text = String::from("2-(2 * !-3)");
    lexer.lex();
    println!("{}", &lexer.text);
    lexer.print();
    println!("\n\n");
    let ast = ast::ast(&lexer.lexems);
    dbg!(ast);
}
