use crate::lexer::Lexem;
use crate::quantity::Unit;

// declare submodule ast::eval
pub mod eval;

#[derive(std::clone::Clone, Debug)]
pub enum Node {
    None,
    Number(f64, String),
    Operator(String),
    Keyword(String),
    Variable(String),
    FunctionCall(String),
    Block,
    UnitBlock(Unit, f64, f64), // unit, factor, shift
    StringBlock(String),
    MatrixBlock(usize, usize), // width, height
    MatrixIndexing(String),
}

#[derive(std::clone::Clone, Debug)]
pub struct Tree {
    node: Node, // it's the content of this node, gives meaning to it's children
    children: Vec<Tree>,
    has_value: bool,
}
impl Tree {
    fn is_none(&self) -> bool {
        match &self.node { Node::None => { !self.has_value }, _ => false }
    }
    fn is_operator(&self) -> bool {
        match &self.node { Node::Operator(_) => { !self.has_value }, _ => false }
    }
    fn is_prod(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "*" }, _ => false }
    }
    fn is_div(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "/" }, _ => false }
    }
    fn is_sum(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "+" }, _ => false }
    }
    fn is_sub(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "-" }, _ => false }
    }
    fn is_pow(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "^" }, _ => false }
    }
    fn is_and(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "and" }, _ => false }
    }
    fn is_or(&self) -> bool {
        match &self.node { Node::Operator(str) => { !self.has_value && str == "or" }, _ => false }
    }
    fn is_bang(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "!" }, _ => false }
    }
    fn is_question(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "?" }, _ => false }
    }
    fn is_greater(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == ">" }, _ => false }
    }
    fn is_greater_equal(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == ">=" }, _ => false }
    }
    fn is_less(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "<" }, _ => false }
    }
    fn is_less_equal(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "<=" }, _ => false }
    }
    fn is_equal_equal(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "==" }, _ => false }
    }
    fn is_assign(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "=" }, _ => false }
    }
    fn is_if(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "if" }, _ => false }
    }
    fn is_else(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "else" }, _ => false }
    }
    fn is_plus_minus(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "pm" }, _ => false }
    }
    fn is_unitblock(&self) -> bool {
        match &self.node { Node::UnitBlock(_, _, _) =>  { !self.has_value }, _ => false }
    }
    fn is_value(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "$" }, _ => false }
    }
    fn is_error(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "&" }, _ => false }
    }
    fn is_while(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "while" }, _ => false }
    }
    fn is_for(&self) -> bool {
        match &self.node { Node::Operator(str) =>  { !self.has_value && str == "for" }, _ => false }
    }
}

impl std::convert::Into<Tree> for Node {
    fn into(self) -> Tree {
        Tree {
            node: self,
            children: Vec::new(),
            has_value: false,
        }
    }
}

fn apply_binary_operation_to_level(level: &mut Vec<Tree>, node_is_wanted_operation: fn(&Tree) -> bool) {
    if level.len() < 3 { return; }
    let mut i = 1;
    while i < level.len() - 1 {
        if node_is_wanted_operation(&level[i]) {
            let right = level.remove(i + 1);
            let left = level.remove(i - 1);
            // now the operator has changed index i -> i - 1
            let mut middle = &mut level[i - 1];
            if left.has_value && right.has_value {
                middle.children.push(left);
                middle.children.push(right);
                middle.has_value = true;
                // we can keep going, we have to keep i the same
                // level = A B C D E F G H I
                //           ^^-^^ -> N
                // level = A N E F G H I
                //           ^^-^^
            }else{
                panic!("A binary operator needs valued expressions to its sides. Found \nleft:\n{:?}\noperator:\n{:?} \nright:\n{:?}", left, middle, right);
            }
        }else{
            i += 1;
        }
    }
}

