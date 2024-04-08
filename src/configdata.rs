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

    /// Load from a file
    pub fn load_from_file() {
        // TODO: impl
    }

    /// Save to a file
    pub fn save_to_file() {
        // TODO: impl
    }

    /// Get the config value of a string key
    pub fn get_value(&self, key: &str) -> Option<i32> {
        self.datamap.get(key).copied()
    }

    /// Set the config value of a string key
    pub fn set_value(&mut self, key: &str, val: i32) {
        self.datamap.insert(key.to_string(), val);
    }

    /// Get the display of the config data
    pub fn display(&self) -> String {
        let mut res = String::new();
        for (key, value) in &self.datamap {
            res.push_str(&format!("{}: {}\n", key, value));
        }
        res
    }
}
