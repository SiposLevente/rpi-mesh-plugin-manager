use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone)]
pub enum PluginType {
    Collection,
    Repo,
    Local,
}

impl Display for PluginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let plugin_type;
        match self {
            PluginType::Collection => plugin_type = String::from("Collection"),
            PluginType::Repo => plugin_type = String::from("Repo"),
            PluginType::Local => plugin_type = String::from("Local"),
        }

        write!(f, "{}", plugin_type)
    }
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

    pub fn get_plugin_type(&self) -> PluginType {
        self.plugin_type.clone()
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]\nenabled={}\ntype={}\nlocation={}\n\n",
            self.name,
            self.enabled,
            self.plugin_type.to_string().to_lowercase(),
            self.location
        )
    }
}
