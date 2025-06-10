use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use homedir::my_home;

/// Stores the config data
pub struct ConfigData {
    datamap: HashMap<String, i32>,
    savepath: PathBuf,
}

impl ConfigData {
    /// Create a new ConfigData with the default parameters
    pub fn new() -> ConfigData {
        let mut savepath = match my_home() {
            Ok(Some(homepath)) => homepath,
            _ => PathBuf::from("./")
        };
        savepath.push(".sheatfish_config.csv");
        // Default config
        let mut res = ConfigData {
            datamap: HashMap::from([
                ("maxcellwidth".to_string(), 5),
                ("vimmode".to_string(), 0),
                ("viewcellswidth".to_string(), 10),
                ("viewcellsheight".to_string(), 10),
                ("historysize".to_string(), 100)
            ]),
            savepath
        };
        // Try to load from the file
        res.try_load_from_file();
        res
    }

    /// Load from a file
    pub fn try_load_from_file(&mut self) {
        let read_res = fs::read_to_string(&self.savepath).unwrap_or(String::new());
        let contents = read_res.replace("\r\n", "\n").replace("\r", "\n");
        for configline in contents.split('\n').filter(|x| !x.trim().is_empty()) {
            // Parse: "key, value"
            let mut splitted = configline.split(',');
            if let Some(key) = splitted.next() {
                if let Some(val) = splitted.next() {
                    self.set_value(
                        key.trim(),
                        val.trim().parse::<i32>().unwrap_or(0)
                    );
                }
            }
        }
    }

    /// Save to a file
    pub fn try_save_to_file(&self) {
        // Generate the result
        let mut res = String::new();
        for (key, value) in &self.datamap {
            res.push_str(&format!("{}, {}\n", key, value));
        }
        // Open the file
        let _ = fs::write(&self.savepath, res);
    }

    /// Get the config value of a string key
    pub fn get_value(&self, key: &str) -> Option<i32> {
        self.datamap.get(key).copied()
    }

    /// Set the config value of a string key
    pub fn set_value(&mut self, key: &str, val: i32) {
        self.datamap.insert(key.to_string(), val);
        self.try_save_to_file();
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
