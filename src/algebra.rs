use unicode_segmentation::UnicodeSegmentation;
use emath::Pos2;
use array2d::Array2D;

const LETTER: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'
,'q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M',
'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

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

fn is_number(characters: Vec<&str>) -> bool {
    let mut iterator = characters.iter().peekable();

    while iterator.peek() != None { 
        for character in LETTER {
            let current_character: &str = &character.to_string();
            if iterator.peek().unwrap() == &&current_character{
                       
            }
        }
    }

    return false;
}

pub fn solve_equation(equation: &str) -> Vec<Pos2> {
    // 
    let characters = UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>();

    is_function(characters);

    let graph = vec![

    ];

    return graph;
}
