use crate::models::app::{MyApp, Section};
use std::env;
use std::process::Command;
// use eframe::egui::{Button, FontId};

fn execute_command(cmd_args: Vec<String>) {
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

fn display_section(ui: &mut egui::Ui, section: &mut Section) {
    let button = ui.add_sized(
        [ui.available_width(), 40.],
        egui::Button::new(section.name.clone()).wrap(true),
    );
    if button.clicked() {
        section.visible = !section.visible;
    }
    if section.visible {
        if section.subsections.len() > 0 {
            for subsection in &mut section.subsections {
                display_section(ui, subsection);
            }
        }
        for ssh_instruction in &section.ssh_instructions {
            if ui.label(ssh_instruction.name.clone()).double_clicked() {
                execute_command(ssh_instruction.command.clone());
            };
        }
    }
}

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
                    display_section(ui, section);
                }
            });
        },
    );
}
