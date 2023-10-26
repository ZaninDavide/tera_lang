use std::{collections::HashMap};

use crate::ast::{Node, Tree};
use crate::quantity::{Quantity, Unit};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub enum RValue {
    Void,
    Number(Quantity),
    String(String),
    Matrix(usize, usize, Vec<RValue>),
}
impl RValue {
    fn get_type(&self) -> &'static str {
        match &self {
            RValue::Void => "Void",
            RValue::Number(_) => "Number",
            RValue::String(_) => "String",
            RValue::Matrix(_, _, _) => "Matrix", // (w,h,entries)
        }
    }
}
impl std::fmt::Display for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RValue::Void => write!(f, "Void"),
            RValue::Number(n) => write!(f, "{n}"),
            RValue::String(s) => write!(f, "{s}"),
            RValue::Matrix(w,h,v) => {
                // TODO: implement a nicer gird-form display for matrices
                let mut str = String::new();
                for j in 0..(*h) {
                    for i in 0..(*w) {
                        let cell_str = match &v[j*w + i] {
                            RValue::String(_) => { format!("\"{}\"", v[j*w + i]) }
                            RValue::Number(_) => { format!("{}", v[j*w + i]) }
                            RValue::Matrix(_,_,_) => { format!("{}", v[j*w + i]) }
                            RValue::Void => { format!("{}", v[j*w + i]) }
                        };
                        str.push_str(&cell_str);
                        if i < w - 1 {
                            str.push_str(", ");
                        }
                    }
                    if j < h - 1 {
                        str.push_str("; ");
                    }
                }
                write!(f, "Matrix {h}×{w}: [{str}]")
            },
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
                    "&" => {
                        eval_number_unary_operator!("&", self.children, vars, n0, n0.sigma())
                    }
                    "$" => {
                        eval_number_unary_operator!("$", self.children, vars, n0, n0.value())
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
                                            if n0.unit != n1.unit { panic!("The binary '+' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
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
                                            if n0.unit != n1.unit { panic!("The binary '-' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
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
                        eval_number_binary_operator!("==", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The binary '==' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
                            if n0 == n1 { 1.0.into() } else { 0.0.into() }
                        } )
                    }
                    ">" => {
                        eval_real_binary_operator!(">", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The binary '>' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
                            if n0.re > n1.re { 1.0.into() } else { 0.0.into() }
                        } )
                    }
                    ">=" => {
                        eval_real_binary_operator!(">=", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The binary '>=' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
                            if n0.re >= n1.re { 1.0.into() } else { 0.0.into() }
                        } )
                    }
                    "<" => {
                        eval_real_binary_operator!("<", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The binary '<' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
                            if n0.re < n1.re { 1.0.into() } else { 0.0.into() }
                        } )
                    }
                    "<=" => {
                        eval_real_binary_operator!("<=", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The binary '<=' operator operates on quantities with the same units but '{}' and '{}' were found.", n0.unit, n1.unit) }
                            if n0.re <= n1.re { 1.0.into() } else { 0.0.into() }
                        } )
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
                            if n0.unit != n1.unit { panic!("The 'pm' operator operates only on quantities with the same units but '{}' and '{}' where found.", n0.unit, n1.unit); }
                            let mut res = n0.clone();
                            res.vre = n1.re*n1.re;
                            res.vim = n1.im*n1.im;
                            res    
                        } )
                    }
                    "while" => {
                        if self.children.len() == 2 {
                            // WHILE 
                            let mut res: Vec<RValue> = Vec::new();
                            while {
                                let ev = &self.children[0].eval(vars);
                                let condition = if let RValue::Number(cond) = ev { cond } else {
                                    panic!("While statements require numeric values as condition but {} was found.", ev);
                                };
                                *condition != 0.0
                            } {
                                res.push(self.children[1].eval(vars));                                
                            }
                            RValue::Matrix(1, res.len(), res)
                        }else{
                            panic!("The 'while' operator is a prefixed binary operator but a number of {} children was found.", self.children.len());
                        }
                    }
                    "for" => {
                        if self.children.len() == 3 {
                            // FOR
                            if let Node::Variable(index_name) = &self.children[0].node {
                                if let Node::Variable(matrix_name) = &self.children[1].node {
                                    // if we iterate on a variable we avoid evaluating the expression and
                                    // use the variable directly
                                    let matrix: &RValue = match vars.get(matrix_name) { 
                                        Some(m) => m, 
                                        None => { panic!("'{}' is not an existing variable.", matrix_name) }
                                    };
                                    let (w, h) = match matrix {
                                        RValue::Matrix(w, h, _) => (*w, *h),
                                        _ => { panic!("'{}' is not a variable containing a matrix.", matrix_name) } 
                                    };
                                    // actually executing the for statement
                                    let mut res_vec = Vec::with_capacity(w*h);
                                    for x in 0..w {
                                        for y in 0..h {
                                            let matrix: &RValue = match vars.get(matrix_name) { 
                                                Some(m) => m, 
                                                None => { panic!("'{}' is not an existing variable.", matrix_name) }
                                            };
                                            let cur = match matrix {
                                                RValue::Matrix(_, _, v) => { (v[y*w + x]).clone() },
                                                _ => { panic!("'{}' is not a variable containing a matrix.", matrix_name) } 
                                            };
                                            vars.insert(index_name.clone(), cur);
                                            res_vec.push(self.children[2].eval(vars));
                                        }
                                    }
                                    RValue::Matrix(w, h, res_vec)
                                }else if self.children[1].has_value {
                                    let matrix: RValue = self.children[1].eval(vars);
                                    let (w, h, vec_matrix) = match matrix {
                                        RValue::Matrix(w, h, vec_matrix) => (w, h, vec_matrix),
                                        value => { panic!("'for' statements iterate over matrices but the given expression was evaluated as {}, which is not a matrix.", value) } 
                                    };
                                    // actually executing the for statement
                                    let mut res_vec = Vec::with_capacity(w*h);
                                    for x in 0..w {
                                        for y in 0..h {
                                            vars.insert(index_name.clone(), vec_matrix[y*w + x].clone());
                                            res_vec.push(self.children[2].eval(vars));
                                        }
                                    }
                                    RValue::Matrix(w, h, res_vec)
                                }else{
                                    panic!("The element after the 'in' keyword of a 'for' statement must be a valid variable name or a valued expression. Found {:?} instead.", self.children[1]);
                                }
                            }else{
                                panic!("The element after a 'for' operator must be a valid variable name. Found {:?} instead, which is not a variable name.", self.children[0]);
                            }
                        }else{
                            panic!("The 'for' operator should have three children but a number of {} children was found.", self.children.len());
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
                        eval_number_unary_function!("sin", self.children, vars, n, {
                            if !n.unit.is_unitless() { panic!("The 'sin' function operates on unitless quantities but '{n}' was found.") }
                            n.sin()
                        })
                    }
                    "cos" => {
                        eval_number_unary_function!("cos", self.children, vars, n, {
                            if !n.unit.is_unitless() { panic!("The 'cos' function operates on unitless quantities but '{n}' was found.") }
                            n.cos()
                        })
                    }
                    "i" => {
                        // multiply by the imaginary unit
                        eval_number_unary_function!("i", self.children, vars, n, Quantity {
                            re: -n.im, im: n.re, vre: n.vim, vim: n.vre, unit: n.unit
                        })
                    }
                    "exp" => {
                        eval_number_unary_function!("exp", self.children, vars, n, {
                            if !n.unit.is_unitless() { panic!("The 'exp' function operates on unitless quantities but '{n}' was found.") }
                            n.exp()
                        })
                    }
                    "Re" | "real" => {
                        eval_number_unary_function!("Re", self.children, vars, n, n.real_part())
                    }
                    "Im" | "imag" => {
                        eval_number_unary_function!("Im", self.children, vars, n, n.imag_part())
                    }
                    "sigma" => {
                        eval_number_unary_function!("sigma", self.children, vars, n, n.sigma())
                    }
                    "sigma2" => {
                        eval_number_unary_function!("sigma2", self.children, vars, n, n.sigma2())
                    }
                    "value" => {
                        eval_number_unary_function!("value", self.children, vars, n, n.value())
                    }
                    "abs" => {
                        eval_number_unary_function!("value", self.children, vars, n, n.abs())
                    }
                    "arg" => {
                        eval_number_unary_function!("value", self.children, vars, n, n.arg())
                    }
                    // TWO PARAMETERS FUNCTIONS
                    "max" => {
                        eval_number_binary_function!("max", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The 'max' function operates on quantities with the same units but '{n0}' and '{n1}' were found.") }
                            n0.max(&n1)
                        })
                    }
                    "min" => {
                        eval_number_binary_function!("min", self.children, vars, n0, n1, {
                            if n0.unit != n1.unit { panic!("The 'min' function operates on quantities with the same units but '{n0}' and '{n1}' were found.") }
                            n0.min(&n1)
                        })
                    }
                    // VOID FUNCTIONS
                    "write" => {
                        if self.children.len() > 0 {
                            for v in self.children.iter() {
                                print!("{}", v.eval(vars));
                            }
                            RValue::Void
                        }else{                        
                            panic!("The 'write' function takes one or more parameters but no parameters were found.")
                        }
                    }
                    "print" => {
                        if self.children.len() > 0 {
                            for v in self.children.iter() {
                                print!("{} ", v.eval(vars));
                            }
                            print!("\n");
                            RValue::Void
                        }else{                        
                            panic!("The 'print' function takes one or more parameters but no parameters were found.")
                        }
                    }
                    "assert" => {
                        if self.children.len() == 1 || self.children.len() == 2 {
                            let v = self.children[0].eval(vars);
                            let mut should_panic = false;
                            match v {
                                RValue::Void => {
                                    should_panic = true;
                                }
                                RValue::Number(n) => {
                                    if n.re != 1.0 || n.im != 0.0 || n.vre != 0.0 || n.vim != 0.0 {
                                        should_panic = true;
                                    }
                                }
                                RValue::String(_) => { should_panic = true; }
                                RValue::Matrix(_, _, _) => { should_panic = true; }
                            }
                            if should_panic {
                                if self.children.len() == 2 {
                                    panic!("{}", self.children[1].eval(vars));
                                }else{
                                    panic!();
                                }
                            }
                            RValue::Void
                        }else{                        
                            panic!("The 'assert' function takes one or two parameters but {} parameters were found.", self.children.len())
                        }
                    }
                    "error" => {
                        if self.children.len() == 1 {
                            panic!("{}", self.children[0].eval(vars));
                        }else if self.children.len () == 0 {
                            panic!();
                        }else{
                            panic!("The 'error' function takes one or two parameters but {} parameters were found.", self.children.len())
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
            Node::UnitBlock(unit, factor, shift) => {
                // assign this unit to this quantity
                eval_number_unary_operator!("UnitBlock", self.children, vars, n0, {
                    let mut res = n0.clone(); 
                    if res.unit == Unit::unitless() {
                        res.unit = unit.clone();
                        res.re += shift;
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
                            panic!("Opening '{{' inside string is missing a corresponding '}}': {str}");
                        }
                        let mut bcount = 1;
                        let varname_from: usize = i + 1;
                        let mut varname_to: usize = 0;
                        let mut unit_from: usize = 0;
                        let mut unit_to: usize = 0;
                        i += 1;
                        'bracketConsumer: while i < chars.len() {
                            if chars[i] == "}" { 
                                bcount -= 1;
                                if bcount == 0 { break 'bracketConsumer; }
                            }else if chars[i] == "{" {
                                bcount += 1;
                                i += 1;
                                if bcount > 1 {
                                    panic!("String block cannot contain nested brackets: '{str}'"); 
                                }
                            } else if chars[i] == "|" {
                                // unit block
                                if chars.len() == i + 1 {
                                    panic!("Opening '|' inside string is missing a corresponding '|': {str}");
                                }
                                unit_from = i + 1;
                                i += 1;
                                'unitConsumer: while i < chars.len() {
                                    if chars[i] == "|" {
                                        unit_to = i - 1;
                                        break 'unitConsumer;
                                    }else{
                                        i += 1;                                        
                                    }
                                }
                                if unit_to == 0 {
                                    panic!("String block cannot contain nested brackets: '{str}'"); 
                                }
                            } else if unit_to != 0 && chars[i] != " " {
                                panic!("String block should finish with the name of the unit: '{str}'");
                            } else if unit_to != 0 && chars[i] == " " {
                                // just skip the space
                            } else {
                                varname_to = i;
                            }
                            i += 1; 
                        }
                        if bcount != 0 {
                            panic!("Opening '{{' inside string is missing a corresponding '}}': '{str}'");
                        }else{
                            let varname: String = chars[varname_from..=varname_to].join("");
                            if let Some(rvalue) = vars.get(varname.trim()) {
                                let unit_full_string: String = chars[unit_from..=unit_to].join("");
                                let unit_string: String = if unit_to > 0 {
                                    unit_full_string.trim().to_owned()
                                } else {
                                    String::new()
                                };
                                let formated_variable_value = match rvalue {
                                    RValue::Number(q) => {
                                        q.to_text(unit_string)
                                    }
                                    _ => {
                                        format!("{}", (*rvalue))
                                    }
                                };
                                evaluated_string.push_str(&formated_variable_value);
                                i += 1;
                            }else{
                                panic!("Unable to give value to string block due to unknown variable: '{}'", varname.trim());
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
            Node::MatrixBlock(width, height) => {
                let mut fields = Vec::new();
                
                let l = self.children.len();
                for i in 0..l {
                    let value = self.children[i].eval(vars);
                    fields.push(value);
                }

                RValue::Matrix(*width, *height, fields)
            }
            Node::MatrixIndexing(matrix_name) => {
                let index0 = if self.children.len() > 0 { self.children[0].eval(vars) } else { RValue::Void };
                let index1 = if self.children.len() > 1 { self.children[1].eval(vars) } else { RValue::Void };

                let original_index_y: i64 = match index0 {
                    RValue::Number(n) => {
                        if n.im == 0.0 && n.vim == 0.0 && n.vre == 0.0 {
                            let i = n.re.floor();
                            if n.re == i && n.re != 0.0 {
                                i as i64
                            }else{
                                panic!("Only pure, integer, non zero values are allowed when indexing a matrix but '{}' was found.", n);
                            }
                        } else{
                            panic!("Only pure, integer, non zero values are allowed when indexing a matrix but '{}' was found.", n);
                        }
                    }
                    other => {
                        panic!("Cannot index matrix with type '{}', '{}' was found.", other.get_type(), other);
                    }
                };


                if let Some(rvalue) = vars.get(matrix_name) {
                    match rvalue {
                        RValue::Matrix(w, h, v) => {
                            if self.children.len() == 1 && *w == 1usize {
                                let index_y = if original_index_y < 0 { (*h as i64) + original_index_y + 1} else { original_index_y } - 1;
                                if index_y >= 0 && index_y < (*h as i64) { 
                                    v[index_y as usize].clone()
                                }else{
                                    panic!("Index must not exceed Matrix bounds. Matrix '{matrix_name}' is '{h}×{w}' but '{original_index_y}' was found.")
                                }
                            }else if self.children.len() == 2 {
                                let original_index_x: i64 = match index1 {
                                    RValue::Number(n) => {
                                        if n.im == 0.0 && n.vim == 0.0 && n.vre == 0.0 {
                                            let i = n.re.floor();
                                            if n.re == i && n.re != 0.0 {
                                                i as i64
                                            }else{
                                                panic!("Only pure, integer, non zero values are allowed when indexing a matrix but '{}' was found.", n);
                                            }
                                        } else{
                                            panic!("Only pure, integer, non zero values are allowed when indexing a matrix but '{}' was found.", n);
                                        }
                                    }
                                    other => {
                                        panic!("Cannot index matrix with type '{}'.", other.get_type());
                                    }
                                };
                                let index_x = if original_index_x < 0 { (*w as i64) + original_index_x + 1} else { original_index_x } - 1;
                                let index_y = if original_index_y < 0 { (*h as i64) + original_index_y + 1} else { original_index_y } - 1;
                                v[(index_y*(*w as i64) + index_x) as usize].clone()
                            }else if self.children.len() == 1 && *w != 1usize {
                                panic!("Cannot index a matrix using one index unless it is a column vector but {matrix_name} is '{h}×{w}' has '{h}' rows and '{w}' columns.");
                            }else{
                                panic!("Cannot index a matrix using '{}' indices", self.children.len());
                            }
                        }
                        _ => {
                            panic!("Unable to index inside '{matrix_name}' because it is of type '{}'. Only variables of type 'Matrix' can be indexed.", rvalue.get_type());
                        }
                    }
                }else{
                    panic!("Unable to give value to:\n {:?}", &self);
                }
            }
            Node::Keyword(str) => {
                panic!("Trying to give value to '{}', which is a keyword and thus has no value.", str);
            }
            Node::None => {
                RValue::Void
            }
        }
    }
}