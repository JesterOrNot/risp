use std::collections::HashMap;

#[allow(dead_code)]
pub enum RispExpr {
  Symbol(char),
  Number(f64),
  List(Box<RispExpr>),
}

#[allow(dead_code)]
pub enum RispErr {
  Reason(String),
}

#[allow(dead_code)]
pub struct RispVars {
  data: HashMap<String, RispExpr>,
}


/// Turns a string into a list of tokens to be lexed
/// 
/// Examples
/// 
/// ```
/// let tokens = tokenize("(+ 4 2)");
/// assert_eq!(tokens, vec![String::from("("), String::from("+"), String::from(4), String::from(2), String::from(")")])
/// ```
pub fn tokenize(expr: String) -> Vec<String> {
  expr
    .replace("(", " ( ")
    .replace(")", " ) ")
    .split_whitespace()
    .map(String::from)
    .collect()
}
