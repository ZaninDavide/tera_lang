use unicode_segmentation::UnicodeSegmentation;
use crate::quantity::Unit;

#[derive(Debug)]
pub enum Lexem {
    LeftPar,
    RightPar,
    LeftBracket,
    RightBracket,
    Identifier(String),
    Number(String, String), // (representation, decorator)
    Operator(String),
    Comma,
    SemiColon,
    UnitBlock(Unit, f64),
    StringBlock(String),
}
impl std::fmt::Display for Lexem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lexem::LeftPar => write!(f, "LP("),
            Lexem::RightPar => write!(f, "RP)"),
            Lexem::LeftBracket => write!(f, "LB{{"),
            Lexem::RightBracket => write!(f, "RB}}"),
            Lexem::Identifier(s) => write!(f, "ID{{{}}}", s),
            Lexem::Number(s, d) => write!(f, "NUM{{{}, \"{}\"}}", s, d),
            Lexem::Operator(s) => write!(f, "OP{{{}}}", s),
            Lexem::Comma => write!(f, "COMMA,"),
            Lexem::SemiColon => write!(f, "SC;"),
            Lexem::UnitBlock(u, n) => write!(f, "UNIT{{{u},{n}}}"),
            Lexem::StringBlock(s) => write!(f, "STRING{{{s}}}"),
        }
    }
}

pub struct Lexer {
    pub text: String,
    pub lexems: Vec<Lexem>,
}
impl Lexer {
    pub fn new() -> Lexer { Lexer{
        text: String::new(), lexems: vec![],
    }}

