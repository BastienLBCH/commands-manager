mod controllers;
mod models;
mod views;

use controllers::maincontroller::Controller;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "",
        options,
        Box::new(|_ctx| Box::new(Controller::default())),
    );
}
