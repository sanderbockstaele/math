use input::token::*;

#[derive(Debug)]
enum ResultError {
    OperationNotFound,
}

struct Expression{
    tokens: Vec<Token>,
    arguments: Vec<f64>,
    operations: Vec<String>,
    results: Vec<f64>,
}

trait Solve {
    fn get_operation_result(&self) -> Result<f64, ResultError>;

    fn new() -> Self;
}

impl Solve for Expression {
    fn new() -> Self {
        Expression {
            tokens: Vec::new(),
            arguments: Vec::new(),
            operations: Vec::new(),
            results: Vec::new(),
        }
    }

    fn get_operation_result(&self) -> Result<f64, ResultError> {
        match self.operations[0].as_str() {
            "+" => { Ok(self.arguments[1] + self.arguments[0]) },
            "-" => { Ok(self.arguments[1] - self.arguments[0]) },
            "*" => { Ok(self.arguments[1] * self.arguments[0]) },
            "/" => { Ok(self.arguments[1] / self.arguments[0]) },
            _=> { return Err(ResultError::OperationNotFound) }
        }
    }
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

pub fn solve_equation(equation: &str) -> Result<Vec<f64>, TokenError> {
    let tokens: Vec<Token> = create_token_vec(equation).unwrap();
    let mut result: Vec<f64> = vec![];

    let mut expression: Expression = Expression::new();
    
    expression.tokens = create_token_vec(equation).unwrap();

    // sort the tokens in either the argument or operation stack
    for token in tokens {
        let token_type: Tokens = token.clone().token;
        match token_type {
            Tokens::Number => expression.arguments.push(convert_to_f64(token.clone().value).unwrap()),
            Tokens::Operation => expression.operations.push(token.clone().name),
            _ => return Err(TokenError::UnknownToken),
        }
    }
    
    expression.results.push(Expression::get_operation_result(&expression).unwrap());
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
