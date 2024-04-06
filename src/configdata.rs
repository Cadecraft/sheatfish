use std::collections::HashMap;

/// Stores the config data
pub struct ConfigData {
    datamap: HashMap<String, i32>
}

impl ConfigData {
    /// Create a new ConfigData with the default parameters
    pub fn new() -> ConfigData {
        // Default config
        ConfigData {
            datamap: HashMap::from([
                ("maxcellwidth".to_string(), 5),
                ("vimmode".to_string(), 0)
            ])
        }
    }

    /// Read from a file

    /// Get the config value of a string key
    pub fn get_value(&self, key: &str) -> Option<i32> {
        self.datamap.get(key).copied()
    }

    /// Set the config value of a string key
    pub fn set_value(&mut self, key: &str, val: i32) {
        self.datamap.insert(key.to_string(), val);
    }
}
