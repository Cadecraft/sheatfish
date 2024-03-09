use crate::sheetdata;
use crate::configdata;

/// Format the inner contents of a cell
fn fmt_string_padding(instr: &str, maxwidth: usize) -> String {
    let mut fmtval: String = String::new();
    for i in 0..maxwidth {
        if i >= instr.len() {
            fmtval.push(' ');
        } else {
            fmtval.push(instr.chars().nth(i).unwrap());
        }
    }
    fmtval
}

/// Render the sheet
pub fn render(config: &configdata::ConfigData, data: &sheetdata::SheetData) {
    println!();
    // Render sheet title and info
    println!("{} ({} x {})", data.file_path, data.bounds().0, data.bounds().1);
    println!("----");
    // Render column titles
    print!(" {} ", fmt_string_padding("", config.maxcellwidth));
    for col in 0..data.bounds().1 {
        // TODO: letters or numbers?
        print!(" {} ", fmt_string_padding(&col.to_string(), config.maxcellwidth))
    }
    println!();
    // Render all sheet rows with cells
    for row in 0..data.bounds().0 {
        // Render row title
        // TODO: letters or numbers?
        print!(" {} ", fmt_string_padding(&row.to_string(), config.maxcellwidth));
        for col in 0..data.bounds().1 {
            // Get formatted cell value with padding
            let cellval = data.cell((row, col)).unwrap_or("");
            let fmtval = fmt_string_padding(cellval, config.maxcellwidth);
            // Render based on user selection
            if data.selected.is_some() && (row, col) == data.selected.unwrap() {
                print!("[{}]", fmtval);
            } else {
                print!(" {} ", fmtval);
            }
        }
        println!();
    }
    println!("----");
    // Render selected cell info
    if data.selected.is_some() && data.selected_cell_value().is_some() {
        println!("({}, {}): {}", data.selected.unwrap().0, data.selected.unwrap().1, data.selected_cell_value().unwrap());
    } else {
        println!("no cell selected");
    }
}
