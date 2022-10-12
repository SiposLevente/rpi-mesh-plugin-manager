use std::{collections::HashMap, fs};

use crate::plugin::{Plugin, PluginType};

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    config_location: String,
    plugin_repo_location: String,
    installed_cache_location: String,
    plugin_folder_location: String,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: HashMap::new(),
            config_location: String::from("config.conf"),
            plugin_repo_location: String::from("./plugins.repo"),
            installed_cache_location: String::from(".installed"),
            plugin_folder_location: String::from("plugins"),
        }
    }

    pub fn read_config(&mut self) {
        match fs::read_to_string(&self.config_location) {
            Ok(i) => {
                for line in i.lines() {
                    let data: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
                    match data[0] {
                        "installed_cache_location" => {
                            self.installed_cache_location = data[1].to_string()
                        }
                        "plugin_repo_location" => self.plugin_repo_location = data[1].to_string(),
                        "plugin_folder_location" => {
                            self.plugin_folder_location = data[1].to_string()
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn cache_repos(&mut self) {
        self.read_repos(self.plugin_repo_location.clone());
        todo!();
    }

    fn read_repos(&mut self, location: String) {
        match fs::read_to_string(location) {
            Ok(i) => {
                let mut lines = i.lines();
                let mut line = lines.next();
                while line != None {
                    if let Some(first_char) = line.unwrap().chars().nth(0) {
                        if first_char == '[' {
                            let name = line.unwrap().replace('[', "").replace(']', "");
                            let mut enabled: bool = false;
                            let mut plugin_type: PluginType = PluginType::Repo;
                            let mut location: String = String::new();

                            line = lines.next();
                            while line != None
                                && line.unwrap().chars().nth(0) != None
                                && line.unwrap().chars().nth(0).unwrap() != '['
                            {
                                let data: Vec<&str> =
                                    line.unwrap().split('=').map(|x| x.trim()).collect();

                                match data[0] {
                                    "enabled" => {
                                        if data[1] == "true" {
                                            enabled = true;
                                        }
                                    }
                                    "type" => match data[1] {
                                        "repo" => plugin_type = PluginType::Repo,
                                        "local" => plugin_type = PluginType::Local,
                                        "collection" => plugin_type = PluginType::Collection,
                                        _ => {}
                                    },
                                    "location" => {
                                        if plugin_type == PluginType::Collection {
                                            self.read_repos(data[1].to_string());
                                        }
                                        location = data[1].to_string();
                                    }
                                    _ => {}
                                }

                                line = lines.next();
                            }

                            if plugin_type != PluginType::Repo {
                                self.plugins.insert(
                                    name.clone(),
                                    Plugin::new(name, enabled, plugin_type, location),
                                );
                            }
                        }
                    };
                    line = lines.next();
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