fn _apply_prefixed_unary_operation_to_level(level: &mut Vec<Tree>, node_is_wanted_operation: fn(&Tree) -> bool) {
    if level.len() < 2 { return; }
    // the unary prefixed operator cannot be the last element
    let mut i: i32 = (level.len() as i32) - 2; 
    // we have to walk backwards if we want notations such as !!value to be easily parsed
    while i >= 0 { // the unary prefixed operator can also be the first element
        if node_is_wanted_operation(&level[i as usize]) {
            let right = level.remove((i+1) as usize);
            // now the operator has not changed index
            let mut middle = &mut level[i as usize];
            if right.has_value {
                middle.children.push(right);
                middle.has_value = true;
                // we can keep going but we have to change i -> i - 1
                // level = A B C D E F G H I
                //           -^^ -> N
                // level = A N D E F G H I
                //         _^^
                i -= 1;
            }else{
                panic!("A unary prefixed operator needs to be followed by a valued expression. Found \noperator:\n{:?} \nright:\n{:?}", middle, right);
            }
        }else{
            i -= 1;
        }
    }
}

fn apply_all_prefixed_unary_operations_to_level(level: &mut Vec<Tree>) {
    if level.len() < 2 { return; }
    // the unary prefixed operator cannot be the last element
    let mut i: i32 = (level.len() as i32) - 2; 
    // we have to walk backwards if we want notations such as !!value to be easily parsed
    while i >= 0 { // the unary prefixed operator can also be the first element
        let none_tree = Tree { node: Node::None, children: Vec::new(), has_value: false};
        let left_ref = level.get((i-1) as usize).unwrap_or(&none_tree);
        if 
            level[i as usize].is_bang() || // not(!) 
            ( ( left_ref.is_operator() || left_ref.is_none() ) && level[i as usize].is_sum() ) || // +(unary)
            ( ( left_ref.is_operator() || left_ref.is_none() ) && level[i as usize].is_sub() ) || // -(unary)
            ( ( left_ref.is_operator() || left_ref.is_none() ) && level[i as usize].is_value() ) || // $(value)
            ( ( left_ref.is_operator() || left_ref.is_none() ) && level[i as usize].is_error() ) // &(error)
        {
            let right = level.remove((i+1) as usize);
            // now the operator has not changed index
            let mut middle = &mut level[i as usize];
            if right.has_value {
                middle.children.push(right);
                middle.has_value = true;
                // we can keep going but we have to change i -> i - 1
                // level = A B C D E F G H I
                //           -^^ -> N
                // level = A N D E F G H I
                //         _^^
                i -= 1;
            }else{
                panic!("A unary prefixed operator needs to be followed by a valued expression. Found \noperator:\n{:?} \nright:\n{:?}", middle, right);
            }
        }else{
            i -= 1;
        }
    }
}

fn apply_postfixed_unary_operation_to_level(level: &mut Vec<Tree>, node_is_wanted_operation: fn(&Tree) -> bool) {
    if level.len() < 2 { return; }
    let mut i = 1;
    while i < level.len() {
        if node_is_wanted_operation(&level[i]) {
            let left = level.remove(i - 1);
            // now the operator has changed index i -> i - 1
            let mut middle = &mut level[i - 1];
            if left.has_value {
                middle.children.push(left);
                middle.has_value = true;
                // we can keep going, we have to keep i the same
                // level = A B C D E F G H I
                //           ^^- -> N
                // level = A N D E F G H I
                //           ^^-
            }else{
                panic!("A unary postfixed operator needs a valued expressions to its left. Found \nleft:\n{:?}\noperator:\n{:?}", left, middle);
            }
        }else{
            i += 1;
        }
    }
}

