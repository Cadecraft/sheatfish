/// Stores the REM data
pub struct RemData {
    pub r: String,
    pub e: String,
    pub m: bool
}

impl RemData {
    /// Create a new RemData with the given parameters
    pub fn new(recent_version: &str, edit_date: &str, morning_star: bool) -> RemData {
        RemData {
            r: recent_version.to_string(),
            e: edit_date.to_string(),
            m: morning_star
        }
    }

    /// Format the RemData
    pub fn fmt(&self, multiline: bool) -> String {
        match multiline {
            true => format!(
                "  R: v{}\n  E: {}\n  M: {}",
                self.r,
                self.e,
                if self.m { "[successful]" } else { "[unsuccessful]" }
            ),
            false => format!(
                "R: v{}, E: {}, M: {}",
                self.r,
                self.e,
                if self.m { "[successful]" } else { "[unsuccessful]" }
            )
        }
    }
}
