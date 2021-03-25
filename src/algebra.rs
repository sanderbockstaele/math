use unicode_segmentation::UnicodeSegmentation;
use emath::Pos2;
use array2d::Array2D;

const LETTER: [&str; 52] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p"
,"q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M",
"N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];

const OPERATION: [&str; 4] = ["+","-","*","/"];

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
²:
fn is_operation (characters: Vec<&str>) -> bool {
    let mut result : bool = false;
    
    for operation in &OPERATION {
        for character in characters.iter().take_while(|&c| c == operation ) {
           result = true;    
        }
    }

    println!("{}", result);

    return result;
}

fn is_variable(characters: Vec<&str>) -> bool {
    let mut result: bool = false;

    if is_operation(characters) == true {
        result = false;
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
    fn test_create_character_vec(){
        let mut characters: Vec<&str> = create_character_vec("test");
        assert_eq!(characters.len(), 4);

        let characters_vec: Vec<&str> = ["t","e","s","t"].to_vec();
        assert_eq!(characters, characters_vec);
    }

    #[test]
    fn test_is_operation(){
        let mut characters: Vec<&str> = create_character_vec("+");
        assert_eq!(is_operation(characters), true);

        let characters: Vec<&str> = create_character_vec("a");
        assert_eq!(is_operation(characters), false);

        let characters: Vec<&str> = create_character_vec("A");
        assert_eq!(is_operation(characters), false);

        let characters: Vec<&str> = create_character_vec("1");
        assert_eq!(is_operation(characters), false);
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
