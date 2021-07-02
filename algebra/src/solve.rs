use input::token::*;

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

fn get_operation(operation: Token, arguments: Vec<Token>) -> i64 {
}

fn dispatch_operations(argument_stack: &mut Vec<Token>, operation_stack: &[Token]) -> i64 {
    let mut operation_result: i64 = 0;
    
    for operation in operation_stack.iter() {
        let arguments: Vec<Token> = vec![
            argument_stack.remove(argument_stack.len() - 1),
            argument_stack.remove(argument_stack.len() - 1),
        ];
        operation_result = get_operation(operation.clone(), arguments);
    }

    return operation_result;
}

pub fn solve_equation(equation: &str) -> Result<Vec<f64>, TokenError> {
    let tokens: Vec<Token> = create_token_vec(equation).unwrap();
    let mut result: Vec<f64> = vec![];

    let mut result_stack: Vec<f64> = vec![];
    let mut argument_stack: Vec<Token> = vec![];
    let mut operation_stack: Vec<Token> = vec![];
    
    // sort the tokens in either the argument or operation stack
    for token in tokens {
        let token_type: Tokens = token.clone().token;
        match token_type {
            Tokens::Number => argument_stack.push(token.clone()),
            Tokens::Operation => operation_stack.push(token.clone()),
            _ => return Err(TokenError::UnknownToken),
        }
    }
    
    dispatch_operations(&mut argument_stack, &operation_stack);
    // dispatch every operation with their arguments
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        let result = pow(2.0, 2).unwrap();

        assert_eq!(result, 4.0);
    }
}
