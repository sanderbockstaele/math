use unicode_segmentation::UnicodeSegmentation;
use std::fmt::Display;

const OPERATION: [char; 4] = ['+','-','*','/'];

const NUMBER: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

const DELIMITERS: [char; 2] = ['(',')'];

#[derive(Debug)]
pub enum TokenError {
    EmptyEquation,
    EmptyequationHeader,
    UnknownToken,

    #[doc(hidden)]
    __NoneExhausive,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenError::EmptyEquation => write!(formatter, "the given equation is empty"),
            TokenError::EmptyequationHeader => write!(formatter, "the given equation has no header"),
            TokenError::UnknownToken => write!(formatter, "could not find the correlating token"),
            TokenError::__NoneExhausive => write!(formatter, "an unknown error occured"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub name: String,
    pub token: Tokens,
    pub value: String,
}

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "name: {} \ttoken: {} \tvalue: {}", self.name, self.token, self.value)
    }
}

#[derive(Clone, Debug)]
pub enum Tokens {
    Function,
    Variable,
    Number,
    Operation,
    Brace,
}

impl Display for Tokens {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Tokens::Function => write!(fmt, "Function"),
            Tokens::Variable => write!(fmt, "Variable"),
            Tokens::Number => write!(fmt, "Number"),
            Tokens::Operation => write!(fmt, "Operation"),
            Tokens::Brace => write!(fmt, "Brace"),
        }
    }
}

// used if either it is a function or a variable
fn is_function(string: String) -> bool {
    let mut result: bool = false;

    for characters in string.chars() {
        if characters == '('  {
            result = true;
        }
    }

    return result;
}

fn get_function_name(characters: String) -> String {
    let mut result: String = String::new();
    println!("entered get_function_name");

    for character in characters.chars() {
        if character == '(' {
            break;
        }
        result.push(character);

    }


    return result;
}

fn is_operation (string: String) -> bool {
    let mut result: bool = false;
    
    for operation in &OPERATION {
        for characters in string.chars() {
            if operation == &characters {
                result = true;
            }
        }
    }

    return result;
}

fn is_number (string: String) -> bool {
    let mut result: bool = false;

    for number in &NUMBER {
        for character in string.chars() {
            if character == *number {
                result = true;
            }
        }
    }
    
    return result;
}


fn is_variable(characters: String) -> bool {
    let mut result: bool = false;
    

    if is_function(characters.clone()) == true {
        result = false;
    } else if is_number(characters.clone()) == true {
        result = false;
    } else if is_operation(characters.clone()) == true {
        result = false;
    } else {
        result = true;
    }

    return result;
}

fn create_character_vec(equation: &str) -> String {
    let graphemes = UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>();
    let mut result: String = "".to_string();

    for character in graphemes {
        result.push_str(character);
    }

    return result;
}

pub fn create_token_vec(equation: &str) -> Result<Vec<Token>, TokenError> {
    let mut token_list: Vec<String> = Vec::new();
    let mut result: Vec<Token> = Vec::new();

    // split the input and add each part to a vec
    let equation_parts = equation.split_whitespace();
    
    for equation_part in equation_parts {
        token_list.push(create_character_vec(equation_part));
    }
    
    // loop over the vec to find wath type the token is
    for token in &token_list {
        if is_function(token.clone()) == true {
            result.push( Token {
                name: get_function_name(token.clone()),
                token: Tokens::Function,
                value: get_function_name(token.clone()),
            });
        } else if is_number(token.clone()) == true {
            result.push( Token {
                name: token.to_string(),
                token: Tokens::Number,
                value: token.to_string(),
            });
        } else if is_operation(token.clone()) == true {
            result.push( Token {
                name: token.to_string(),
                token: Tokens::Operation,
                value: token.to_string(),
            });
        } else if is_variable(token.clone()) == true {
            result.push( Token {
                name: token.to_string(),
                token: Tokens::Variable,
                value: token.to_string(),
            });
        } else {
            return Err(TokenError::UnknownToken)
        }
    }
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_function(){
        let mut characters: String = create_character_vec("test()");
        assert_eq!(is_function(characters), true);

        
        characters = create_character_vec("12345");
        assert_eq!(is_function(characters), false);

        characters = create_character_vec("test");
        assert_eq!(is_function(characters), false);
    }
    
    #[test]
    fn test_is_number(){
        let mut characters: String = create_character_vec("123");
        assert_eq!(is_number(characters), true);

        
        characters = create_character_vec("1");
        assert_eq!(is_number(characters), true);

        characters = create_character_vec("test");
        assert_eq!(is_number(characters), false);
    }
    
    
    #[test]
    fn test_create_character_vec(){
        let characters: String = create_character_vec("test");
        assert_eq!(characters.len(), 4);
    }
    
    #[test]
    fn test_is_operation(){
        let mut characters: String = create_character_vec("+");
        assert_eq!(is_operation(characters), true);

        characters = create_character_vec("a");
        assert_eq!(is_operation(characters), false);

        characters = create_character_vec("A");
        assert_eq!(is_operation(characters), false);

        characters = create_character_vec("1");
        assert_eq!(is_operation(characters), false);
    }
    
    #[test]
    fn test_is_variable() {
        let mut characters: String = create_character_vec("abc");
        assert_eq!(is_variable(characters), true);

        characters = create_character_vec("3abc");
        assert_eq!(is_variable(characters), false);
    
        characters = create_character_vec("12345");
        assert_eq!(is_variable(characters), false);

        characters = create_character_vec("test()");
        assert_eq!(is_variable(characters), false);
    }

    #[test]
    fn test_get_function_name() {
        let mut characters: String = create_character_vec("sin()");
        
        assert_eq!(get_function_name(characters), "sin");
    }

    #[test]
    fn test_create_token_vec() {
        
    }
}