    pub fn lex(&mut self) {
        let text_terminated = format!("{}\0", &self.text);
        let chars = text_terminated.graphemes(true).collect::<Vec<&str>>();
        let n = chars.len();
        let mut i = 0;

        let string_operators = vec![
            "or", "and", "nand", "xor", "if", "else", "pm"
        ];

        'main: while i < n {
            // go through each character one by one
            let mut char = chars[i];
            if char == "\0" {
                // END OF FILE
                // the string is guaranteed to end with \0,
                // this makes the algorithm cleaner because
                // consumers will always stop when finding \0.
                break 'main;
            }else if char == "(" {
                // LEFT PARENTHESIS
                self.lexems.push(Lexem::LeftPar);
                i += 1;
            }else if char == ")" {
                // RIGHT PARENTHESIS
                self.lexems.push(Lexem::RightPar);
                i += 1;
            }else if char == "{" {
                // LEFT BRACKET
                self.lexems.push(Lexem::LeftBracket);
                i += 1;
            }else if char == "}" {
                // RIGHT BRACKET
                self.lexems.push(Lexem::RightBracket);
                i += 1;
            }else if char == "|" {
                i += 1;
                let mut found_end = false;
                let mut unit_block_str: String = String::new(); 
                'consumerUnitBlock: while i < chars.len() {
                    if chars[i] == "|" { 
                        found_end = true;
                        i += 1;
                        break 'consumerUnitBlock; 
                    }else if chars[i] != " " && chars[i] != "\t" && chars[i] != "\n"{
                        unit_block_str.push_str(chars[i]);
                    }
                    i += 1;
                }
                if found_end {
                    let slash_split: Vec<&str> = unit_block_str.split('/').collect();
                    let mut prod = "";
                    let mut div= "";
                    match slash_split.len() {
                        1 => {
                            prod = slash_split[0];
                        }
                        2 => {
                            prod = slash_split[0];
                            div = slash_split[1];
                        }
                        _ => {
                            panic!("Couldn't parse the unit block '{}' because more than one '/' where found", unit_block_str);
                        }
                    }

                    let mut unit: Unit = Unit::unitless();
                    let mut factor: f64 = 1.0;
                    for x in prod.split('.').map(|t| {
                        if t == "" { return (Unit::unitless(), 1.0, 0.0); }
                        crate::quantity::Unit::parse_single_unit(t)
                    }) {
                        if x.2 != 0.0 { panic!("Unit blocks cannot contain shifted units but '{unit_block_str}' was found.") }
                        unit = unit * x.0;
                        factor *= x.1;
                    }
                    for x in div.split('.').map(|t| {
                        if t == "" { return (Unit::unitless(), 1.0, 0.0); }
                        crate::quantity::Unit::parse_single_unit(t)
                    }) {
                        if x.2 != 0.0 { panic!("Unit blocks cannot contain shifted units but '{unit_block_str}' was found.") }
                        unit = unit / x.0;
                        factor /= x.1;
                    }
                    self.lexems.push(Lexem::UnitBlock(unit, factor));
                }else{
                    panic!("Opening '|' is missing a matching closing '|'.");
                }
            }else if char == "\"" {
                // String block
                i += 1;
                let mut found_end = false;
                let mut str_block: String = String::new(); 
                'consumerStringBlock: while i < chars.len() {
                    if chars[i] == "\"" { 
                        found_end = true;
                        i += 1;
                        break 'consumerStringBlock; 
                    }else if chars[i] == "\\" {
                        match chars[i + 1] {
                            "n" => {
                                i += 1; str_block.push_str("\n");
                            }
                            "t" => {
                                i += 1; str_block.push_str("\t");
                            }
                            "\"" => {
                                i += 1; str_block.push_str("\"");
                            }
                            // "\\" is done in evaluation
                            _ => { str_block.push_str("\\"); }
                        }
                    }else{
                        str_block.push_str(chars[i]);
                    }
                    i += 1;
                }
                if found_end {
                    self.lexems.push(Lexem::StringBlock(str_block));
                }else{
                    panic!("Opening '\"' is missing a matching closing '\"'.");
                }
            }else if char == "," {
                // COMMA
                self.lexems.push(Lexem::Comma);
                i += 1;
            }else if char == ";" {
                // SEMI-COLON
                self.lexems.push(Lexem::SemiColon);
                i += 1;
            }else if "+-*/^?".find(char).is_some() {
                // PLUS, MINUS, TIMES, DIVIDE, POWER, QUESTION
                self.lexems.push(Lexem::Operator(String::from(char)));
                i += 1;
            }else if char == " " || char == "\t" || char == "\n" {
                // SPACES
                i += 1;
            }else if char == "=" {
                // EQUALS EQUALS
                if chars[i + 1] == "=" {
                    self.lexems.push(Lexem::Operator(String::from("==")));
                    i += 2;
                }else{
                    self.lexems.push(Lexem::Operator(String::from("=")));
                    i += 1;
                }
            }else if char == "!" {
                // NOT EQUAL
                if chars[i + 1] == "=" {
                    self.lexems.push(Lexem::Operator(String::from("!=")));
                    i += 2;
                }else{
                    self.lexems.push(Lexem::Operator(String::from(char)));
                    i += 1;
                }
            }else if char == ">" {
                if chars[i + 1] == "=" {
                    // GREATER THEN OR EQUAL TO
                    self.lexems.push(Lexem::Operator(String::from(">=")));
                    i += 2;
                }else{
                    // GREATER THAN
                    self.lexems.push(Lexem::Operator(String::from(">")));
                    i += 1;
                }
            }else if char == "<" {
                if chars[i + 1] == "=" {
                    // LESS THEN OR EQUAL TO
                    self.lexems.push(Lexem::Operator(String::from("<=")));
                    i += 2;
                }else{
                    // LESS THAN
                    self.lexems.push(Lexem::Operator(String::from("<")));
                    i += 1;
                }
            }else if char == "±" {
                // PLUS MINUS
                self.lexems.push(Lexem::Operator(String::from("pm")));
                i += 1;
            }else if "1234567890.".find(char).is_some() {
                // NUMBER
                let mut number = String::from(char);
                let mut decorator = String::new();
                let mut inside_decorator = false;
                let mut j = i + 1;
                // consume all letters after these
                'consumerN: while j < n {
                    char = chars[j];
                    if !inside_decorator && "1234567890.".find(char).is_some() {
                        // this char is part of the number
                        number.push_str(char);
                        j += 1;
                    }else if !inside_decorator && "'".find(char).is_some() {
                        // this character can be skipped example: 1'000 == 1000
                        j += 1;
                    }else if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ°".find(char).is_some() {
                        // this char is part of the number's decorator
                        inside_decorator = true;
                        decorator.push_str(char);
                        j += 1;
                    }else if inside_decorator && "0123456789".find(char).is_some() {
                        // this char is part of the number's decorator
                        decorator.push_str(char);
                        j += 1;
                    } else{
                        // the number is finished
                        self.lexems.push(Lexem::Number(number, decorator));
                        break 'consumerN;
                    }
                }
                i = j;
            }else if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".find(char).is_some() {
                // IDENTIFIER
                let mut word = String::from(char);
                let mut j = i + 1;
                // consume all letters after these
                'consumerL: while j < n {
                    char = chars[j];
                    if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_".find(char).is_some() {
                        // this char is part of the identifier name
                        word.push_str(char);
                        j += 1;
                    }else{
                        // the identifier is finished
                        if string_operators.contains(&&word[..]) {
                            self.lexems.push(Lexem::Operator(word));
                        }else{
                            self.lexems.push(Lexem::Identifier(word));
                        }
                        i = j;
                        break 'consumerL;
                    }
                }
            }else{
                panic!("Syntax error at character number {}: '{}'", i, char);
            }
        }
    }

    pub fn print(&self) {
        for lref in self.lexems.iter() {
            print!("{} ", lref);
        }
    }
}
