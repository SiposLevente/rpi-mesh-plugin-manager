mod plugin;
mod plugin_manager;

use std::env::args;

use plugin_manager::PluginManager;

fn main() {
    match args().nth(1) {
        Some(first_arg) => {
            let mut plug_manager = PluginManager::new();
            plug_manager.cache_repos();
            match first_arg.as_str() {
                "install" => install(&mut plug_manager),
                "update" => update(&mut plug_manager),
                "upgrade" => upgrade(&plug_manager),
                "uninstall" => uninstall(&plug_manager),
                "list" => list(&plug_manager),

                _ => print_help(),
            }
        }
        None => print_help(),
    }
}

fn list(plug_manager: &PluginManager) {
    println!("{}", plug_manager.list());
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
    println!("RPi mesh's plugin manager\n\nUSAGE:\n\trpi-mesh-plugin-manager [OPTIONS] [PLUGIN NAME]\n\nCOMMANDS:\n\tinstall\t\t\tInstalls the specified plugin\n\tupdate\t\t\tUpdates repositories\n\tupgrade\t\t\tUpgrades specific plugins. Upgrades all plugins when none are specified.\n\tuninstall\t\tUninstalls specified plugin\n\tlist\t\t\tDisplays a list of available plugins\n\thelp\t\t\tDisplays this text");
}
