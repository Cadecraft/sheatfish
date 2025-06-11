pub struct Command {
    terms: Vec<String>
}

impl Command {
    pub fn from(input_line: &str) -> Command {
        let mut terms: Vec<String> =
            input_line.trim().split(' ')
            .map(|v| v.trim().to_string())
            .collect();
        // Remove leading ':' (for vim users)
        if terms.len() > 0 {
            if let Some(noprefix) = terms[0].strip_prefix(':') {
                terms[0] = noprefix.to_string();
            }
        }
        Command { terms }
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Get a term of the command, 0-indexed (will panic if out of bounds)
    pub fn term(&self, i: usize) -> &str {
        &self.terms[i]
    }
}
