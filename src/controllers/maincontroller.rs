use std::env;
use std::process::Command;

use eframe::egui;

use crate::models::app::{CommandManagerApp, Page};
use crate::views::mainview::show_home_page;

pub struct MainController {
    app: CommandManagerApp,
}

impl Default for MainController {
    fn default() -> Self {
        MainController {
            app: CommandManagerApp::default(),
        }
    }
}

impl eframe::App for MainController {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.load_configuration();
        ctx.set_pixels_per_point(self.app.pixels_per_points);
        egui::CentralPanel::default().show(ctx, |ui| match self.app.current_page {
            Page::Home => show_home_page(&mut self.app, ui),
        });
    }
}

impl MainController {
    pub fn execute_command(cmd_args: Vec<String>) {
        let mut args: Vec<String> = Vec::new();
        let shell_to_use: String;
        if env::consts::OS == "windows" {
            args.push(String::from("/C"));
            args.append(&mut cmd_args.clone());
            shell_to_use = String::from("cmd");
            Command::new(shell_to_use)
                .args(args)
                .spawn()
                .expect("Failed to execute command");
        } else {
            args.push(String::from("-c"));
            args.append(&mut cmd_args.clone());
            shell_to_use = String::from("sh");
            Command::new(shell_to_use)
                .args(args)
                .spawn()
                .expect("Failed to execute command");
        }
    }
}

