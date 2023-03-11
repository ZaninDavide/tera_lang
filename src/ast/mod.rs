use crate::lexer::Lexem;
use crate::quantity::Unit;

// declare submodule ast::eval
pub mod eval;

#[derive(std::clone::Clone, Debug)]
pub enum Node {
    None,
    Number(f64, String),
    Operator(String),
    Variable(String),
    FunctionCall(String),
    Block,
    UnitBlock(Unit, f64),
    StringBlock(String),
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
        match &self.node { Node::UnitBlock(_, _) =>  { !self.has_value }, _ => false }
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
            ( ( left_ref.is_operator() || left_ref.is_none() ) && level[i as usize].is_sub() ) // -(unary)
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
                    panic!("Each opening parenthesis needs a corresponding closing parenthesis. parcount: {}", parcount);
                }else{
                    ast(&lexems[from+1..to])
                }
            },
            Lexem::LeftBracket => {
                // Block
                let mut elements = Vec::new();
                                 
                // Consume the content of brackets
                // and every time we find a semi-colon(;) at a bracket level of +1 we add
                // the ast of that section as element of the block
                let mut bracketcount = 1;
                let mut from: usize = i;
                let mut to: usize = 0;
                i += 2;
                'consumerPar: while i < lexems.len() { 
                    if let Lexem::LeftBracket = lexems[i] {
                        bracketcount += 1;
                    }else if let Lexem::RightBracket = lexems[i] {
                        bracketcount -= 1;
                    }else if let Lexem::SemiColon = lexems[i] {
                        if bracketcount == 1 {
                            elements.push(ast(&lexems[from+1..i]));
                            from = i;
                        }
                    }
                    if bracketcount == 0 {
                        to = i;
                        i += 1;
                        break 'consumerPar;
                    }else{
                        i += 1;
                    }
                }
                if to == 0 {
                    panic!("Each opening parenthesis needs a corresponding closing parenthesis");
                }
               
                // we need to push the last argument
                elements.push(ast(&lexems[from+1..i-1]));
                
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
                                let mut from: usize = i + 1;
                                let mut to: usize = 0;
                                i += 2;
                                'consumerPar: while i < lexems.len() { 
                                    if let Lexem::LeftPar = lexems[i] {
                                        parcount += 1;
                                    }else if let Lexem::RightPar = lexems[i] {
                                        parcount -= 1;
                                    }else if let Lexem::Comma = lexems[i] {
                                        if parcount == 1 {
                                            args.push(ast(&lexems[from+1..i]));
                                            from = i;
                                        }
                                    }
                                    if parcount == 0 {
                                        to = i;
                                        i += 1;
                                        break 'consumerPar;
                                    }else{
                                        i += 1;
                                    }
                                }
                                if to == 0 {
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
            Lexem::UnitBlock(unit, factor) => {
                i += 1;
                Tree {
                    node: Node::UnitBlock(unit.clone(), factor.clone()),
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
                dbg!(lexems);
                dbg!(level);
                panic!("Closing parenthesis with no matching opening parenthesis.")
            }
            Lexem::RightBracket => {
                panic!("Closing bracket with no matching opening bracket.")
            }
            Lexem::Comma => {
                dbg!(lexems);
                dbg!(level);
                panic!("Comma found outside of any function call");
            }
            Lexem::SemiColon => {
                dbg!(lexems);
                dbg!(level);
                panic!("Semicolon found outside of any block");
            }
        };
        level.push(tree);
        
    }

    // I don't use this method anymore because it's harder to deal with the special case of +(unary) and -(unary)
    // _apply_prefixed_unary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_bang() });

    // not(!), +(unary), -(unary)
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

    // assign(=)
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_assign() });

    if level.len() > 1 {
        panic!("The parsing couldn't finish. The reduced level resulted in:\n{:?}", level);
    }else if level.len() == 0 {
        panic!("The parsing couldn't finish. The reduced level resulted empty");
    }

    level.remove(0)
}