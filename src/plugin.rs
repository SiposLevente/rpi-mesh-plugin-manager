#[derive(Debug, PartialEq, Clone)]
pub enum PluginType {
    Collection,
    Repo,
    Local,
}

#[derive(Debug)]
pub struct Plugin {
    name: String,
    plugin_type: PluginType,
    location: String,
}

impl Plugin {
    pub fn new(name: String, plugin_type: PluginType, location: String) -> Plugin {
        Plugin {
            name,
            plugin_type,
            location,
        }
    }

    pub fn get_plugin_type(&self) -> PluginType{
        self.plugin_type.clone()
    }

    pub fn get_location(&self) -> String{
        self.location.clone()
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}
