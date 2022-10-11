use std::fs;

use crate::plugin::{Plugin, PluginType};

pub struct PluginManager {
    plugins: Vec<Plugin>,
    config_location: String,
    plugin_repo_location: String,
    cache_location: String,
    installed_cache_location: String,
    plugin_folder_location: String,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: vec![],
            config_location: String::from("config.conf"),
            cache_location: String::from(".cache"),
            plugin_repo_location: String::from("plugins.repo"),
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
                        "cache_location" => self.cache_location = data[1].to_string(),
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
        self.read_repos();
        for plugin in &self.plugins{
            println!("{:?}", plugin);
        }
        todo!()
    }

    fn read_repos(&mut self) {
        match fs::read_to_string(&self.plugin_repo_location) {
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
                                    "location" => location = data[1].to_string(),
                                    _ => {}
                                }

                                line = lines.next();
                            }

                            self.plugins.push(Plugin::new(name, enabled, plugin_type, location));
                        }
                    };
                    line = lines.next();
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