fn apply_if_statements_to_level(level: &mut Vec<Tree>) {
    if level.len() < 3 { return; }
    let mut i: i32 = (level.len() as i32) - 3; 
    while i >= 0 {
        if level[i as usize].is_if() 
        {
            let right2 = level.remove((i+2) as usize);
            let right1 = level.remove((i+1) as usize);
            let mut middle = &mut level[i as usize];
            if right1.has_value {
                if let Node::Block = right2.node {
                    if right2.has_value {
                        middle.children.push(right1); // condition
                        middle.children.push(right2); // block
                        middle.has_value = true;
                        // we can keep going but we have to change i -> i - 1
                        // level = A B C D E F G H I
                        //           -^^ -> N
                        // level = A N D E F G H I
                        //         _^^
                        i -= 1;
                    }else{
                        panic!("The second element after an 'if' keyword must be a valued block. Found '{:?}' instead, which has no value.", right2);
                    }
                }else{
                    panic!("The second element after an 'if' keyword must be a valued block. Found '{:?}' instead, which is not a block", right2);
                }
            }else{
                panic!("The first element after an 'if' keyword must be a valued expression. Found '{:?}' instead", right1);
            }
        }else{
            i -= 1;
        }
    }
}

fn apply_else_statements_to_level(level: &mut Vec<Tree>) {
    if level.len() < 3 { return; }
    let mut i = level.len() - 2;
    while i >= 1 {
        if level[i].is_else() {
            let right = level.remove(i + 1);
            level.remove(i);
            let left = level.get_mut(i - 1).unwrap();
            if let Node::Operator(str) = &left.node {
                if str == "if" {
                    if let Node::Operator(str2) = &right.node {
                        if str2 == "if" {
                            left.children.push(right);
                            // we can keep going but we have to change i -> i - 2
                            // level = A B C D E F G H I
                            //           ^^-^^ -> B
                            // level = A B D E F G H I
                            //         _^^
                            i = (i as i16 -2).max(0) as usize;    
                        }else{
                            panic!("The 'else' operator needs an if statement or a block to it's right-hand side but '{:?}' was found", right);
                        }
                    }else if let Node::Block = &right.node {
                        left.children.push(right);
                        i  = (i as i16 -2).max(0) as usize;
                    }else{
                        panic!("The 'else' operator needs an if statement or a block to it's right-hand side but '{:?}' was found", right);
                    }
                }else{
                    panic!("The 'else' operator needs an if statement to it's left-hand side but '{:?}' was found", left);
                }
            }else{                    
                panic!("The 'else' operator needs an if statement to it's left-hand side but '{:?}' was found", left);
            }
        }else{
            i -= 1;
        }
    }
}

