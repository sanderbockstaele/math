extern crate pest;
extern crate emath;

use pest::Parser;
use emath::Pos2;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

pub fn solve_equation(equation: String) -> Vec<Pos2> {

}