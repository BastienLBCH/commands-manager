mod controllers;
mod models;
mod views;

use controllers::maincontroller::Controller;
// use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Mon application egui MVC",
        options,
        Box::new(|_ctx| Box::new(Controller::default())),
    );
}
