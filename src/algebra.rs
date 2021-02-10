use std::error::Error;
use std::fmt;

extern crate pest;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

#[derive(Debug)]
pub struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn solve_equation(equation: String) -> Result<i64, MyError> {
	if equation.is_empty() {
		return Err(MyError::new("the given equation is empty"))
	} else {
		return Ok(0)
	}

}