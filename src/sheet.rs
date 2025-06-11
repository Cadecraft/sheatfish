use std::cmp;

/// Stores the data for the sheet's cells
pub struct Sheet {
    sheet: Vec<Vec<String>>,
    pub selected: Option<(usize, usize)> // (y, x)
}

impl Sheet {
    /// Create a blank default sheet
    pub fn new() -> Sheet {
        Sheet {
            sheet: vec![vec!["".to_string(); 16]; 16],
            selected: Some((0, 0))
        }
    }
    /// Clear the sheet
    pub fn clear(&mut self) {
        self.sheet.clear();
        self.selected = None;
    }
    /// Load the sheet from a string and return whether successful
    pub fn load_string(&mut self, newstring: String) -> bool {
        // Update the sheet by parsing the string
        // TODO: comma/quote handling
        self.clear();
        let mut bound_width: usize = 0;
        for resline in newstring.split('\n') {
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
        // Make the sheet rectangular, if not already, given the longest row
        for line in &mut self.sheet {
            while line.len() < bound_width {
                line.push(String::new());
            }
        }
        // Success
        true
    }
    /// Generate a string from this sheet
    pub fn generate_string(&mut self) -> String {
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
        res
    }
    /// Load a vector literal
    pub fn load_vector(&mut self, newsheet: &Vec<Vec<String>>) {
        self.clear();
        self.sheet = newsheet.clone();
        self.selected = None;
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
    /// Move the coordinates of the selected cell
    pub fn move_selected_coords(&mut self, delta: (isize, isize)) {
        let Some(selected) = self.selected else {
            // Default to the start if possible
            self.set_selected_coords((0, 0));
            return;
        };
        // TODO: saturating subtract instead (bounds check in set_selected_coords means vim number commands do nothing if saturating)
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
        true
    }
    /// Insert a row at a coordinate
    pub fn insert_row(&mut self, rowcoord: usize) -> bool {
        if rowcoord > self.bounds().0 {
            return false;
        }
        self.sheet.insert(rowcoord, vec![String::new(); self.bounds().1]);
        true
    }
    /// Insert a column at a coordinate
    pub fn insert_column(&mut self, colcoord: usize) -> bool {
        if colcoord > self.bounds().1 {
            return false;
        }
        for row in &mut self.sheet {
            if colcoord > row.len() {
                continue;
            }
            row.insert(colcoord, String::new());
        }
        true
    }
    /// Sort a column at a coordinate
    pub fn sort_column(&mut self, colcoord: usize) -> bool {
        self.sort_column_bounded(colcoord, 0, self.bounds().0 - 1)
    }
    /// Sort the region of a column from rowstart to rowend, inclusive, by string
    pub fn sort_column_bounded(&mut self, colcoord: usize, rowstart: usize, rowend: usize) -> bool {
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
        true
    }
    /// Sort the region of a column from rowstart to rowend, inclusive, by number
    pub fn sort_column_bounded_num(&mut self, colcoord: usize, rowstart: usize, rowend: usize) -> bool {
        // TODO: impl sort range
        if colcoord >= self.bounds().1 || rowstart > rowend || rowend >= self.bounds().0 {
            return false;
        }
        let mut thisregion: Vec<(String, f32)> = Vec::new();
        for row in &mut self.sheet[rowstart..=rowend] {
            if colcoord >= row.len() {
                // TODO: err
                return false; // Cannot sort when not rectangular
            }
            thisregion.push((row[colcoord].clone(), row[colcoord].clone().parse::<f32>().unwrap_or(0.0)));
        }
        // TODO: fix with sort_by: v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        thisregion.sort_by(|k, j| k.1.partial_cmp(&j.1).unwrap()); // Sort by the f32 (number) component
        for (i, row) in &mut self.sheet[rowstart..=rowend].iter_mut().enumerate() {
            row[colcoord] = thisregion[i].0.clone(); // Put the string component
        }
        true
    }
    /// Make this sheet match another sheet
    pub fn set_equal(&mut self, other: &Sheet) {
        self.clear();
        self.selected = other.selected;
        self.sheet = other.sheet.clone();
    }
    /// Copy this sheet
    pub fn clone(&self) -> Sheet {
        // TODO: impl clone and match
        let mut res: Sheet = Sheet::new();
        res.set_equal(&self);
        res
    }
}
