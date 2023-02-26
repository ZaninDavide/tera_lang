use crate::ast::{Node, Tree};

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

macro_rules! eval_number_unary_operator { 
    ($name:literal, $children:expr, $n0:ident, $body:expr) => {
        { 
            if $children.len() == 1 {
                let childval0: RValue = $children[0].eval();
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
    ($name:literal, $children:expr, $n0:ident, $n1:ident, $body:expr) => {
        { 
            if $children.len() == 2 {
                let childval0: RValue = $children[0].eval();
                let childval1: RValue = $children[1].eval();
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
    ($name:literal, $children:expr, $n0:ident, $body:expr) => {
        { 
            if $children.len() == 1 {
                let childval0: RValue = $children[0].eval();
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
    ($name:literal, $children:expr, $n0:ident, $n1:ident, $body:expr) => {
        { 
            if $children.len() == 2 {
                let childval0: RValue = $children[0].eval();
                let childval1: RValue = $children[1].eval();
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
    pub fn eval(&self) -> RValue {
        if let Node::Number(val, dec) = &self.node {
            // TODO: number to value
            return RValue::Number(*val);
        }else if let Node::Operator(opname) = &self.node {
            let length = self.children.len();
            match &opname[..] {
                "!" => {
                    eval_number_unary_operator!("!", self.children, n0, if n0 == 0.0 {1.0} else {0.0})
                }
                "?" => {
                    eval_number_unary_operator!("!", self.children, n0, if n0 != 0.0 {1.0} else {0.0})
                }
                "+" => {
                    if length == 1 {
                        let childval = self.children[0].eval();
                        match childval {
                            RValue::Number(_) => {
                                return childval;
                            }
                            _ => {
                                panic!("The unary '+' operator operates on values of type 'Number' but an element of type '{}' was found.", childval.get_type());
                            }
                        }
                    }else if length == 2 {
                        let childval0 = self.children[0].eval();
                        let childval1 = self.children[1].eval();
                        match childval0 {
                            RValue::Number(n0) => {
                                match childval1 {
                                    RValue::Number(n1) => {
                                        return RValue::Number(n0 + n1);
                                    }
                                    _ => {
                                        panic!("The binary '+' operator operates on values of type 'Number' but an element of type '{}' was found on the right-hand side.", childval0.get_type());
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
                        let childval = self.children[0].eval();
                        match childval {
                            RValue::Number(n) => {
                                return RValue::Number(-n);
                            }
                            _ => {
                                panic!("The unary '-' operator operates on values of type 'Number' but an element of type '{}' was found.", childval.get_type());
                            }
                        }
                    }else if length == 2 {
                        let childval0 = self.children[0].eval();
                        let childval1 = self.children[1].eval();
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
                    eval_number_binary_operator!("^", self.children, n0, n1, n0.powf(n1))
                }
                "*" => {
                    eval_number_binary_operator!("*", self.children, n0, n1, n0 * n1)
                }
                "/" => {
                    eval_number_binary_operator!("/", self.children, n0, n1, n0 / n1)
                }
                "==" => {
                    eval_number_binary_operator!("==", self.children, n0, n1, if n0 == n1 { 1.0 } else { 0.0 } )
                }
                ">" => {
                    eval_number_binary_operator!(">", self.children, n0, n1, if n0 > n1 { 1.0 } else { 0.0 } )
                }
                ">=" => {
                    eval_number_binary_operator!(">=", self.children, n0, n1, if n0 >= n1 { 1.0 } else { 0.0 } )
                }
                "<" => {
                    eval_number_binary_operator!("<", self.children, n0, n1, if n0 < n1 { 1.0 } else { 0.0 } )
                }
                "<=" => {
                    eval_number_binary_operator!("<=", self.children, n0, n1, if n0 <= n1 { 1.0 } else { 0.0 } )
                }
                "and" => {
                    eval_number_binary_operator!("and", self.children, n0, n1, if n0 != 0.0 && n1 != 0.0 {1.0} else {0.0} )
                }
                "or" => {
                    eval_number_binary_operator!("or", self.children, n0, n1, if n0 != 0.0 || n1 != 0.0 {1.0} else {0.0} )
                }
                _ => {
                    panic!("Unknown operator '{}'", opname);
                }
            }
        } else if let Node::FunctionCall(fname) = &self.node {
            match &fname[..] {
                // ONE PARAMETER FUNCTIONS
                "sin" => {
                    eval_number_unary_function!("sin", self.children, n, n.sin())
                }
                "cos" => {
                    eval_number_unary_function!("cos", self.children, n, n.cos())
                }
                // TWO PARAMETERS FUNCTIONS
                "max" => {
                    eval_number_binary_function!("max", self.children, n0, n1, n0.max(n1))
                }
                "min" => {
                    eval_number_binary_function!("max", self.children, n0, n1, n0.min(n1))
                }
                // VOID FUNCTIONS
                "write" => {
                    if self.children.len() > 0 {
                        for v in self.children.iter() {
                            print!("{}", v.eval());
                        }
                        RValue::Void
                    }else{                        
                        panic!("The 'write' function takes one or more parameters but no parameters were found.")
                    }
                }
                _ => {
                    panic!("Unknown function called '{}'", &fname);
                }
            }
        }else{
            panic!("Unable to give value to:\n {:?}", &self);
        }
    }
}