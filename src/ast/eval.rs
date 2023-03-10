use std::{collections::HashMap};

use crate::ast::{Node, Tree};
use crate::quantity::{Quantity, Unit};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub enum RValue {
    Void,
    Number(Quantity),
    String(String),
}
impl RValue {
    fn get_type(&self) -> &'static str {
        match &self {
            RValue::Void => "Void",
            RValue::Number(_) => "Number",
            RValue::String(_) => "String",
        }
    }
}
impl std::fmt::Display for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RValue::Void => write!(f, "Void"),
            RValue::Number(n) => write!(f, "{n}"),
            RValue::String(s) => write!(f, "{s}"),
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

macro_rules! eval_real_binary_operator { 
    ($name:literal, $children:expr, $vars:expr, $n0:ident, $n1:ident, $body:expr) => {
        { 
            if $children.len() == 2 {
                let childval0: RValue = $children[0].eval($vars);
                let childval1: RValue = $children[1].eval($vars);
                match childval0 {
                    RValue::Number($n0) => {
                        if $n0.is_real() {
                            match childval1 {
                                RValue::Number($n1) => {
                                    if $n1.is_real() {
                                        return RValue::Number($body);
                                    }else{
                                        panic!("The '{}' operator operates on values in the reals but on the right-hand side '{}' was found which has an imaginary part", $name, $n1);
                                    }
                                }
                                _ => {
                                    panic!("The '{}' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", $name, childval1.get_type());
                                }
                            }
                        }else{
                            panic!("The '{}' operator operates on values in the reals but on the left-hand side '{}' was found which has an imaginary part", $name, $n0);
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
                RValue::Number(Quantity::from_value_decorator(*val, dec))
            }
            Node::Operator(opname) => {
                let length = self.children.len();
                match &opname[..] {
                    "!" => {
                        eval_number_unary_operator!("!", self.children, vars, n0, if n0 == 0.0 {1.0.into()} else {0.0.into()})
                    }
                    "?" => {
                        eval_number_unary_operator!("?", self.children, vars, n0, if n0 != 0.0 {1.0.into()} else {0.0.into()})
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
                                            panic!("The binary '-' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", childval1.get_type());
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
                        todo!() // eval_number_binary_operator!("^", self.children, vars, n0, n1, n0.powf(n1)) 
                    }
                    "*" => {
                        eval_number_binary_operator!("*", self.children, vars, n0, n1, n0 * n1)
                    }
                    "/" => {
                        eval_number_binary_operator!("/", self.children, vars, n0, n1, n0 / n1)
                    }
                    "==" => {
                        eval_number_binary_operator!("==", self.children, vars, n0, n1, if n0 == n1 { 1.0.into() } else { 0.0.into() } )
                    }
                    ">" => {
                        eval_real_binary_operator!(">", self.children, vars, n0, n1, if n0.re > n1.re { 1.0.into() } else { 0.0.into() } )
                    }
                    ">=" => {
                        eval_real_binary_operator!(">=", self.children, vars, n0, n1, if n0.re >= n1.re { 1.0.into() } else { 0.0.into() } )
                    }
                    "<" => {
                        eval_real_binary_operator!("<", self.children, vars, n0, n1, if n0.re < n1.re { 1.0.into() } else { 0.0.into() } )
                    }
                    "<=" => {
                        eval_real_binary_operator!("<=", self.children, vars, n0, n1, if n0.re <= n1.re { 1.0.into() } else { 0.0.into() } )
                    }
                    "and" => {
                        eval_number_binary_operator!("and", self.children, vars, n0, n1, if n0 != 0.0 && n1 != 0.0 {1.0.into()} else {0.0.into()} )
                    }
                    "or" => {
                        eval_number_binary_operator!("or", self.children, vars, n0, n1, if n0 != 0.0 || n1 != 0.0 {1.0.into()} else {0.0.into()} )
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
                    "pm" => {
                        eval_number_binary_operator!("pm", self.children, vars, n0, n1, { 
                            if n0.unit == n1.unit {
                                let mut res = n0.clone();
                                res.vre = n1.re*n1.re;
                                res.vim = n1.im*n1.im;
                                res    
                            }else{
                                panic!("The 'pm' operator operates only on quantities with the same units but '{}' and '{}' where found.", n0.unit, n1.unit);
                            }
                        } )
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
                    "i" => {
                        // multiply by the imaginary unit
                        eval_number_unary_function!("i", self.children, vars, n, Quantity {
                            re: -n.im, im: n.re, vre: n.vim, vim: n.vre, unit: n.unit
                        })
                    }
                    // TWO PARAMETERS FUNCTIONS
                    "max" => {
                        eval_number_binary_function!("max", self.children, vars, n0, n1, n0.max(&n1))
                    }
                    "min" => {
                        eval_number_binary_function!("max", self.children, vars, n0, n1, n0.min(&n1))
                    }
                    // VOID FUNCTIONS
                    "write" => {
                        if self.children.len() > 0 {
                            for v in self.children.iter() {
                                print!("{}", v.eval(vars));
                            }
                            print!("\n");
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
            Node::UnitBlock(unit, factor) => {
                // assign this unit to this quantity
                eval_number_unary_operator!("UnitBlock", self.children, vars, n0, {
                    let mut res = n0.clone(); 
                    if res.unit == Unit::unitless() {
                        res.unit = unit.clone();
                        res = res * (*factor);
                        res
                    }else{
                        panic!("Applying units is allowed only on unitless values but '{}' was found next to a unit block", res)
                    }
                })
            }
            Node::StringBlock(str) => {
                let mut evaluated_string = String::with_capacity(str.len());
                let chars = str.graphemes(true).collect::<Vec<&str>>();

                let mut i = 0;
                let mut last_slash = false;
                while i < chars.len() {
                    if chars[i] == "{" && !last_slash {
                        if chars.len() == i + 1 {
                            panic!("Opening '{{' inside string is missing a corresponding '}}'");
                        }
                        let mut bcount = 1;
                        let from = i + 1;
                        let mut to = 0;
                        'bracketConsumer: while i < chars.len() {
                            if chars[i] == "}" { 
                                bcount -= 1;
                                if bcount == 0 { break 'bracketConsumer; }
                            }else if chars[i] == "}" { 
                                bcount += 1;
                                i += 1;
                            } else {
                                to = i;
                            }
                            i += 1; 
                        }
                        if bcount != 0 {
                            panic!("Opening '{{' inside string is missing a corresponding '}}'");
                        }else{
                            let varname = chars[from..=to].join("");
                            if let Some(rvalue) = vars.get(varname.trim()) {
                                let formated_variable_value = format!("{}", (*rvalue));
                                evaluated_string.push_str(&formated_variable_value);
                                i += 1;
                            }else{
                                panic!("Unable to give value to string block due to unknown variable: '{}'", varname);
                            }
                        }
                    }else if chars[i] == "{" && last_slash {
                        evaluated_string.push('{');
                        last_slash = false;
                        i += 1;
                    }else if chars[i] == "\\" && !last_slash {
                        last_slash = true;
                        i += 1;
                    } else if chars[i] == "\\" && last_slash {
                        last_slash = false;
                        evaluated_string.push('\\');
                        i += 1;
                    } else {
                        last_slash = false;
                        evaluated_string.push_str(chars[i]);
                        i += 1;
                    }
                }

                RValue::String(evaluated_string)
            }
            Node::None => {
                RValue::Void
            }
        }
    }
}