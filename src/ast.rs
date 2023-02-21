use crate::lexer::Lexem;

#[derive(std::clone::Clone, Debug)]
pub enum Node {
    None,
    Number(f64),
    Operator(String),
    // FunctionCall(String),
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

pub fn ast(lexems: &[Lexem]) -> Tree{
    // TODO
    // • unary operators (also + and - to handle)
    // • add all other binary operators

    let mut level: Vec<Tree> = Vec::new();
    let mut i = 0;
    while i < lexems.len() {
        let tree = match &lexems[i] {
            Lexem::Number(num, dec) => {
                i += 1;
                // NUMBER TO VALUE
                let mut tr: Tree = Node::Number(num.parse().unwrap()).into();
                tr.has_value = true;
                tr
            },
            Lexem::Operator(opname) => {
                i += 1;
                // OPERATOR TO NODE.
                let node;
                match &opname[..] {
                    "!" | "?" => {
                        node = Node::Operator(opname.clone()).into();
                    },
                    "+" | "-" | "*" | "/" | "^" | "==" | "!=" | ">" | "<" | ">=" | "<=" | "and" | "or" | "nand" | "xor" => {
                        // sum and subtraction are considered binary even though they might be unary
                        // this will be handled separately
                        node = Node::Operator(opname.clone()).into();
                    },
                    _ => {
                        panic!("Unknown operator '{}'", opname);
                    }
                }
                node
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
                if to == 0 {
                    panic!("Each opening parenthesis needs a corresponding closing parenthesis");
                }else{
                    ast(&lexems[from+1..to])
                }
            },
            Lexem::RightPar => {
                panic!("Closing parenthesis with no matching opening parenthesis.")
            }
            _ => {
                panic!();
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

    // elevation
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_pow() });

    // prod, div
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_prod() || tree.is_div() });

    // sum, sub
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_sum() || tree.is_sub() });

    // and
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_and() });

    // or
    apply_binary_operation_to_level(&mut level, |tree: &Tree| -> bool { tree.is_or() });

    if level.len() > 1 {
        panic!("The parsing couldn't finish. The reduced level resulted in:\n{:?}", level);
    }

    level.remove(0)
}