fn apply_while_statements_to_level(level: &mut Vec<Tree>) {
    if level.len() < 3 { return; }
    let mut i: i32 = (level.len() as i32) - 3; 
    while i >= 0 {
        if level[i as usize].is_while() 
        {
            let right2 = level.remove((i+2) as usize);
            let right1 = level.remove((i+1) as usize);
            let mut middle = &mut level[i as usize];
            if right1.has_value {
                if let Node::Block = right2.node {
                    if right2.has_value {
                        middle.children.push(right1); // condition
                        middle.children.push(right2); // block
                        middle.has_value = true;
                        // we can keep going but we have to change i -> i - 1
                        // level = A B C D E F G H I
                        //           -^^^^ -> N
                        // level = A N E F G H I
                        //         _^^
                        i -= 1;
                    }else{
                        panic!("The second element after a 'while' keyword must be a valued block. Found '{:?}' instead, which has no value.", right2);
                    }
                }else{
                    panic!("The second element after a 'while' keyword must be a valued block. Found '{:?}' instead, which is not a block", right2);
                }
            }else{
                panic!("The first element after a 'while' keyword must be a valued expression. Found '{:?}' instead", right1);
            }
        }else{
            i -= 1;
        }
    }
}
fn apply_for_statements_to_level(level: &mut Vec<Tree>) {
    if level.len() < 5 { return; }
    let mut i: i32 = (level.len() as i32) - 3; 
    while i >= 0 {
        if level[i as usize].is_for() 
        {
            // for x in matrix {}
            // ^^^ ^ ^^ ^^^^^^ ^^
            //  0  1 2     3   4
            let right4 = level.remove((i+4) as usize);    // 4
            let right3 = level.remove((i+3) as usize);    // 3
            let right2 = level.remove((i+2) as usize);    // 2
            let right1 = level.remove((i+1) as usize);    // 1
            let mut middle = &mut level[i as usize]; // 0
            if let Node::Variable(_index_name) = &right1.node {
            if let Node::Keyword(key_name) = &right2.node {
            if key_name == "in" {
            if right3.has_value {
            if let Node::Block = &right4.node {
                if right4.has_value == false { panic!("The second element after the 'in' keyword of a 'for' statement must be a valued block. Found '{:?}' instead, which has no value.", right4)}
                middle.children.push(right1);
                middle.children.push(right3);
                middle.children.push(right4);
                middle.has_value = true;
                // we can keep going but we have to change i -> i - 1
                // level = A B C D E F G H I
                //           _^^^^^^^^ -> N
                // level = A N G H I
                // 
                i -= 1;
            }else{
                panic!("The second element after the 'in' keyword of a 'for' statement must be a valued block. Found '{:?}' instead, which is not a block.", right4);
            }}else{
                panic!("The element after the 'in' keyword of a 'for' statement must be a valued expression. Found {:?} instead.", right3);
            }}else{
                panic!("The second element after a 'for' keyword must be the 'in' keyword. Found {:?} instead, which is not the right keyword.", right2);
            }}else{
                panic!("The second element after a 'for' keyword must be the 'in' keyword. Found {:?} instead, which is not a keyword.", right2);
            }}else{
                panic!("The first element after a 'for' keyword must be a valid variable name. Found {:?} instead.", right1);
            }
        }else{
            i -= 1;
        }
    }
}

