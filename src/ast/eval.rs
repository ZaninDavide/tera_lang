use std::{collections::HashMap};

use crate::ast::{Node, Tree};

#[derive(Clone, Debug)]
pub enum RValue {
    Void,
    Number(f64),
}
impl RValue {
    fn get_type(&self) -> &'static str {
        match &self {
            RValue::Void => "Void",
            RValue::Number(_) => "Number",
        }
    }
}
impl std::fmt::Display for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RValue::Void => write!(f, "Void"),
            RValue::Number(n) => write!(f, "{n}"),
        }
    }
}

pub struct Evaluator {
    tree: Tree,
    vars: HashMap<String, RValue>
}
impl Evaluator {
    pub fn from_tree(tree: Tree) -> Self {
        Evaluator {
            tree: tree, vars: HashMap::new()
        }
    }
    pub fn eval(&mut self) -> RValue {
        self.tree.eval(&mut self.vars)
    }
    pub fn set_var(&mut self, varname: String, value: RValue) {
        self.vars.insert(varname, value);
    }
}

macro_rules! eval_number_unary_operator { 
    ($name:literal, $children:expr, $vars:expr, $n0:ident, $body:expr) => {
        { 
            if $children.len() == 1 {
                let childval0: RValue = $children[0].eval($vars);
                match childval0 {
                    RValue::Number($n0) => {
                        return RValue::Number($body);
                    }
                    _ => {
                        panic!("The '{}' operator operates on values of type 'Number' but an element of type '{}' was found.", $name, childval0.get_type());
                    }
                }
            }else{
                panic!("The '{}' operator is unary only, but {} parameters were found.", $name, $children.len());
            }
        }
    } 
}

macro_rules! eval_number_binary_operator { 
    ($name:literal, $children:expr, $vars:expr, $n0:ident, $n1:ident, $body:expr) => {
        { 
            if $children.len() == 2 {
                let childval0: RValue = $children[0].eval($vars);
                let childval1: RValue = $children[1].eval($vars);
                match childval0 {
                    RValue::Number($n0) => {
                        match childval1 {
                            RValue::Number($n1) => {
                                return RValue::Number($body);
                            }
                            _ => {
                                panic!("The '{}' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", $name, childval1.get_type());
                            }
                        }
                    }
                    _ => {
                        panic!("The '{}' operator operates on values of type 'Number' but an element of type '{}' was found on the left-hand side.", $name, childval0.get_type());
                    }
                }
            }else{
                panic!("The '{}' operator is binary only but a number of {} children were found.", $name, $children.len());
            }
        }
    } 
}

macro_rules! eval_number_unary_function { 
    ($name:literal, $children:expr, $vars:expr, $n0:ident, $body:expr) => {
        { 
            if $children.len() == 1 {
                let childval0: RValue = $children[0].eval($vars);
                match childval0 {
                    RValue::Number($n0) => {
                        return RValue::Number($body);
                    }
                    _ => {
                        panic!("The '{}' function takes on value of type 'Number' but an element of type '{}' was found.", $name, childval0.get_type());
                    }
                }
            }else{
                panic!("The '{}' function takes one parameter, but {} parameters were found.", $name, $children.len());
            }
        }
    } 
}

macro_rules! eval_number_binary_function { 
    ($name:literal, $children:expr, $vars:expr, $n0:ident, $n1:ident, $body:expr) => {
        { 
            if $children.len() == 2 {
                let childval0: RValue = $children[0].eval($vars);
                let childval1: RValue = $children[1].eval($vars);
                match childval0 {
                    RValue::Number($n0) => {
                        match childval1 {
                            RValue::Number($n1) => {
                                return RValue::Number($body);
                            }
                            _ => {
                                panic!("The '{}' function takes two values of type 'Number' but an element of type '{}' was found as second parameter.", $name, childval1.get_type());
                            }
                        }
                    }
                    _ => {
                        panic!("The '{}' function takes two values of type 'Number' but an element of type '{}' was found as first parameter.", $name, childval0.get_type());
                    }
                }
            }else{
                panic!("The '{}' function takes two parameters, but {} parameters were found.", $name, $children.len());
            }
        }
    } 
}

