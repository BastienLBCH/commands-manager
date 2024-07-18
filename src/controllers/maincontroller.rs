use crate::models::app::{CommandManagerApp, Page};
use crate::views::mainview::show_home_page;
use eframe::egui;

pub struct Controller {
    app: CommandManagerApp,
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            app: CommandManagerApp::default(),
        }
    }
}

impl eframe::App for Controller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.load_configuration();
        ctx.set_pixels_per_point(self.app.pixels_per_points);
        egui::CentralPanel::default().show(ctx, |ui| match self.app.current_page {
            Page::Home => show_home_page(&mut self.app, ui),
        });
    }
}
