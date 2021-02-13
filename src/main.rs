mod algebra;
mod app;

extern crate eframe;

use crate::app::App;

fn main() {
    let app = App::default();
    eframe::run_native(Box::new(app));
}
