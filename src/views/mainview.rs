use egui::Color32;

use crate::models::app::{MyApp, Section};
use std::env;
use std::process::Command;

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

fn generate_string_from_depth(text_to_write: &str, depth: usize) -> String {
    let mut generated_string: String = String::from("-".repeat(depth as usize));
    generated_string.push_str(" ");
    generated_string.push_str(text_to_write);
    generated_string
}

fn display_section(ui: &mut egui::Ui, section: &mut Section, depth: u8) {
    let base_color: u8 = 232;
    let depth_multiplier: u8 = 16;
    let rgb_intensity = base_color - (depth_multiplier * depth);
    let rgb_intensity = rgb_intensity;
    let button = ui.add_sized(
        [ui.available_width(), 40.],
        egui::Button::new(generate_string_from_depth(
            section.name.clone().as_str(),
            depth as usize,
        ))
        .fill(Color32::from_rgb(
            rgb_intensity,
            rgb_intensity,
            rgb_intensity,
        ))
        .wrap(true),
    );
    if button.clicked() {
        section.visible = !section.visible;
    }
    if section.visible {
        if section.subsections.len() > 0 {
            for subsection in &mut section.subsections {
                display_section(ui, subsection, depth + 1);
            }
        }
        for ssh_instruction in &section.ssh_instructions {
            if ui
                .label(generate_string_from_depth(
                    ssh_instruction.name.clone().as_str(),
                    depth as usize,
                ))
                .double_clicked()
            {
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
                    display_section(ui, section, 0);
                }
            });
        },
    );
}
