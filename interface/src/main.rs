mod app;

use crate::app::App;
use std::env;
use crate::a;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::default();
    // eframe::run_native(Box::new(app));

    algebra::solve_equation(&args[1]);
}
