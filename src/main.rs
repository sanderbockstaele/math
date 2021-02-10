mod algebra;
extern crate pest;

#[macro_use]
extern crate pest_derive;

use crate::algebra::solve_equation;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

#[macro_use] extern crate text_io;

fn parse(input: String){
    let pairs = IdentParser::parse(Rule::input, &input).unwrap_or_else(|e| panic!("{}", e));

    for pairs in pairs {
        for inner_pair in pairs.into_inner() {
            match inner_pair.as_rule() {
                Rule::equation => {
                    solve_equation(inner_pair.as_str().to_string());
                }
                _ => {

                }
            }
        }
        
    }
}

fn input_loop(){
    let string: String = read!("{}\n");
    parse(string);

}

fn main() {
    input_loop();
}
