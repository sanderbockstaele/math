use algebra::error::AlgebraError;
use unicode_segmentation::UnicodeSegmentation;

const LETTER: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'
,'q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M',
'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

const OPERATION: [char; 4] = ['+','-','*','/'];

const NUMBER: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

#[derive(Clone)]
struct Token {
    name: String,
    token: Tokens,
    value: String,
}

struct Equation {
    tokens: Vec<Token>,
    name: &'static str,
}

#[derive(Clone)]
enum Tokens {
    Function,
    Variable,
    Number,
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

    for character in characters.chars() {
        while character != '(' {
            result.push(character);
        }
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

fn create_token_vec(equation: &str) -> Result<Vec<Token>, AlgebraError> {
    let mut token_list: Vec<String> = Vec::new();
    let mut result: Vec<Token> = Vec::new();

    let equation_parts = equation.split_whitespace();
    
    for equation_part in equation_parts {
        token_list.push(create_character_vec(equation_part));
    }

    for token in &token_list {
        if is_function(token.clone()) == true {
            result.push( Token {
                name: get_function_name(token.clone()),
                token: Tokens::Function,
                value: get_function_name(token.clone()),
            });
        } if is_number(token.clone()) == true {
            result.push( Token {
                name: String::new(),
                token: Tokens::Number,
                value: token.to_string(),
            });
        } else {
            return Err(AlgebraError::UnknownToken)
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
    
}
