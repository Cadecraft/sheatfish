use crate::sheetdata;
use crate::configdata;
use std::cmp;

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
pub fn render(config: &mut configdata::ConfigData, data: &sheetdata::SheetData) {
    for _i in 0..20 { println!(); } // todo: better clear
    //crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge);
    // Render sheet title and info
    println!("{}{} ({} x {})", if data.unsaved { "*" } else { "" }, data.file_path, data.bounds().0, data.bounds().1);
    println!("----");

    // Determine sheet bounds
    let viewwidth: usize = config.get_value("viewcellswidth").unwrap_or(10).try_into().unwrap_or(10);
    let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
    let selectedcoords = data.selected.unwrap_or((0, 0));
    let vleft: usize = cmp::max(selectedcoords.1.saturating_sub(viewwidth / 2), 0);
    let vright: usize = cmp::min(vleft + viewwidth, data.bounds().1); // Non-inclusive bound
    let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
    let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);

    // Render column titles
    print!(" {} ", fmt_string_padding("", config.get_value("maxcellwidth").unwrap_or(5).try_into().unwrap_or(5)));
    for col in vleft..vright {
        // TODO: letters or numbers?
        print!(" {} ", fmt_string_padding(&col.to_string(), config.get_value("maxcellwidth").unwrap_or(5).try_into().unwrap_or(5)))
    }
    println!();
    // Render all sheet rows with cells
    // todo: colors
    // TODO: max view width in cells, pan the window around to keep the cursor in the center
    for row in vtop..vbottom {
        // Render row title
        // TODO: letters or numbers?
        print!(" {} ", fmt_string_padding(&row.to_string(), config.get_value("maxcellwidth").unwrap_or(5).try_into().unwrap_or(5)));
        for col in vleft..vright {
            // Get formatted cell value with padding
            let cellval = data.cell((row, col)).unwrap_or("");
            let fmtval = fmt_string_padding(cellval, config.get_value("maxcellwidth").unwrap_or(5).try_into().unwrap_or(5));
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
