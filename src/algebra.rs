use unicode_segmentation::UnicodeSegmentation;
use emath::Pos2;

const LETTER: [&str; 52] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p"
,"q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M",
"N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];

const OPERATION: [&str; 4] = ["+","-","*","/"];

const NUMBER: [&str; 10] = ["0","1","2","3","4","5","6","7","8","9"];

struct Token<'a> {
    name: &'a str,
    token: Tokens,
    value: &'a str,
}

struct Equation<'a> {
    tokens: Vec<Token<'a>>,
    name: &'a str,
}

enum Tokens {
    Function,
    Variable,
    Number,
}

// used if either it is a function or a variable
fn is_function(characters: &Vec<&str>) -> bool {
    let mut iterator = characters.iter().peekable();

    while iterator.peek() != None {
        if iterator.peek().unwrap() == &&"(" {
            return true;
        }
        iterator.next();
    }

    return false;
}

fn get_function_name(characters: &Vec<&str>) -> &str {
    let mut result: &str = &str::new();

    for character in characters {
        while character != "(" {
            result.push_str(character);
        }
    }

    return result; 
}

fn is_operation (characters: &Vec<&str>) -> bool {
    let mut result: bool = false;
    
    for operation in &OPERATION {
        for _character in characters.iter().take_while(|&c| c == operation ) {
           result = true;    
        }
    }

    println!("{}", result);

    return result;
}

fn is_number (characters: &Vec<&str>) -> bool {
    let mut result: bool = false;

    for number in &NUMBER {
        for _character in characters.iter().take_while(|&c| c == number) {
            result = true;
        }
    }
    return result;
}

fn is_variable(characters: &Vec<&str>) -> bool {
    let mut result: bool = false;
    

    if is_function(&characters) == true {
        result = false;
    } else if is_number(&characters) == true {
        result = false;
    } else if is_operation(&characters) == true {
        result = false;
    } else {
        result = true;
    }

    return result;
}

fn create_character_vec(equation: &str) -> Vec<&str> {
    UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>()
}

fn create_token_vec(equation: &str) -> Vec<Token> {
    let mut token_list: Vec<Vec<&str>> = Vec::new();
    let mut result: Vec<Token> = Vec::new();

    let equation_parts = equation.split_whitespace();
    
    for equation_part in equation_parts {
        token_list.push(create_character_vec(equation_part);
    }

    for (token, i) in token_list.iter().enumerate() {
        if is_function(&token_list[i]) == true {
            result.push( Token {
                
            });
        }
    }


}

pub fn solve_equation(equation: &str) -> Vec<Pos2> {


    let graph = vec![

    ];

    return graph;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_function(){
        let mut characters: Vec<&str> = create_character_vec("test()");
        assert_eq!(is_function(&characters), true);

        
        characters = create_character_vec("12345");
        assert_eq!(is_function(&characters), false);

        characters = create_character_vec("test");
        assert_eq!(is_function(&characters), false);
    }

    #[test]
    fn test_is_number(){
        let mut characters: Vec<&str> = create_character_vec("123");
        assert_eq!(is_number(&characters), true);

        
        characters = create_character_vec("1");
        assert_eq!(is_number(&characters), true);

        characters = create_character_vec("test");
        assert_eq!(is_number(&characters), false);
    }

    #[test]
    fn test_create_character_vec(){
        let characters: Vec<&str> = create_character_vec("test");
        assert_eq!(characters.len(), 4);

        characters_vec: Vec<&str> = ["t","e","s","t"].to_vec();
        assert_eq!(characters, characters_vec);
    }

    #[test]
    fn test_is_operation(){
        let mut characters: Vec<&str> = create_character_vec("+");
        assert_eq!(is_operation(&characters), true);

        characters = create_character_vec("a");
        assert_eq!(is_operation(&characters), false);

        characters = create_character_vec("A");
        assert_eq!(is_operation(&characters), false);

        characters = create_character_vec("1");
        assert_eq!(is_operation(&characters), false);
    }
    
    #[test]
    fn test_is_variable() {
        let mut characters: Vec<&str> = create_character_vec("abc");
        assert_eq!(is_variable(&characters), true);

        characters = create_character_vec("3abc");
        assert_eq!(is_variable(&characters), false);
    
        characters = create_character_vec("12345");
        assert_eq!(is_variable(&characters), false);

        characters = create_character_vec("test()");
        assert_eq!(is_variable(&characters), false);
    }

    #[test]
    fn test_get_function_name() {
        let mut characters: Vec<&str> = create_character_vec("sin()");
        assert_eq!(get_function_name(characters), "sin");


    }
}
