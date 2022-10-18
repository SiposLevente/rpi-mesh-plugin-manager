mod plugin;
mod plugin_manager;

use std::env::args;

use plugin_manager::PluginManager;

fn main() {
    let mut plug_manager = PluginManager::new();
    plug_manager.cache_repos();
    match args().nth(1) {
        Some(first_arg) => match first_arg.as_str() {
            "install" => install(&mut plug_manager),
            "update" => update(&mut plug_manager),
            "upgrade" => upgrade(&plug_manager),
            "uninstall" => uninstall(&plug_manager),

            _ => print_help(),
        },
        None => print_help(),
    }
}

fn update(plug_manager: &mut PluginManager) {
    plug_manager.update();
}

fn uninstall(plug_manager: &PluginManager) {
    plug_manager.uninstall(args());
}

fn upgrade(plug_manager: &PluginManager) {
    plug_manager.upgrade(args());
}

fn install(plug_manager: &mut PluginManager) {
    plug_manager.install(args());
}

fn print_help() {
    println!("RPi mesh's plugin manager\n\nUSAGE:\n\trpi-mesh-plugin-manager [OPTIONS] [PLUGIN NAME]\n\nCOMMANDS:\n\tinstall - Installs the specified plugin\n\tupdate - Updates repositories\n\tupgrade - Upgrades specific plugins. Upgrades all plugins when none are specified.\n\tuninstall - Uninstalls specified plugin\n\thelp - Displays this text");
}
