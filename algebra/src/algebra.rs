use emath::Pos2;
use input::token::*;

enum Types {
    Float,
    Int,
    Unknown
}

#[derive(Debug)]
enum AlgebraError {
    ToManyArguments,
    ToLessArguments,
    NoArguments,
}

fn sqrt(input: String) -> f64 {
    let input_float: f64 = input.parse::<f64>().unwrap();

    input_float.sqrt()
}

fn pow(input: Vec<String>) -> Result<f64, AlgebraError> {
    let mut input_float: Vec<f64> = Vec::new();
    
    for string in input {
        input_float.push( string.parse::<f64>().unwrap());
    }

    if input_float.len() > 2 {
        return Err(AlgebraError::ToManyArguments);
    }
    if input_float.len() < 2 {
        return Err(AlgebraError::ToLessArguments);
    }
    if input_float.len() == 0 {
        return Err(AlgebraError::NoArguments);
    }
    
    let mut iterator: f64 = 0.0;
    let mut result: f64 = input_float[0];
    
    while iterator < input_float[1] {
        result = result * input_float[0];
        
        iterator = iterator + 1.0;
    }
    
    Ok(result)
}

fn get_function_result(function_name: String, arguments: Vec<String>) -> String {
    match function_name.as_str() {
        "sqrt" => sqrt(arguments[0].clone()).to_string(),
        "pow" => { 
            pow(vec![arguments[0].clone(), arguments[1].clone()]).unwrap().to_string()
        }, 
        &_ => "".to_string(),
    }
}

pub fn solve_equation(equation: &str) -> Vec<Pos2> {

    let mut result :Vec<Pos2> =  vec![

    ];

    return result;
}
