use egui::Color32;

use crate::models::app::{CommandManagerApp, Section};
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

fn define_best_text_color(background_color_rgb: [u8; 3]) -> egui::Color32 {
    let mut normalized_rgb_values: [f64; 3] = [0.0, 0.0, 0.0];
    let mut rgb_values_with_gamma_correction: [f64; 3] = [0.0, 0.0, 0.0];

    for i in 0..3 {
        normalized_rgb_values[i] = (background_color_rgb[i] as f64) / 255.0;
        if normalized_rgb_values[i] <= 0.04045 {
            rgb_values_with_gamma_correction[i] = normalized_rgb_values[i] / 12.92;
        } else {
            rgb_values_with_gamma_correction[i] = (normalized_rgb_values[i] + 0.055) / 1.055;
            rgb_values_with_gamma_correction[i] = rgb_values_with_gamma_correction[i].powf(2.4);
        }
    }

    let luminance = 0.2126 * rgb_values_with_gamma_correction[0]
        + 0.7152 * rgb_values_with_gamma_correction[1]
        + 0.0722 * rgb_values_with_gamma_correction[2];

    if luminance > 0.179 {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    }
}

fn display_section(
    ui: &mut egui::Ui,
    section: &mut Section,
    depth: u8,
    indentation_amplifier: f32,
    rgb_values: [u8; 3],
) {
    let depth_multiplier: u8 = 16;
    let mut new_rgb_values = rgb_values.clone();
    for i in 0..3 {
        new_rgb_values[i] = rgb_values[i] - (depth_multiplier * depth);
    }
    ui.horizontal(|ui| {
        if depth > 0 {
            ui.add_space(indentation_amplifier);
        }
        ui.vertical(|ui| {
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(define_best_text_color(new_rgb_values));
                let button = ui.add_sized(
                    [200., 40.],
                    egui::Button::new(generate_string_from_depth(
                        section.name.clone().as_str(),
                        depth as usize,
                    ))
                    .fill(Color32::from_rgb(
                        new_rgb_values[0],
                        new_rgb_values[1],
                        new_rgb_values[2],
                    ))
                    .wrap(true),
                );
                if button.clicked() {
                    section.toggle_visibility();
                }
            });
            if section.visible {
                if section.subsections.len() > 0 {
                    for subsection in &mut section.subsections {
                        display_section(
                            ui,
                            subsection,
                            depth + 1,
                            indentation_amplifier,
                            new_rgb_values,
                        );
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
        });
    });
}

pub fn show_home_page(app: &mut CommandManagerApp, ui: &mut egui::Ui) {
    ui.heading(app.app_name.clone());

    ui.separator();

    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                for section in &mut app.sections {
                    display_section(
                        ui,
                        section,
                        0,
                        app.indentation_amplifier.clone(),
                        app.rgb_values.clone(),
                    );
                }
            });
        },
    );
}
