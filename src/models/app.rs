use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml::{Table, Value};

pub struct MyApp {
    pub app_name: String,
    pub conf_loaded: bool,
    pub current_page: Page,
    pub sections: Vec<Section>,
    pub pixels_per_points: f32,
    pub indentation_amplifier: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            app_name: String::from("commands-manager"),
            conf_loaded: false,
            current_page: Page::Home,
            sections: vec![],
            pixels_per_points: 1.2,
            indentation_amplifier: 16.,
        }
    }
}

#[derive(Debug)]
pub struct SSHInstructions {
    pub name: String,
    pub command: Vec<String>,
}

#[derive(Debug)]
pub struct Section {
    pub ssh_instructions: Vec<SSHInstructions>,
    pub subsections: Vec<Section>,
    pub name: String,
    pub visible: bool,
}

fn load_section_content_from_configuration_part(
    section_name: &String,
    values: &toml::Table,
) -> Section {
    let mut ssh_instructions: Vec<SSHInstructions> = Vec::new();
    let section_name = section_name;
    let mut section = Section::new(Vec::new());

    section.name = section_name.clone();
    for (key, value) in values.iter() {
        match value {
            Value::Array(commands) => {
                ssh_instructions.push(SSHInstructions {
                    name: key.clone(),
                    command: commands
                        .clone()
                        .into_iter()
                        .map(|individual_argument_to_stringify| {
                            String::from(individual_argument_to_stringify.as_str().unwrap_or(""))
                        })
                        .collect(),
                });
            }
            Value::Table(new_section) => {
                let new_section: Section =
                    load_section_content_from_configuration_part(key, new_section);
                section.subsections.push(new_section);
            }
            _other => continue,
        }
    }
    section.ssh_instructions = ssh_instructions;
    section
}

fn load_configuration_options(app: &mut MyApp, configuration_options: &toml::Table) {
    for (key, value) in configuration_options {
        match key.as_str() {
            "pixels_per_point" => {
                let pixels_per_point_value: f32 = value.as_float().unwrap_or(1.2) as f32;
                app.pixels_per_points = pixels_per_point_value;
            }
            "indentation_amplifier" => {
                let indentation_amplifier: f32 = value.as_float().unwrap_or(16.0) as f32;
                app.indentation_amplifier = indentation_amplifier;
            }
            _other => continue,
        }
    }
}

impl MyApp {
    pub fn load_configuration(&mut self) {
        if !self.conf_loaded {
            let file_path = "Config.toml";
            if !Path::new(file_path).exists() {
                let mut file = File::create(file_path).expect("Impossible to create config file");
                file.write_all("".as_bytes())
                    .expect("Impossible to write default config file content");
            }

            let config_content = fs::read_to_string("config.toml")
                .expect("Impossible to read the file")
                .parse::<Table>()
                .unwrap_or(toml::map::Map::new());

            if config_content.len() > 0 {
                for (section_name, values) in config_content.iter() {
                    if let Some(value) = values.as_table() {
                        if section_name == "commands-manager-configuration" {
                            load_configuration_options(self, value);
                        } else {
                            let mut new_section =
                                load_section_content_from_configuration_part(section_name, value);
                            new_section.name = section_name.clone();
                            self.sections.push(new_section);
                        }
                    }
                }
            }
            self.conf_loaded = true;
        }
    }
}

pub enum Page {
    Home,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

impl Section {
    pub fn new(ssh_instructions: Vec<SSHInstructions>) -> Self {
        Section {
            ssh_instructions,
            subsections: Vec::new(),
            name: String::from("Catégorie"),
            visible: false,
        }
    }
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}
