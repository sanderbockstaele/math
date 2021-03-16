use unicode_segmentation::UnicodeSegmentation;
use emath::Pos2;
use array2d::Array2D;

const LETTER: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'
,'q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M',
'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

const OPERATION: [char; 4] = ['+','-','*','/'];

struct Token<'a> {
    name: &'a str,
    token: Tokens,
    value: &'a str,
}

enum Tokens {
    Function,
    Variable,
    Number,
}

// used if either it is a function or a variable
fn is_function(characters: Vec<&str>) -> bool {
    let mut iterator = characters.iter().peekable();

    while iterator.peek() != None {
        if iterator.peek().unwrap() == &&"(" {
            return true;
        }
        iterator.next();
    }

    return false;
}



fn is_variable(characters: Vec<&str>) -> bool {
    let mut iterator = characters.iter().peekable();

    while iterator.peek() != None {
        // here are 2 problems and workarounds
        // 1.
        // iterator.peek().unwrap returns a &&&str
        // and you can't compare a &&&str to a char
        // 2.
        // i made a const array of all the alphabetic characters
        // because somehow .is_alphabetic() doesn't work
        for i in 1..52 {
            let current_character: &str = &LETTER[i].to_string();
            if iterator.peek().unwrap() == &&current_character{
                return true;                      
            }
        }

        for i in 4 {
            let curren_operation: &str = &OPERATION[i].to_string();
            if iterator.peek().unwrap() == && curren_operation {
                return false;
            }
        }
    }

    return false;
}

fn create_character_vec(equation: &str) -> Vec<&str> {
    UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>()
}

pub fn solve_equation(equation: &str) -> Vec<Pos2> {
    is_function(create_character_vec(equation));

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
        assert_eq!(is_function(characters), true);

        characters = create_character_vec("12345");
        assert_eq!(is_function(characters), false);

        characters = create_character_vec("test");
        assert_eq!(is_function(characters), false);
    }
    #[test]
    fn test_is_variable() {
        let mut characters: Vec<&str> = create_character_vec("abc");
        assert_eq!(is_variable(characters), true);

        characters = create_character_vec("3abc");
        assert_eq!(is_variable(characters), false);
    
        characters = create_character_vec("12345");
        assert_eq!(is_variable(characters), false);

        characters = create_character_vec("test()");
        assert_eq!(is_variable(characters), false);
    }
}
