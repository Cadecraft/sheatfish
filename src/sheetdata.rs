use std::{ fs, cmp, collections::VecDeque };
use crate::configdata::ConfigData;

/// Stores the data for the sheet
pub struct SheetData {
    pub file_path: String,
    sheet: Vec<Vec<String>>,
    history: VecDeque<Vec<Vec<String>>>, // Stack of prior sheet states
    historyframe: i32, // The current index of history (if equals history length, then at new frame)
    pub selected: Option<(usize, usize)>, // (y, x)
    pub unsaved: bool
}

impl SheetData {
    pub fn new() -> SheetData {
        SheetData {
            file_path: "new_file".to_string(),
            sheet: Vec::new(),
            history: VecDeque::new(),
            historyframe: -1,
            selected: Some((0, 0)),
            unsaved: true
        }
    }
    /// DBG: Get the history info (length and frame)
    pub fn dbg_get_history_info(&self) -> (usize, i32) {
        (self.history.len(), self.historyframe)
    }
    /// Get the sheet bounds (y len, x len)
    /// If the sheet is not rectangular, bounds are based off first row
    pub fn bounds(&self) -> (usize, usize) {
        if self.sheet.len() == 0 {
            return (0, 0);
        }
        (self.sheet.len(), self.sheet[0].len())
    }
    /// Get whether a point is in bounds (precisely, not rectangularly)
    pub fn in_bounds(&self, coords: (usize, usize)) -> bool {
        coords.0 < self.bounds().0 && coords.1 < self.sheet[coords.0].len()
    }
    /// Get the value at a point in the sheet
    pub fn cell(&self, coords: (usize, usize)) -> Option<&str> {
        if !self.in_bounds(coords) {
            return None;
        }
        Some(&self.sheet[coords.0][coords.1])
    }
    /// Clear the sheet state
    /// (reset all history; call this everywhere where the sheet is reset BEFORE resetting it)
    fn clear_sheet_state(&mut self) {
        self.sheet.clear();
        self.history.clear();
        self.unsaved = false;
        self.selected = None;
        self.historyframe = -1;
    }
    /// Update a sheet state
    /// (set to unsaved and add in the history; call this everywhere the sheet is changed BEFORE making the change)
    fn update_sheet_state(&mut self, config: &ConfigData) {
        self.unsaved = true;
        // Erase history after the current frame
        if (self.historyframe > -1) {
            self.history.truncate(self.historyframe as usize + 1);
        } else {
            self.history.clear();
        }
        // Add this after
        self.history.push_back(self.sheet.clone());
        self.historyframe += 1;
        if self.history.len() > config.get_value("historysize").unwrap_or(10).try_into().unwrap_or(0) {
            // Delete from the front
            self.history.pop_front();
            self.historyframe -= 1;
        }
    }
    /// Undo (move back in history) and return whether successful
    pub fn undo(&mut self) -> bool {
        // Save the current state if needed
        if self.historyframe <= 0 {
            return false;
        }
        self.historyframe -= 1;
        match self.history.get(self.historyframe as usize) {
            None => {
                false
            },
            Some(thissheet) => {
                self.sheet = thissheet.clone();
                true
            }
        }
        // TODO: impl better: selection out of bounds? (refactor to store selection in the history too? separate struct for "state"?)
    }
    /// Redo (move forward in history) and return whether successful
    pub fn redo(&mut self) -> bool {
        if self.historyframe >= 0 && self.historyframe as usize >= self.history.len() - 1 {
            return false;
        }
        self.historyframe += 1;
        match self.history.get(self.historyframe as usize) {
            None => {
                self.historyframe -= 1; // Go back
                false
            },
            Some(thissheet) => {
                self.sheet = thissheet.clone();
                true
            }
        }
    }
    /// Load a file, return whether successful
    pub fn load_file(&mut self, path: &str) -> bool {
        self.file_path = path.to_string();
        // Get the file
        let read_res = fs::read_to_string(path);
        if read_res.is_err() {
            return false;
        }
        let res = read_res.unwrap().replace("\r\n", "\n").replace("\r", "\n");
        // Update the sheet by parsing res
        // todo: comma/quote handling
        self.clear_sheet_state(); // TODO: test
        let mut bound_width: usize = 0;
        for resline in res.split('\n') {
            if resline.trim().is_empty() {
                continue;
            }
            self.sheet.push(Vec::new());
            let sheetn = self.sheet.len();
            let mut n: usize = 0;
            for resword in resline.split(',') {
                self.sheet[sheetn - 1].push(resword.trim().to_string());
                n += 1;
            }
            bound_width = cmp::max(bound_width, n);
            // Fill in extra lines
            while n < bound_width {
                self.sheet[sheetn - 1].push(String::new());
                n += 1;
            }
        }
        // TODO: test rectangularization
        // Make the sheet rectangular, if it is not already, given the longest row
        for line in &mut self.sheet {
            while line.len() < bound_width {
                line.push(String::new());
            }
        }
        // Success
        true
    }
    /// Load a vector literal
    pub fn load_vector(&mut self, newsheet: &Vec<Vec<String>>) {
        self.file_path = "generated_file".to_string();
        self.clear_sheet_state(); // TODO: test
        self.sheet = newsheet.clone();
    }
    /// Save to a file, return whether successful
    pub fn save_file(&mut self, path: &str) -> bool {
        if path == self.file_path && !self.unsaved {
            // Same file, so do not save
            return false; // todo: better error message ("already saved")
        }
        self.file_path = path.to_string();
        // Create res by iterating over the sheet
        let mut res: String = String::new();
        for row in &self.sheet {
            let mut first_line = true;
            for col in row {
                if first_line {
                    first_line = false;
                } else {
                    res.push(',');
                    res.push(' ');
                }
                res.push_str(col);
            }
            res.push('\n');
        }
        // Open the file
        let write_res = fs::write(path, res);
        if write_res.is_err() {
            return false;
        }
        // Now the file has been saved
        self.unsaved = false;
        true
    }
    /// Move the coordinates of the selected cell
    pub fn move_selected_coords(&mut self, delta: (isize, isize)) {
        let Some(selected) = self.selected else {
            // Default to the start if possible
            self.set_selected_coords((0, 0));
            return;
        };
        let new0: usize = (selected.0 as isize + delta.0).try_into().unwrap_or(0);
        let new1: usize = (selected.1 as isize + delta.1).try_into().unwrap_or(0);
        self.set_selected_coords((new0, new1));
    }
    /// Set the coordinates of the selected cell
    pub fn set_selected_coords(&mut self, coords: (usize, usize)) {
        if !self.in_bounds(coords) {
            return;
        }
        self.selected = Some(coords);
    }
    /// Get the value of the selected cell
    pub fn selected_cell_value(&self) -> Option<&str> {
        if self.selected.is_none() {
            return None;
        }
        self.cell(self.selected.unwrap())
    }
    /// Set the value of a cell
    pub fn set_cell_value(&mut self, coords: (usize, usize), newval: String, config: &ConfigData) {
        if !self.in_bounds(coords) {
            return;
        }
        self.sheet[coords.0][coords.1] = newval;
        self.update_sheet_state(config);
    }
    /// Set the value of the selected cell
    pub fn set_selected_cell_value(&mut self, newval: String, config: &ConfigData) {
        if self.selected.is_none() {
            return;
        }
        self.set_cell_value(self.selected.unwrap(), newval, config);
    }
    /// Delete a row at a coordinate
    pub fn delete_row(&mut self, rowcoord: usize, config: &ConfigData) -> bool {
        if rowcoord >= self.bounds().0 || self.bounds().0 <= 1 {
            return false;
        }
        self.sheet.remove(rowcoord);
        if let Some((row, col)) = self.selected {
            if row >= self.bounds().0 {
                self.selected = Some((row - 1, col));
            }
        }
        self.update_sheet_state(config);
        true
    }
    /// Delete a column at a coordinate
    pub fn delete_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        if colcoord >= self.bounds().1 || self.bounds().1 <= 1 {
            return false;
        }
        for row in &mut self.sheet {
            if colcoord >= row.len() {
                continue;
            }
            row.remove(colcoord);
        }
        if let Some((row, col)) = self.selected {
            if col >= self.bounds().1 {
                self.selected = Some((row, col - 1));
            }
        }
        self.update_sheet_state(config);
        true
    }
    /// Insert a row at a coordinate
    pub fn insert_row(&mut self, rowcoord: usize, config: &ConfigData) -> bool {
        if rowcoord > self.bounds().0 {
            return false;
        }
        self.sheet.insert(rowcoord, vec![String::new(); self.bounds().1]);
        self.update_sheet_state(config);
        true
    }
    /// Insert a column at a coordinate
    pub fn insert_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        if colcoord > self.bounds().1 {
            return false;
        }
        for row in &mut self.sheet {
            if colcoord > row.len() {
                continue;
            }
            row.insert(colcoord, String::new());
        }
        self.update_sheet_state(config);
        true
    }
    /// Sort a column at a coordinate
    pub fn sort_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        self.sort_column_bounded(colcoord, 0, self.bounds().0 - 1, config)
    }
    /// Sort the region of a column from rowstart to rowend, inclusive
    pub fn sort_column_bounded(&mut self, colcoord: usize, rowstart: usize, rowend: usize, config: &ConfigData) -> bool {
        // TODO: impl sort range and number-based sort
        if colcoord >= self.bounds().1 || rowstart > rowend || rowend >= self.bounds().0 {
            return false;
        }
        let mut thisregion: Vec<String> = Vec::new();
        for row in &mut self.sheet[rowstart..=rowend] {
            if colcoord >= row.len() {
                // TODO: err
                return false; // Cannot sort when not rectangular
            }
            thisregion.push(row[colcoord].clone());
        }
        thisregion.sort();
        for (i, row) in &mut self.sheet[rowstart..=rowend].iter_mut().enumerate() {
            row[colcoord] = thisregion[i].clone();
        }
        self.update_sheet_state(config);
        true
    }
}
