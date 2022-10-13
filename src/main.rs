mod plugin;
mod plugin_manager;

use std::env::args;

use plugin_manager::PluginManager;

fn main() {
    let mut plug_manager = PluginManager::new();

    match args().nth(1) {
        Some(first_arg) => match first_arg.as_str() {
            "install" => install(&mut plug_manager),
            "upgrade" => upgrade(&plug_manager),

            _ => print_help(),
        },
        None => print_help(),
    }
}

fn upgrade(plug_manager: &PluginManager) {
    todo!()
}

fn install(plug_manager: &mut PluginManager) {
    plug_manager.cache_repos();
    plug_manager.install(args());
}


fn print_help() {
    println!("Usage: bla bla bla");
}
