use std::fs;
use std::cmp;

/// Stores the data for the sheet
pub struct SheetData {
    pub file_path: String,
    sheet: Vec<Vec<String>>,
    pub selected: Option<(usize, usize)>, // (y, x)
    pub unsaved: bool
}

impl SheetData {
    pub fn new() -> SheetData {
        SheetData {
            file_path: "new_file".to_string(),
            sheet: Vec::new(),
            selected: Some((0, 0)),
            unsaved: true
        }
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
        self.sheet.clear();
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
        // So far, the file is "saved" (may be modified by loading, but saving should do nothing currently)
        self.unsaved = false;
        self.selected = None;
        // Success
        true
    }
    /// Load a vector literal
    pub fn load_vector(&mut self, newsheet: &Vec<Vec<String>>) {
        self.file_path = "generated_file".to_string();
        self.sheet = newsheet.clone();
        // So far, the file is unsaved
        self.unsaved = true;
        self.selected = None;
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
    pub fn set_cell_value(&mut self, coords: (usize, usize), newval: String) {
        if !self.in_bounds(coords) {
            return;
        }
        self.sheet[coords.0][coords.1] = newval;
        self.unsaved = true; // Was modified
    }
    /// Set the value of the selected cell
    pub fn set_selected_cell_value(&mut self, newval: String) {
        if self.selected.is_none() {
            return;
        }
        self.set_cell_value(self.selected.unwrap(), newval);
    }
    /// Delete a row at a coordinate
    pub fn delete_row(&mut self, rowcoord: usize) -> bool {
        if rowcoord >= self.bounds().0 || self.bounds().0 <= 1 {
            return false;
        }
        self.sheet.remove(rowcoord);
        if let Some((row, col)) = self.selected {
            if row >= self.bounds().0 {
                self.selected = Some((row - 1, col));
            }
        }
        self.unsaved = true; // Was modified
        true
    }
    /// Delete a column at a coordinate
    pub fn delete_column(&mut self, colcoord: usize) -> bool {
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
        self.unsaved = true; // Was modified
        true
    }
    /// Insert a row at a coordinate
    pub fn insert_row(&mut self, rowcoord: usize) -> bool {
        if rowcoord >= self.bounds().0 {
            return false;
        }
        self.sheet.insert(rowcoord, vec![String::new(); self.bounds().1]);
        self.unsaved = true; // Was modified
        true
    }
    /// Insert a column at a coordinate
    pub fn insert_column(&mut self, colcoord: usize) -> bool {
        if colcoord >= self.bounds().1 {
            return false;
        }
        for row in &mut self.sheet {
            if colcoord >= row.len() {
                continue;
            }
            row.insert(colcoord, String::new());
        }
        self.unsaved = true; // Was modified
        true
    }
    /// Sort a column at a coordinate
    pub fn sort_column(&mut self, colcoord: usize) -> bool {
        // TODO: impl sort range and number-based sort
        if colcoord >= self.bounds().1 {
            return false;
        }
        let mut thiscol: Vec<String> = Vec::new();
        for row in &mut self.sheet {
            if colcoord >= row.len() {
                // TODO: err
                return false; // Cannot sort when not rectangular
            }
            thiscol.push(row[colcoord].clone());
        }
        thiscol.sort();
        for (i, row) in &mut self.sheet.iter_mut().enumerate() {
            row[colcoord] = thiscol[i].clone();
        }
        true
    }
}
