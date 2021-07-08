use input::token::*;
use std::cmp::Ordering;

#[derive(Debug)]
enum ResultError {
    OperationNotFound,
    ToMuchArguments,
    ToLessArguments,
}

struct Expression{
    tokens: Vec<Token>,
    arguments: Vec<f64>,
    operations: Vec<String>,
    results: Vec<f64>,
}

trait Solve {
}

fn convert_to_u64(input: String) -> Result<u64, std::num::ParseIntError> {
    let result = match input.parse::<u64>() {
        Ok(input) => input,
        Err(e) => return Err(e),
    };

    Ok(result)
}

fn convert_to_f64(input: String) -> Result<f64, std::num::ParseFloatError> {
    let result = match input.parse::<f64>() {
        Ok(input) => input,
        Err(e) => return Err(e),
    };

    Ok(result)
}

fn sqrt(input: f64) -> f64 {
    input.sqrt()
}

fn pow(input: f64, power: u64) -> f64{
    let mut iterator: u64 = 0;
    let mut result: f64 = 0.0;
    
    while iterator < power {
        result = result * input;
        
        iterator = iterator + 1;
    }
    
    return result;
}

fn get_function_result(function_name: String, arguments: Vec<String>) -> String {
    let mut int_arguments: Vec<u64> = Vec::new();
    let mut float_arguments: Vec<f64> = Vec::new();

    // create i64 forms of all the arguments
    for string in &arguments {
        int_arguments.push(convert_to_u64(string.to_string()).unwrap()) 
    }

    // create f64 forms of all the arguments
    for string in &arguments {
        float_arguments.push(convert_to_f64(string.to_string()).unwrap())
    }

    match function_name.as_str() {
        "sqrt" => sqrt(convert_to_f64(arguments[0].clone()).unwrap()).to_string(),
        "pow" =>  pow(convert_to_f64(arguments[0].clone()).unwrap(), int_arguments[1]).to_string(), 
        &_ => "".to_string(),
    }
}

fn get_operation_result(operation: String, arguments: Vec<f64>) -> Result<f64, ResultError> {
    match arguments.len().cmp(&2) {
        Ordering::Less => return Err(ResultError::ToLessArguments),
        Ordering::Greater => return Err(ResultError::ToMuchArguments),
        Ordering::Equal => {},
    }
    match operation.as_str() {
        "+" => { Ok(arguments[1] + arguments[0]) },
        "-" => { Ok(arguments[1] - arguments[0]) },
        "*" => { Ok(arguments[1] * arguments[0]) },
        "/" => { Ok(arguments[1] / arguments[0]) },
        _=> { return Err(ResultError::OperationNotFound) }
    }
}

fn dispatch_operations(argument_stack: &mut Vec<f64>, operation_stack: &mut Vec<String>) -> f64 {
    let mut operation_result: f64 = 0.0;
    
    for operation in operation_stack.iter() {
        let arguments: Vec<f64> = vec![
            argument_stack.remove(argument_stack.len() - 1),
            argument_stack.remove(argument_stack.len() - 1),
        ];
        operation_result = get_operation_result(operation.clone(), arguments).unwrap();
    }

    return operation_result;
}

pub fn solve_equation(equation: &str) -> Result<Vec<f64>, TokenError> {
    let tokens: Vec<Token> = create_token_vec(equation).unwrap();
    let mut result: Vec<f64> = vec![];

    let mut result_stack: Vec<f64> = vec![];
    let mut argument_stack: Vec<f64> = vec![];
    let mut operation_stack: Vec<String> = vec![];
    
    // sort the tokens in either the argument or operation stack
    for token in tokens {
        let token_type: Tokens = token.clone().token;
        match token_type {
        Tokens::Number => argument_stack.push(convert_to_f64(token.clone().value).unwrap()),
            Tokens::Operation => operation_stack.push(token.clone().name),
            _ => return Err(TokenError::UnknownToken),
        }
    }
    
    result.push(dispatch_operations(&mut argument_stack, &mut operation_stack));
    // dispatch every operation with their arguments
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[teist]
    fn test_pow() {
        let result = pow(2.0, 2).unwrap();

        assert_eq!(result, 4.0);
    }
    #[test]
    fn test_get_operation() {
        
    }
}
