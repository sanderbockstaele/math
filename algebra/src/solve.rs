use emath::Pos2;
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
    let mut input_float: Vec<f64> = Vec::new(); 
    
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

    for string in &arguments {
        int_arguments.push(convert_to_u64(string.to_string()).unwrap()) 
    }

    match function_name.as_str() {
        "sqrt" => sqrt(convert_to_f64(arguments[0].clone()).unwrap()).to_string(),
        "pow" =>  pow(convert_to_f64(arguments[0].clone()).unwrap(), int_arguments[1]).to_string(), 
        &_ => "".to_string(),
    }
}

fn handle_error(input: AlgebraError) {

}

pub fn solve_equation(equation: &str) -> Vec<Pos2> {
    let tokens: Vec<input::token::Token> = input::token::create_token_vec(equation).unwrap();
    
    let mut result :Vec<Pos2> =  vec![

    ];

    return result;
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
