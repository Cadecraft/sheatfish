/// Stores the data for the sheet
pub struct SheetData {
    pub file_path: String,
    sheet: Vec<Vec<String>>,
    pub selected: Option<(usize, usize)> // (y, x)
}

impl SheetData {
    pub fn new() -> SheetData {
        SheetData {
            file_path: "no_file".to_string(),
            sheet: Vec::new(),
            selected: Some((0, 0))
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
        // Update the vec
        self.sheet.clear();
        true
    }
    /// Load a vector literal
    pub fn load_vector(&mut self, newsheet: &Vec<Vec<String>>) {
        self.file_path = "generated_file".to_string();
        self.sheet = newsheet.clone();
    }
    /// Move the coordinates of the selected cell
    pub fn move_selected_coords(&mut self, delta: (i32, i32)) {
        if self.selected.is_none() {
            return;
        }
        let curr0: i32 = self.selected.unwrap().0.try_into().unwrap();
        let curr1: i32 = self.selected.unwrap().1.try_into().unwrap();
        if curr0 + delta.0 < 0 || curr1 + delta.1 < 0 {
            return;
        }
        let new0: usize = (curr0 + delta.0).try_into().unwrap();
        let new1: usize = (curr1 + delta.1).try_into().unwrap();
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
}
