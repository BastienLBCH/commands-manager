use crate::models::app::MyApp;
use std::env;
use std::process::Command;
// use eframe::egui::{Button, FontId};

pub fn show_home_page(app: &mut MyApp, ui: &mut egui::Ui) {
    app.load_configuration();

    ui.heading(app.app_name.clone());

    ui.separator();

    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                for section in &mut app.sections {
                    let button = ui.add_sized(
                        [ui.available_width(), 40.],
                        egui::Button::new(section.category_name.clone()).wrap(true),
                    );
                    if button.clicked() {
                        section.visible = !section.visible;
                    }
                    if section.visible {
                        for ssh_instruction in &section.ssh_instructions {
                            if ui.label(ssh_instruction.name.clone()).double_clicked() {
                                println!("Running command: {}", ssh_instruction.command);

                                if env::consts::OS == "windows" {
                                    let result = Command::new("cmd")
                                        .args(["/C", ssh_instruction.command.as_str()])
                                        .output()
                                        .expect("failed to execute command");
                                    println!("{:?}", result.stdout);
                                } else {
                                    let result = Command::new("sh")
                                        .arg("-c")
                                        .arg(ssh_instruction.command.clone())
                                        .output()
                                        .expect("Failed to execute command");
                                    println!("{:?}", result.stdout);
                                }
                            };
                        }
                    }
                }
            });
        },
    );
}