impl Tree {
    fn eval(&self, vars: &mut HashMap<String, RValue>) -> RValue {
        match &self.node {
            Node::Number(val, dec) => {
                // TODO: number to value
                return RValue::Number(*val);
            }
            Node::Operator(opname) => {
                let length = self.children.len();
                match &opname[..] {
                    "!" => {
                        eval_number_unary_operator!("!", self.children, vars, n0, if n0 == 0.0 {1.0} else {0.0})
                    }
                    "?" => {
                        eval_number_unary_operator!("!", self.children, vars, n0, if n0 != 0.0 {1.0} else {0.0})
                    }
                    "+" => {
                        if length == 1 {
                            let childval = self.children[0].eval(vars);
                            match childval {
                                RValue::Number(_) => {
                                    return childval;
                                }
                                _ => {
                                    panic!("The unary '+' operator operates on values of type 'Number' but an element of type '{}' was found.", childval.get_type());
                                }
                            }
                        }else if length == 2 {
                            let childval0 = self.children[0].eval(vars);
                            let childval1 = self.children[1].eval(vars);
                            match childval0 {
                                RValue::Number(n0) => {
                                    match childval1 {
                                        RValue::Number(n1) => {
                                            return RValue::Number(n0 + n1);
                                        }
                                        _ => {
                                            panic!("The binary '+' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", childval1.get_type());
                                        }
                                    }
                                }
                                _ => {
                                    panic!("The binary '+' operator operates on values of type 'Number' but an element of type '{}' was found on the left-hand side.", childval0.get_type());
                                }
                            }
                        }else{
                            panic!("The '+' operator can be either unary or binary but a number of {} children were found.", length)
                        }
                    }
                    "-" => {
                        if length == 1 {
                            let childval = self.children[0].eval(vars);
                            match childval {
                                RValue::Number(n) => {
                                    return RValue::Number(-n);
                                }
                                _ => {
                                    panic!("The unary '-' operator operates on values of type 'Number' but an element of type '{}' was found.", childval.get_type());
                                }
                            }
                        }else if length == 2 {
                            let childval0 = self.children[0].eval(vars);
                            let childval1 = self.children[1].eval(vars);
                            match childval0 {
                                RValue::Number(n0) => {
                                    match childval1 {
                                        RValue::Number(n1) => {
                                            return RValue::Number(n0 - n1);
                                        }
                                        _ => {
                                            panic!("The binary '-' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", childval0.get_type());
                                        }
                                    }
                                }
                                _ => {
                                    panic!("The binary '-' operator operates on values of type 'Number' but an element of type '{}' was found on the left-hand side.", childval0.get_type());
                                }
                            }
                        }else{
                            panic!("The '-' operator can be either unary or binary but a number of {} children were found.", length)
                        }
                    }
                    "^" => {
                        eval_number_binary_operator!("^", self.children, vars, n0, n1, n0.powf(n1))
                    }
                    "*" => {
                        eval_number_binary_operator!("*", self.children, vars, n0, n1, n0 * n1)
                    }
                    "/" => {
                        eval_number_binary_operator!("/", self.children, vars, n0, n1, n0 / n1)
                    }
                    "==" => {
                        eval_number_binary_operator!("==", self.children, vars, n0, n1, if n0 == n1 { 1.0 } else { 0.0 } )
                    }
                    ">" => {
                        eval_number_binary_operator!(">", self.children, vars, n0, n1, if n0 > n1 { 1.0 } else { 0.0 } )
                    }
                    ">=" => {
                        eval_number_binary_operator!(">=", self.children, vars, n0, n1, if n0 >= n1 { 1.0 } else { 0.0 } )
                    }
                    "<" => {
                        eval_number_binary_operator!("<", self.children, vars, n0, n1, if n0 < n1 { 1.0 } else { 0.0 } )
                    }
                    "<=" => {
                        eval_number_binary_operator!("<=", self.children, vars, n0, n1, if n0 <= n1 { 1.0 } else { 0.0 } )
                    }
                    "and" => {
                        eval_number_binary_operator!("and", self.children, vars, n0, n1, if n0 != 0.0 && n1 != 0.0 {1.0} else {0.0} )
                    }
                    "or" => {
                        eval_number_binary_operator!("or", self.children, vars, n0, n1, if n0 != 0.0 || n1 != 0.0 {1.0} else {0.0} )
                    }
                    "=" => {
                        if self.children.len() == 2 {
                            let child0: &Node = &self.children[0].node;
                            if let Node::Variable(varname) = child0 {
                                // TODO: what if they create a variable with the same name of a function?
                                let childvar1 = self.children[1].eval(vars);
                                vars.insert(varname.clone(), childvar1);
                                RValue::Void
                            }else{
                                panic!("The '=' operator expects a variable name on the left-hand side.");
                            }
                        }else{
                            panic!("The '=' operator is binary only but a number of {} children were found.", self.children.len());
                        }
                    }
                    "if" => {
                        if self.children.len() == 2 {
                            // IF 
                            if let RValue::Number(condition) = &self.children[0].eval(vars) {
                                if *condition != 0.0 {
                                    self.children[1].eval(vars)
                                }else{
                                    RValue::Void
                                }
                            }else{
                                RValue::Void
                            }
                        }else if self.children.len() == 3 {
                            // IF ELSE
                            if let RValue::Number(condition) = &self.children[0].eval(vars) {
                                if *condition != 0.0 {
                                    self.children[1].eval(vars)
                                }else{
                                    self.children[2].eval(vars)
                                }
                            }else{
                                self.children[2].eval(vars)
                            }
                        }else{
                            panic!("The 'if' operator is a prefixed binary or ternary operator but a number of {} children were found.", self.children.len());
                        }
                    }
                    _ => {
                        panic!("Unknown operator '{}'", opname);
                    }
                }
            } 
            Node::FunctionCall(fname) => {
                match &fname[..] {
                    // ONE PARAMETER FUNCTIONS
                    "sin" => {
                        eval_number_unary_function!("sin", self.children, vars, n, n.sin())
                    }
                    "cos" => {
                        eval_number_unary_function!("cos", self.children, vars, n, n.cos())
                    }
                    // TWO PARAMETERS FUNCTIONS
                    "max" => {
                        eval_number_binary_function!("max", self.children, vars, n0, n1, n0.max(n1))
                    }
                    "min" => {
                        eval_number_binary_function!("max", self.children, vars, n0, n1, n0.min(n1))
                    }
                    // VOID FUNCTIONS
                    "write" => {
                        if self.children.len() > 0 {
                            for v in self.children.iter() {
                                print!("{} ", v.eval(vars));
                            }
                            RValue::Void
                        }else{                        
                            panic!("The 'write' function takes one or more parameters but no parameters were found.")
                        }
                    }
                    "writeln" => {
                        if self.children.len() > 0 {
                            for v in self.children.iter() {
                                print!("{} ", v.eval(vars));
                            }
                            print!("\n");
                            RValue::Void
                        }else{                        
                            panic!("The 'writeln' function takes one or more parameters but no parameters were found.")
                        }
                    }
                    _ => {
                        panic!("Unknown function called '{}'", &fname);
                    }
                }
            }
            Node::Variable(varname) => {
                if let Some(rvalue) = vars.get(varname) {
                    (*rvalue).clone()
                }else{
                    // TODO: suggest a variable with a similar name
                    panic!("Unable to give value to:\n {:?}", &self);
                }
            }
            Node::Block => {
                    let l = self.children.len();
                    let mut res = RValue::Void;
                    for i in 0..l {
                        let value = self.children[i].eval(vars);
                        if i == l - 1 {
                            res = value;
                        }
                    }
                    res
            }
            Node::None => {
                RValue::Void
            }
        }
    }
}