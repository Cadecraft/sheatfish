/// Stores the config data
pub struct ConfigData {
    pub maxcellwidth: usize
}

impl ConfigData {
    /// Create a new ConfigData with the default parameters
    pub fn new() -> ConfigData {
        // Default config
        ConfigData {
            maxcellwidth: 5
        }
    }
}
