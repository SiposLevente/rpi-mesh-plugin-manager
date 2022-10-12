#[derive(Debug, PartialEq)]
pub enum PluginType {
    Collection,
    Repo,
    Local,
}

#[derive(Debug)]
pub struct Plugin {
    name: String,
    enabled: bool,
    plugin_type: PluginType,
    location: String,
}

impl Plugin {
    pub fn new(name: String, enabled: bool, plugin_type: PluginType, location: String) -> Plugin {
        Plugin {
            name,
            enabled,
            plugin_type,
            location,
        }
    }
}
