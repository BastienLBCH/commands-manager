use crate::models::app::{MyApp, Page};
use crate::views::mainview::show_home_page;
use eframe::egui;

pub struct Controller {
    app: MyApp,
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            app: MyApp::default(),
        }
    }
}

impl eframe::App for Controller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.app.current_page {
            Page::Home => show_home_page(&mut self.app, ui),
        });
    }
}
