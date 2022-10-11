mod plugin;
mod plugin_manager;

use std::env;

use plugin_manager::PluginManager;

fn main() {
    let mut plug_manager = PluginManager::new();
    plug_manager.read_config();

    match env::args().nth(1) {
        Some(first_arg) => {
            match first_arg.as_str() {
            "update" => update(&mut plug_manager),
            "install" => install(&plug_manager),
            "upgrade" => upgrade(&plug_manager),

            _ => print_help(),
        }},
        None => print_help(),
    }
}

fn upgrade(plug_manager: &PluginManager) {
    todo!()
}

fn install(plug_manager: &PluginManager) {
    todo!()
}

fn update(plug_manager: &mut PluginManager){
    plug_manager.cache_repos();
}

fn print_help() {
    println!("Usage: bla bla bla");
}
