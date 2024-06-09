use std::{ fs, collections::VecDeque };
use crate::configdata::ConfigData;
use crate::sheet::Sheet;

/// Stores the data for managing the sheet's history/file status, and the sheet itself
pub struct SheetData {
    pub file_path: String,
    sheet: Sheet,
    history: VecDeque<Sheet>, // Stack of prior sheet states 
    historyframe: i32, // The current index of history (if equals history length, then at new frame)
    pub unsaved: bool
}

impl SheetData {
    pub fn new() -> SheetData {
        SheetData {
            file_path: "new_file".to_string(),
            sheet: Sheet::new(),
            history: VecDeque::new(),
            historyframe: -1,
            unsaved: true
        }
    }
    /// DBG: Get the history info (length and frame)
    pub fn dbg_get_history_info(&self) -> (usize, i32) {
        (self.history.len(), self.historyframe)
    }
    /// Clear the sheet state
    /// (reset all history; call this everywhere where the sheet is reset BEFORE resetting it)
    fn clear_sheet_state(&mut self) {
        self.sheet.clear();
        self.history.clear();
        self.unsaved = false;
        self.historyframe = -1;
    }
    /// Update a sheet state
    /// (set to unsaved and add in the history; call this everywhere the sheet is changed BEFORE making the change)
    fn update_sheet_state(&mut self, config: &ConfigData) {
        self.unsaved = true;
        // Erase history after the current frame
        if self.historyframe > -1 {
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
        self.unsaved = true;
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
                self.sheet.set_equal(&thissheet);
                true
            }
        }
    }
    /// Redo (move forward in history) and return whether successful
    pub fn redo(&mut self) -> bool {
        self.unsaved = true;
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
                self.sheet.set_equal(&thissheet);
                true
            }
        }
    }
    /// Load a file, return whether successful
    pub fn load_file(&mut self, path: &str) -> bool {
        self.clear_sheet_state();
        self.file_path = path.to_string();
        // Get the file
        let read_res = fs::read_to_string(path);
        if read_res.is_err() {
            return false;
        }
        let res = read_res.unwrap().replace("\r\n", "\n").replace("\r", "\n");
        // Update the sheet
        self.sheet.load_string(res)
    }
    /// Load a vector literal
    pub fn load_vector(&mut self, newsheet: &Vec<Vec<String>>) {
        self.clear_sheet_state();
        self.file_path = "generated_file".to_string();
        self.sheet.load_vector(newsheet);
    }
    /// Save to a file, return whether successful
    pub fn save_file(&mut self, path: &str) -> bool {
        if path == self.file_path && !self.unsaved {
            // Same file, so do not save
            return false; // todo: better error message ("already saved")
        }
        self.file_path = path.to_string();
        // Generate the string
        let res: String = self.sheet.generate_string();
        // Open the file
        let write_res = fs::write(path, res);
        if write_res.is_err() {
            return false;
        }
        // Now the file has been saved
        self.unsaved = false;
        true
    }
    /// Get the selected cell in the current sheet
    pub fn selected(&self) -> Option<(usize, usize)> {
        self.sheet.selected
    }
    /// Call bounds on the current sheet
    pub fn bounds(&self) -> (usize, usize) {
        self.sheet.bounds()
    }
    pub fn in_bounds(&self, coords: (usize, usize)) -> bool {
        self.sheet.in_bounds(coords)
    }
    pub fn cell(&self, coords: (usize, usize)) -> Option<&str> {
        self.sheet.cell(coords)
    }
    pub fn move_selected_coords(&mut self, delta: (isize, isize)) {
        self.sheet.move_selected_coords(delta);
    }
    pub fn set_selected_coords(&mut self, coords: (usize, usize)) {
        self.sheet.set_selected_coords(coords);
    }
    pub fn selected_cell_value(&self) -> Option<&str> {
        self.sheet.selected_cell_value()
    }
    pub fn set_cell_value(&mut self, coords: (usize, usize), newval: String, config: &ConfigData) {
        self.sheet.set_cell_value(coords, newval);
        self.update_sheet_state(config);
    }
    pub fn set_selected_cell_value(&mut self, newval: String, config: &ConfigData) {
        self.sheet.set_selected_cell_value(newval);
        self.update_sheet_state(config);
    }
    pub fn delete_row(&mut self, rowcoord: usize, config: &ConfigData) -> bool {
        self.sheet.delete_row(rowcoord);
        self.update_sheet_state(config);
        true
    }
    pub fn delete_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        self.sheet.delete_column(colcoord);
        self.update_sheet_state(config);
        true
    }
    pub fn insert_row(&mut self, rowcoord: usize, config: &ConfigData) -> bool {
        self.sheet.insert_row(rowcoord);
        self.update_sheet_state(config);
        true
    }
    pub fn insert_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        self.sheet.insert_column(colcoord);
        self.update_sheet_state(config);
        true
    }
    pub fn sort_column(&mut self, colcoord: usize, config: &ConfigData) -> bool {
        self.sheet.sort_column(colcoord);
        self.update_sheet_state(config);
        true
    }
    pub fn sort_column_bounded(&mut self, colcoord: usize, rowstart: usize, rowend: usize, config: &ConfigData) -> bool {
        self.sheet.sort_column_bounded(colcoord, rowstart, rowend);
        self.update_sheet_state(config);
        true
    }
    pub fn sort_column_bounded_num(&mut self, colcoord: usize, rowstart: usize, rowend: usize, config: &ConfigData) -> bool {
        self.sheet.sort_column_bounded_num(colcoord, rowstart, rowend);
        self.update_sheet_state(config);
        true
    }
}