pub fn ast(lexems: &[Lexem]) -> Tree{    
    if lexems.len() == 0 {
        return Tree {
            node: Node::None,
            children: Vec::new(),
            has_value: true,
        }
    }

    let mut level: Vec<Tree> = Vec::new();
    let mut i = 0;
    while i < lexems.len() {
        let tree = match &lexems[i] {
            Lexem::Number(num, dec) => {
                i += 1;
                // NUMBER TO VALUE
                let mut tr: Tree = Node::Number(num.parse().unwrap(), dec.clone()).into();
                tr.has_value = true;
                tr
            },
            Lexem::Operator(opname) => {
                i += 1;
                // OPERATOR TO NODE.
                Node::Operator(opname.clone()).into()
            },
            Lexem::Keyword(keyword) => {
                i += 1;
                // OPERATOR TO NODE.
                Node::Keyword(keyword.clone()).into()
            },
            Lexem::LeftPar => {
                // find start and end of this parenthesis section
                let mut parcount = 1;
                let from: usize = i;
                let mut to: usize = 0;
                i += 1;
                'consumerPar: while i < lexems.len() { 
                    if let Lexem::LeftPar = lexems[i] {
                        parcount += 1;
                    }else if let Lexem::RightPar = lexems[i] {
                        parcount -= 1;
                    }
                    if parcount == 0 {
                        to = i;
                        i += 1;
                        break 'consumerPar;
                    }else{
                        i += 1;
                    }
                }
                if parcount != 0 {
                    panic!("Each opening parenthesis needs a corresponding closing parenthesis. Parcount: {parcount}");
                }else{
                    ast(&lexems[from+1..to])
                }
            },
            Lexem::LeftSqBracket => {
                // this is a matrix
                let mut bracketcount = 1;
                let mut elements = Vec::new();
                let mut cur_matrix_width: usize = 0;
                let mut first_row = true;
                let mut matrix_width: usize = 0;
                let mut matrix_height: usize = 0;
                let mut element_from: usize = i;
                let mut last_was_semicolon: bool = false;
                let mut last_was_comma: bool = false;
                i += 1;
                'consumerPar: while i < lexems.len() { 
                    if let Lexem::LeftSqBracket = lexems[i] {
                        bracketcount += 1;
                    }else if let Lexem::RightSqBracket = lexems[i] {
                        bracketcount -= 1;
                    }
                    if bracketcount == 0 {
                        i += 1;
                        break 'consumerPar;
                    }else if bracketcount == 1 {
                        if let Lexem::Comma = lexems[i] {
                            // separator: [1, 2, 3; 4, 5, 6]
                            //              ^
                            elements.push(ast(&lexems[element_from+1..i]));
                            element_from = i;
                            cur_matrix_width += 1;
                            last_was_comma = true;
                            last_was_semicolon = false;
                        }else if let Lexem::SemiColon = lexems[i] {
                            // separator: [1, 2, 3; 4, 5, 6]
                            //                    ^
                            elements.push(ast(&lexems[element_from+1..i]));
                            element_from = i;
                            cur_matrix_width += 1;
                            if !first_row && cur_matrix_width != matrix_width {
                                panic!("The preceding rows of the matrix have width {matrix_width} but this row has width {cur_matrix_width}.");
                            }
                            first_row = false; 
                            matrix_width = cur_matrix_width;
                            cur_matrix_width = 0;
                            matrix_height += 1;
                            last_was_semicolon = true;
                            last_was_comma = false;
                        }else{
                            last_was_semicolon = false;
                            last_was_comma = false;
                        }
                    }else{
                        // we are scanning the inside of a nested matrix
                    }
                    i += 1;
                }

                if !last_was_semicolon {
                    elements.push(ast(&lexems[element_from+1..i-1]));
                    if !last_was_comma {
                        cur_matrix_width += 1;
                    }
                    if !first_row && cur_matrix_width != matrix_width {
                        panic!("The preceding rows of the matrix have width {matrix_width} but this row has width {cur_matrix_width}.");
                    }
                    matrix_width = cur_matrix_width;
                    matrix_height += 1;
                }

                if bracketcount != 0 {
                    panic!("Each square bracket needs a corresponding closing square bracket. Bracketcount: {bracketcount}");
                }else{
                    Tree {
                        node: Node::MatrixBlock(matrix_width, matrix_height),
                        children: elements,
                        has_value: true,
                    }
                }
            },
            Lexem::LeftBracket => {
                // Block
                let mut elements = Vec::new();
                                 
                // Consume the content of brackets
                // and every time we find a semi-colon(;) at a bracket level of 1 we add
                // the ast of that section as element of the block
                i += 1;
                let mut bracketcount = 1;
                let mut sqbracketcount = 0;
                let mut from: usize = i;
                'consumerPar: while i < lexems.len() { 
                    match lexems[i] {
                        Lexem::LeftBracket => { bracketcount += 1; }
                        Lexem::RightBracket => { bracketcount -= 1; }
                        Lexem::LeftSqBracket => { sqbracketcount += 1; }
                        Lexem::RightSqBracket => { sqbracketcount -= 1; }
                        Lexem::SemiColon => {
                            if bracketcount == 1 && sqbracketcount == 0 {
                                // everything until but not including the semicolon
                                elements.push(ast(&lexems[from..i]));
                                // everything from but not including the semicolon
                                from = i + 1;
                            }
                        }
                        _ => (),
                    }
                    i += 1;

                    if bracketcount == 0 {
                        break 'consumerPar;
                    }else if bracketcount < 0 {
                        panic!("A closing bracket was found before a corresponding opening bracket.");
                    }
                    if sqbracketcount < 0 {
                        panic!("A closing square bracket was found before a corresponding opening bracket.");
                    }
                }
                if bracketcount != 0 {
                    panic!("Each opening bracket needs a corresponding closing bracket");
                }else if sqbracketcount != 0 {
                    panic!("Each opening square bracket needs a corresponding closing square bracket");
                }
               
                // we need to push the last argument
                // we subtract one because we don't want the closing bracket
                // println!("Block {}/{}: {:?}", (i as i32) - (from as i32), lexems.len(), &lexems[from..i]);
                // println!("Content: {:?}", elements);
                elements.push(ast(&lexems[from..i-1]));

                
                Tree {
                    node: Node::Block,
                    children: elements,
                    has_value: true,
                }
            },
            Lexem::Identifier(str) => {
                if i == lexems.len() - 1 {
                    // this is for sure a variable
                    i += 1;
                    Tree {
                        node: Node::Variable(str.clone()),
                        children: Vec::new(),
                        has_value: true,
                    }
                }else{
                    match &lexems[i + 1] {
                        Lexem::LeftPar => {
                            let empty: bool;
                            if lexems.len() > i + 2 {
                                if let Lexem::RightPar = &lexems[i + 2] {
                                    // this is an empty function call
                                    empty = true;
                                }else{
                                    empty = false;
                                }
                            }else{
                                panic!("Each opening parenthesis needs a corresponding closing parenthesis");
                            }

                            if empty {
                                i += 3;
                                Tree {
                                    node: Node::FunctionCall(str.clone()),
                                    children: Vec::new(),
                                    has_value: true,
                                }
                            }else{
                                // Function call
                                let mut args = Vec::new();
                                
                                // To determine the function arguments we have to consume the parenthesis
                                // and every time we find a comma(,) at a parenthesis level of +1 we add
                                // the ast of that section as argument to the function call
                                let mut parcount = 1;
                                let mut bracketcount = 0;
                                let mut sqbracketcount = 0;
                                let mut from: usize = i + 1;
                                i += 2;
                                'consumerPar: while i < lexems.len() { 
                                    if let Lexem::LeftPar = lexems[i] {
                                        parcount += 1;
                                    }else if let Lexem::RightPar = lexems[i] {
                                        parcount -= 1;
                                    }else if let Lexem::LeftBracket = lexems[i] {
                                        bracketcount += 1;
                                    }else if let Lexem::RightBracket = lexems[i] {
                                        bracketcount -= 1;
                                    }else if let Lexem::LeftSqBracket = lexems[i] {
                                        sqbracketcount += 1;
                                    }else if let Lexem::RightSqBracket = lexems[i] {
                                        sqbracketcount -= 1;
                                    }else if let Lexem::Comma = lexems[i] {
                                        if parcount == 1 && bracketcount == 0 && sqbracketcount == 0 {
                                            args.push(ast(&lexems[from+1..i]));
                                            from = i;
                                        }
                                    }
                                    if parcount == 0 {
                                        i += 1;
                                        break 'consumerPar;
                                    }else{
                                        i += 1;
                                    }
                                }
                                if parcount != 0 {
                                    panic!("Each opening parenthesis needs a corresponding closing parenthesis");
                                }
                                
                                // we need to push the last argument
                                args.push(ast(&lexems[from+1..i-1]));
                                
                                Tree {
                                    node: Node::FunctionCall(str.clone()),
                                    children: args,
                                    has_value: true,
                                }
                            }
                        }
                        Lexem::LeftSqBracket => {
                            let empty: bool;
                            if lexems.len() > i + 2 {
                                if let Lexem::RightPar = &lexems[i + 2] {
                                    // this is an empty function call
                                    empty = true;
                                }else{
                                    empty = false;
                                }
                            }else{
                                panic!("Each opening parenthesis needs a corresponding closing parenthesis");
                            }

                            if empty {
                                panic!("Trying to index a matrix without specifying any entry. Check if you are trying to create an empty array but put an identifier before the matrix.");
                            }else{
                                // Indexing the matrix
                                let mut args = Vec::new();
                                
                                // To determine the indices we have to consume the square brackets
                                // and every time we find a comma(,) at a parenthesis level of +1 we add
                                // the ast of that section as argument to the function call
                                
                                let mut sqbracketcount = 1;
                                let mut parcount = 0;
                                let mut from: usize = i + 1;
                                i += 2;
                                'consumerPar: while i < lexems.len() { 
                                    if let Lexem::LeftSqBracket = lexems[i] {
                                        sqbracketcount += 1;
                                    }else if let Lexem::RightSqBracket = lexems[i] {
                                        sqbracketcount -= 1;
                                    }else if let Lexem::LeftPar = lexems[i] {
                                        parcount += 1;
                                    }else if let Lexem::RightPar = lexems[i] {
                                        parcount -= 1;
                                    }else if let Lexem::Comma = lexems[i] {
                                        if sqbracketcount == 1 && parcount == 0 {
                                            args.push(ast(&lexems[from+1..i]));
                                            from = i;
                                        }
                                    }
                                    if sqbracketcount == 0 && parcount == 0{
                                        i += 1;
                                        break 'consumerPar;
                                    }else{
                                        i += 1;
                                    }
                                }

                                if sqbracketcount != 0 {
                                    dbg!(lexems);
                                    panic!("Each opening square bracket needs a corresponding closing square bracket");
                                }
                                
                                // we need to push the last argument
                                args.push(ast(&lexems[from+1..i-1]));
                                
                                Tree {
                                    node: Node::MatrixIndexing(str.clone()),
                                    children: args,
                                    has_value: true,
                                }
                            }
                        }
                        _ => {
                            // Variable
                            i += 1;
                            Tree {
                                node: Node::Variable(str.clone()),
                                children: Vec::new(),
                                has_value: true,
                            }
                        }
                    }  
                }
            },
            Lexem::UnitBlock(unit, factor, shift) => {
                i += 1;
                Tree {
                    node: Node::UnitBlock(unit.clone(), factor.clone(), shift.clone()),
                    children: Vec::new(),
                    has_value: false,
                }
            }
            Lexem::StringBlock(str) => {
                i += 1;
                Tree {
                    node: Node::StringBlock(str.clone()),
                    children: Vec::new(),
                    has_value: true,
                }
            }
            Lexem::RightPar => {
                panic!("Closing parenthesis with no matching opening parenthesis.")
            }
            Lexem::RightBracket => {
                panic!("Closing bracket with no matching opening bracket.")
            }
            Lexem::RightSqBracket => {
                panic!("Closing square bracket with no matching opening square bracket.")
            }
            Lexem::Comma => {
                panic!("Comma found outside of any function call or matrix.");
            }
            Lexem::SemiColon => {
                // dbg!(lexems);
                // dbg!(level);
                panic!("Semicolon found outside of any block");
            }
        };
        level.push(tree);
        
    }

    // I don't use this method anymore because it's harder to deal with the special case of +(unary) and -(unary)
    // _apply_prefixed_unary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_bang() });

    // not(!), +(unary), -(unary), $(value), &(error)
    apply_all_prefixed_unary_operations_to_level(&mut level);

    // question(?)
    apply_postfixed_unary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_question() });

    // unit_block(|...|)
    apply_postfixed_unary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_unitblock() });

    // elevation
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_pow() });

    // prod, div
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_prod() || tree.is_div() });

    // pm
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_plus_minus() });

    // sum, sub
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_sum() || tree.is_sub() });

    // eq(==), gt(>), gte(>=), lt(<), lte(<=)
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { 
        tree.is_equal_equal() || tree.is_greater() || tree.is_greater_equal() || 
        tree.is_less() || tree.is_less_equal() 
    });

    // and
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_and() });

    // or
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_or() });

    // if
    apply_if_statements_to_level(&mut level);
    
    // else
    apply_else_statements_to_level(&mut level);

    // while
    apply_while_statements_to_level(&mut level);

    // for
    apply_for_statements_to_level(&mut level);

    // assign(=)
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_assign() });

    if level.len() > 1 {
        panic!("The parsing couldn't finish. The reduced level resulted in:\n{:?}", level);
    }else if level.len() == 0 {
        panic!("The parsing couldn't finish. The reduced level resulted empty");
    }

    level.remove(0)
}