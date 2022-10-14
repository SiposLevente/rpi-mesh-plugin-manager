use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
    process::{exit, Command},
};

use fs_extra::dir::CopyOptions;

use crate::plugin::{Plugin, PluginType};

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    config_location: String,
    plugin_repo_location: String,
    plugin_folder_location: String,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        let config_location = String::from("config.conf");
        let plugin_repo_location = String::from("plugins.repo");
        let plugin_folder_location = String::from("plugins");

        if !Path::new(&config_location).is_file() {
            if let Err(e) = fs::File::create(&config_location) {
                println!(
                    "Error! Config file is missing and it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&plugin_repo_location).is_file() {
            if let Err(e) = fs::File::create(&plugin_repo_location) {
                println!(
                    "Error! Repo file is missing and it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&plugin_folder_location).is_dir() {
            if let Err(e) = fs::create_dir(&plugin_folder_location) {
                println!(
                    "Error! Plugin folder missing and it cannot be created! {}",
                    e
                )
            }
        }

        let mut plugin_manager = PluginManager {
            plugins: HashMap::new(),
            config_location,
            plugin_repo_location,
            plugin_folder_location,
        };

        plugin_manager.read_config();
        plugin_manager
    }

    pub fn read_config(&mut self) {
        match fs::read_to_string(&self.config_location) {
            Ok(i) => {
                for line in i.lines() {
                    let data: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
                    match data[0] {
                        "plugin_repo_location" => {
                            if Path::new(&data[1].to_string()).is_file() {
                                self.plugin_repo_location = data[1].to_string();
                            }
                        }
                        "plugin_folder_location" => {
                            if Path::new(&data[1].to_string()).is_dir() {
                                self.plugin_folder_location = data[1].to_string();
                            }
                        }
                        _ => {}
                    }
                }
            }

            Err(e) => {
                println!("Error: {}! Exiting plugin manager!", e);
                exit(1);
            }
        }
    }

    pub fn cache_repos(&mut self) {
        self.read_repos(self.plugin_repo_location.clone());
    }

    fn read_repos(&mut self, location: String) {
        match fs::read_to_string(location) {
            Ok(i) => {
                let mut lines = i.lines();
                let mut line = lines.nth(0);
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

                            if plugin_type != PluginType::Collection && enabled {
                                self.plugins
                                    .insert(name.clone(), Plugin::new(name, plugin_type, location));
                            }
                        }
                    };
                    line = lines.next();
                }
            }
            Err(e) => {
                println!("Error: {}! Exiting plugin manager!", e);
                exit(1);
            }
        }
    }

    pub fn install(&self, args: std::env::Args) {
        let plugins_to_install: Vec<String> = args.skip(2).collect();
        for plugin in plugins_to_install {
            if self.plugins.contains_key(&plugin) {
                if let Some(plugint_to_be_installed) = self.plugins.get(&plugin) {
                    print!("Installing plugin {}...", plugin);
                    match plugint_to_be_installed.get_plugin_type() {
                        PluginType::Local => self.install_local_plugin(plugint_to_be_installed),
                        PluginType::Repo => self.install_git_plugin(plugint_to_be_installed),
                        _ => {
                            println!("Wrong plugin type! Skipping {}", plugin)
                        }
                    }
                } else {
                    println!("Error getting plugin!");
                }
            }
        }
    }

    fn install_local_plugin(&self, plugin: &Plugin) {
        if Path::is_dir(Path::new(&plugin.get_location())) {
            let plugin_path = format!("{}/{}", &self.plugin_folder_location, &plugin.get_name());

            if Path::is_dir(Path::new(&plugin_path)) {
                println!("Plugin {} is already installed!", plugin.get_name());
            } else {
                if let Err(e) = fs_extra::dir::copy(
                    plugin.get_location(),
                    &self.plugin_folder_location,
                    &CopyOptions::new(),
                ) {
                    println!(
                        "Error while installing {}! Skipping plugin! Error: {}",
                        plugin.get_name(),
                        e
                    )
                }
                println!("OK!")
            }
        } else {
            println!("Cannot find plugin: {}! Skipping!", plugin.get_name());
        }
    }

    fn install_git_plugin(&self, plugin: &Plugin) {
        let status = Command::new("git")
            .arg("clone")
            .arg(plugin.get_location())
            .arg(format!(
                "{}/{}",
                &self.plugin_folder_location,
                plugin.get_name()
            ))
            .status()
            .expect("Cannot execute git command! Check if it is installed correctly!");

        if let Some(code) = status.code() {
            if code == 0 {
                println!("OK!")
            } else {
                println!(
                    "Git error code: {}! Skipping plugin {}!",
                    code,
                    plugin.get_name()
                );
            }
        }
    }

    pub fn upgrade(&self, args: std::env::Args) {
        let plugins_to_upgrade: Vec<String> = args.skip(2).collect();
        for plugin in plugins_to_upgrade {
            if self.plugins.contains_key(&plugin) {
                if let Some(plugint_to_be_upgraded) = self.plugins.get(&plugin) {
                    print!("Upgrading plugin {}...", plugin);
                    match plugint_to_be_upgraded.get_plugin_type() {
                        PluginType::Local => self.upgrade_local_plugin(plugint_to_be_upgraded),
                        PluginType::Repo => self.upgrade_git_plugin(plugint_to_be_upgraded),
                        _ => {
                            println!("Wrong plugin type! Skipping {}", plugin)
                        }
                    }
                } else {
                    println!("Error getting plugin!");
                }
            }
        }
    }

    pub fn upgrade_local_plugin(&self, plugin: &Plugin) {
        if Path::is_dir(Path::new(&plugin.get_location())) {
            let plugin_path = format!("{}/{}", &self.plugin_folder_location, &plugin.get_name());

            if Path::is_dir(Path::new(&plugin_path)) {
                let mut options = CopyOptions::new();
                options.overwrite = true;
                if let Err(e) = fs_extra::dir::copy(
                    plugin.get_location(),
                    &self.plugin_folder_location,
                    &options,
                ) {
                    println!(
                        "Error while upgrading {}! Skipping plugin! Error: {}",
                        plugin.get_name(),
                        e
                    )
                }else{
                    println!("OK!");
                }

            } else {
                println!("Plugin {} is not installed so it cannot be upgraded!", plugin.get_name());
            }
        } else {
            println!("Cannot find plugin: {}! Skipping!", plugin.get_name());
        }
    }

    pub fn upgrade_git_plugin(&self, plugin: &Plugin) {
        let status = Command::new("git")
            .arg("-C")
            .arg(format!(
                "{}/{}",
                &self.plugin_folder_location,
                plugin.get_name()
            )).arg("pull")
            .status()
            .expect("Cannot execute git command! Check if it is installed correctly!");

        if let Some(code) = status.code() {
            if code == 0 {
                println!("OK!");
            } else {
                println!(
                    "Git error code: {}! Skipping plugin {}!",
                    code,
                    plugin.get_name()
                );
            }
        }
    }
}
