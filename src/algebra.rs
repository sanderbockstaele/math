use unicode_segmentation::UnicodeSegmentation;
use emath::Pos2;

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

pub fn solve_equation(equation: &str) -> Vec<Pos2> {
    // 
    let characters = UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>();

    is_function(characters);

    let graph = vec![

    ];

    return graph;
}
