use std::collections::HashMap;
use std::fs;

/// Stores the config data
pub struct ConfigData {
    datamap: HashMap<String, i32>,
    savepath: String,
}

impl ConfigData {
    /// Create a new ConfigData with the default parameters
    pub fn new() -> ConfigData {
        // Default config
        let mut res = ConfigData {
            datamap: HashMap::from([
                ("maxcellwidth".to_string(), 5),
                ("vimmode".to_string(), 0),
                ("viewcellswidth".to_string(), 10),
                ("viewcellsheight".to_string(), 10),
                ("historysize".to_string(), 100)
            ]),
            savepath: String::from("sheatfish_config.csv")
        };
        // Try to load from the file
        res.load_from_file();
        res
    }

    /// Load from a file
    pub fn load_from_file(&mut self) -> bool {
        let read_res = fs::read_to_string(&self.savepath);
        if read_res.is_err() {
            return false;
        }
        let res = read_res.unwrap().replace("\r\n", "\n").replace("\r", "\n");
        for resline in res.split('\n') {
            // Parse: "key: value"
            if resline.trim().is_empty() {
                continue;
            }
            let mut thekey: &str = "";
            for (i, term) in resline.split(',').enumerate() {
                if i == 0 {
                    thekey = term.trim();
                }
                else if i == 1 {
                    self.set_value(thekey, term.trim().parse::<i32>().unwrap_or(0));
                }
            }
        }
        true
    }

    /// Save to a file
    pub fn save_to_file(&self) -> bool {
        // Generate the result
        let mut res = String::new();
        for (key, value) in &self.datamap {
            res.push_str(&format!("{}, {}\n", key, value));
        }
        // Open the file
        let write_res = fs::write(&self.savepath, res);
        if write_res.is_err() {
            return false;
        }
        true
    }

    /// Get the config value of a string key
    pub fn get_value(&self, key: &str) -> Option<i32> {
        self.datamap.get(key).copied()
    }

    /// Set the config value of a string key
    pub fn set_value(&mut self, key: &str, val: i32) {
        self.datamap.insert(key.to_string(), val);
        // Save the change to the file, if possible
        self.save_to_file();
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
