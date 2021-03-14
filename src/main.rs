mod app;
mod algebra;

extern crate eframe;

use crate::app::App;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::default();
    // eframe::run_native(Box::new(app));

    algebra::solve_equation(&args[1]);
}
