use eframe::epaint::Color32;
use crate::controllers::maincontroller::MainController;
use crate::models::app::Section;

impl Section {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        base_rgb_values: [u8; 3],
        indentation_amplifier: f32,
        depth: u8,
    ) {
        let depth_multiplier: u8 = 16;
        let mut background_color = base_rgb_values.clone();

        for i in 0..3 {
            // Calculate the depth impact for each RGB value
            let depth_impact: u8 = depth_multiplier * depth;
            if depth_impact > base_rgb_values[i] {
                background_color[i] = 0;
            }
            else {
                background_color[i] = base_rgb_values[i] - (depth_multiplier * depth);
            }
        }

        ui.horizontal(|ui| {
            if depth > 0 {
                ui.add_space(indentation_amplifier);
            }
            ui.vertical(|ui| {
                ui.scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(crate::views::mainview::define_best_text_color(background_color));
                    let button = ui.add_sized(
                        [200., 40.],
                        egui::Button::new(crate::views::mainview::generate_string_from_depth(
                            self.name.clone().as_str(),
                            depth as usize,
                        ))
                            .fill(Color32::from_rgb(
                                background_color[0],
                                background_color[1],
                                background_color[2],
                            ))
                            .wrap(true),
                    );
                    if button.clicked() {
                        self.toggle_visibility();
                    }
                });
                if self.visible {
                    if self.subsections.len() > 0 {
                        for subsection in &mut self.subsections {
                            subsection.show(
                                ui,
                                background_color,
                                indentation_amplifier,
                                depth + 1,
                            );
                        }
                    }
                    for ssh_instruction in &self.ssh_instructions {
                        if ui
                            .label(crate::views::mainview::generate_string_from_depth(
                                ssh_instruction.name.clone().as_str(),
                                depth as usize,
                            ))
                            .double_clicked()
                        {
                            MainController::execute_command(ssh_instruction.command.clone());
                        };
                    }
                }
            });
        });
    }
}