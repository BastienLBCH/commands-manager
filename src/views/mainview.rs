use crate::models::app::{CommandManagerApp};

pub(crate) fn generate_string_from_depth(text_to_write: &str, depth: usize) -> String {
    let mut generated_string: String = String::from("-".repeat(depth as usize));
    generated_string.push_str(" ");
    generated_string.push_str(text_to_write);
    generated_string
}

pub(crate) fn define_best_text_color(background_color_rgb: [u8; 3]) -> egui::Color32 {
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

pub fn show_home_page(app: &mut CommandManagerApp, ui: &mut egui::Ui) {
    ui.heading(app.app_name.clone());

    ui.separator();

    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                for section in &mut app.sections {
                    section.show(
                        ui,
                        app.rgb_values,
                        app.indentation_amplifier,
                        0
                    )
                }
            });
        },
    );
}


