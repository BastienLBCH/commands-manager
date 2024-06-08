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
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            app_name: String::from("commands-manager"),
            conf_loaded: false,
            current_page: Page::Home,
            sections: vec![],
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
                    let mut ssh_instructions: std::vec::Vec<SSHInstructions> = vec![];
                    match values.as_table() {
                        None => continue,
                        Some(config_table) => {
                            for (command, arguments) in config_table {
                                match arguments {
                                    Value::Array(args) => {
                                        ssh_instructions.push(SSHInstructions {
                                            name: command.clone(),
                                            command: args
                                                .clone()
                                                .into_iter()
                                                .map(|individual_argument_to_stringify| {
                                                    String::from(
                                                        individual_argument_to_stringify
                                                            .as_str()
                                                            .unwrap_or(""),
                                                    )
                                                })
                                                .collect(),
                                        });
                                    }
                                    _other => continue,
                                }
                            }
                        }
                    }

                    let mut new_section = Section::new(ssh_instructions);
                    new_section.category_name = section_name.clone();
                    self.sections.push(new_section);
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

pub struct SSHInstructions {
    pub name: String,
    pub command: Vec<String>,
}

pub struct Section {
    pub ssh_instructions: Vec<SSHInstructions>,
    pub category_name: String,
    pub visible: bool,
}

impl Section {
    pub fn new(ssh_instructions: Vec<SSHInstructions>) -> Self {
        Section {
            ssh_instructions,
            category_name: String::from("Catégorie"),
            visible: false,
        }
    }
}
