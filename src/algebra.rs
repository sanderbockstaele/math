use unicode_segmentation::UnicodeSegmentation;
use prev_iter::PrevPeekable;
use emath::Pos2;

pub fn solve_equation(equation: &str) -> Vec<Pos2> {
    // 
    let characters = UnicodeSegmentation::graphemes(equation, true).collect::<Vec<&str>>();

    let graph = vec![

    ];

    return graph;
}
