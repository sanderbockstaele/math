use algebra::solve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = solve::solve_equation(&args[1]);
}
