use std::{
    collections::HashMap,
    fs::{self, read_dir, OpenOptions},
    io::prelude::*,
    path::Path,
    process::{exit, Command},
};

use fs_extra::{dir::CopyOptions, file::read_to_string};

use crate::plugin::{Plugin, PluginType};

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    config_location: String,
    official_repo_location: String,
    repo_folder_location: String,
    installed_cache_location: String,
    plugin_folder_location: String,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        let config_location = String::from("config.conf");
        let official_repo_location = String::from("plugins.repo");
        let repo_folder_location = String::from("repos");
        let installed_cache_location = String::from(".installed");
        let plugin_folder_location = String::from("plugins");

        if !Path::new(&config_location).is_file() {
            if let Err(e) = fs::File::create(&config_location) {
                println!(
                    "Error! Config file is missing and, it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&official_repo_location).is_file() {
            if let Err(e) = fs::File::create(&official_repo_location) {
                println!(
                    "Error! Repo file is missing and, it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&installed_cache_location).is_file() {
            if let Err(e) = fs::File::create(&installed_cache_location) {
                println!(
                    "Error! Installed file cache is missing and, it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&plugin_folder_location).is_dir() {
            if let Err(e) = fs::create_dir(&plugin_folder_location) {
                println!(
                    "Error! Plugin folder missing and, it cannot be created! {}",
                    e
                )
            }
        }

        if !Path::new(&repo_folder_location).is_dir() {
            if let Err(e) = fs::create_dir(&repo_folder_location) {
                println!(
                    "Error! Repo folder missing and, it cannot be created! {}",
                    e
                )
            }
        }

        let mut plugin_manager = PluginManager {
            plugins: HashMap::new(),
            config_location,
            repo_folder_location,
            official_repo_location,
            installed_cache_location,
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
                        "installed_cache_location" => {
                            if Path::new(&data[1].to_string()).is_file() {
                                self.installed_cache_location = data[1].to_string()
                            }
                        }
                        "official_repo_location" => {
                            if Path::new(&data[1].to_string()).is_file() {
                                self.official_repo_location = data[1].to_string();
                            }
                        }
                        "repo_folder_location" => {
                            if Path::new(&data[1].to_string()).is_file() {
                                self.repo_folder_location = data[1].to_string();
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
        self.read_repos(self.official_repo_location.clone());

        match read_dir(&self.repo_folder_location) {
            Ok(repos) => {
                for repo in repos {
                    match repo {
                        Ok(name) => {
                            self.read_repos(name.path().display().to_string());
                        }
                        Err(e) => println!("Error while gettin repo! Error: {}", &e),
                    }
                }
            }
            Err(e) => println!("Error while gettin repos folder location! Error: {}", &e),
        };
    }

    fn load_into_plugins(&mut self, plugins_string: String) {
        let mut lines = plugins_string.lines();
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
                        let data: Vec<&str> = line.unwrap().split('=').map(|x| x.trim()).collect();

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

    fn read_repos(&mut self, location: String) {
        match fs::read_to_string(location) {
            Ok(i) => {
                self.load_into_plugins(i);
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

    fn run_setup(&self, plugin_location: String) -> bool {
        if let Ok(status) = Command::new(format!("{}/setup.sh", plugin_location)).status() {
            if let Some(code) = status.code() {
                if code == 0 {
                    return true;
                } else {
                    return false;
                }
            }
            println!("Error getting status code from setup script!");
            return false;
        }
        println!("Error running setup script!");
        false
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
                } else {
                    if Path::new(&format!("{}/setup.sh", plugin_path)).is_file() {
                        if !self.run_setup(plugin_path) {
                            print!("Error while running setup script! Pluginin is copied to plugin forder! Please manually install {} plugin if installation is needed!", plugin.get_name());
                        }
                    }
                    self.add_to_installed_cache(plugin.get_name(), true);
                    println!("OK!")
                }
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
                let plugin_path =
                    format!("{}/{}", &self.plugin_folder_location, &plugin.get_name());
                if Path::new(&format!("{}/setup.sh", plugin_path)).is_file() {
                    if !self.run_setup(plugin_path) {
                        print!("Error while running setup script! Pluginin is copied to plugin forder! Please manually install {} plugin if installation is needed!", plugin.get_name());
                    }
                }
                self.add_to_installed_cache(plugin.get_name(), true);
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
        if args.len() < 3 {
            self.upgrade_all(self.get_installed_plugins());
        } else {
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
    }

    pub fn upgrade_local_plugin(&self, plugin: &Plugin) {
        if Path::is_dir(Path::new(&plugin.get_location())) {
            let plugin_path = format!("{}/{}", &self.plugin_folder_location, &plugin.get_name());

            if Path::is_dir(Path::new(&plugin_path)) && plugin.get_name() != "" {
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
                } else {
                    println!("OK!");
                }
            } else {
                println!(
                    "Plugin {} is not installed so it cannot be upgraded!",
                    plugin.get_name()
                );
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
            ))
            .arg("pull")
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

    fn upgrade_all(&self, plugins: Vec<String>) {
        for plugin in plugins {
            if let Some(seclected_plugin) = self.plugins.get(&plugin) {
                print!("Upgrading plugin {}...", plugin);
                match seclected_plugin.get_plugin_type() {
                    PluginType::Local => self.upgrade_local_plugin(seclected_plugin),
                    PluginType::Repo => self.upgrade_git_plugin(seclected_plugin),
                    _ => {
                        println!("Wrong plugin type! Skipping {}", plugin)
                    }
                }
            } else {
                println!("Skipping {}! No such plugin!", plugin);
            }
        }
    }

    fn add_to_installed_cache(&self, plugin_name: String, append: bool) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(append)
            .open(&self.installed_cache_location)
            .expect("Cannot open installed cache!");

        if let Err(e) = writeln!(file, "{}", plugin_name) {
            println!("Cannot write to installed cache! Error: {}", e)
        }
    }

    fn get_installed_plugins(&self) -> Vec<String> {
        read_to_string(&self.installed_cache_location)
            .expect("Cannot read installed cache!")
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .clone()
    }

    pub fn uninstall(&self, args: std::env::Args) {
        let plugins_to_delete: Vec<String> = args.skip(2).collect();
        for plugin in plugins_to_delete {
            if self.plugins.contains_key(&plugin) {
                if let Some(plugint_to_be_uninstalled) = self.plugins.get(&plugin) {
                    let plugin_path = format!(
                        "{}/{}",
                        &self.plugin_folder_location,
                        &plugint_to_be_uninstalled.get_name()
                    );

                    if Path::new(&plugin_path).is_dir() {
                        print!("Uninstalling plugin {}...", plugin);

                        if let Err(e) = fs::remove_dir_all(plugin_path) {
                            println!("Error while uninstalling {}! Error: {}", plugin, e);
                        } else {
                            println!("Ok!");
                            let mut installed_plugins = self.get_installed_plugins();
                            let remove_index = installed_plugins
                                .iter()
                                .position(|x| x == &plugin && x != "")
                                .unwrap();
                            installed_plugins.remove(remove_index);
                            let mut content = String::new();

                            for installed_plugin in installed_plugins {
                                let tmp_plugin: String = installed_plugin;
                                content.push_str(tmp_plugin.as_str());
                            }

                            if let Err(e) = fs::write(&self.installed_cache_location, content) {
                                println!(
                                    "Error while writing to installed plugin cache! Error: {}",
                                    e
                                );
                            }
                        }
                    } else {
                        println!("Plugin {} is not installed!", plugin);
                    }
                } else {
                    println!("Error getting plugin!");
                }
            }
        }
    }

    pub fn update(&mut self) {
        print!("Updating official repo...");
        self.update_repo(&self.official_repo_location);

        match read_dir(&self.repo_folder_location) {
            Ok(repos) => {
                for repo in repos {
                    match repo {
                        Ok(name) => {
                            print!("Updating {} repo...", name.file_name().to_str().unwrap());
                            self.update_repo(&name.path().display().to_string());
                        }
                        Err(e) => println!("Error while gettin repo! Error: {}", &e),
                    }
                }
            }
            Err(e) => println!("Error while gettin repos folder location! Error: {}", &e),
        };
    }

    pub fn update_repo(&self, location: &String) {
        match self.get_remote_from_config(&location){
            Some(remote) => {
                if let Ok(resp) = reqwest::blocking::get(
                    remote,
                ) {
                    if let Ok(text) = resp.text() {
                        let mut file = OpenOptions::new().write(true)
                        .append(false)
                        .open(location)
                        .expect("Cannot open repo for upgrading!");

                        if let Err(e) = writeln!(file, "{}", text) {
                            println!("Cannot write to repo! Error: {}", e)
                        }
                        println!("OK!")
                    } else{
                        println!("Error while getting text from remote repo!");
                    }
                } else{
                    println!("Error while getting remote repoes!");
                }
            },
            None => println!("Skipping! No remote defined in repo!"),
        }
    }

    fn get_remote_from_config(&self, location: &String) -> Option<String> {
        let mut remote_url: Option<String> = None;

        match fs::read_to_string(location) {
            Ok(i) => {
                if i.contains("remote") {
                    for line in i.lines() {
                        let data: Vec<&str> = line.split('=').map(|x| x.trim()).collect();
                        if let "remote" = data[0] {
                            remote_url = Some(data[1].to_string());
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}! Exiting plugin manager!", e);
                exit(1);
            }
        }
        remote_url
    }
}
