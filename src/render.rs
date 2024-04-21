use crate::sheetdata;
use crate::configdata;
use crate::ioutils::{
    printat, printstyl, clear, set_raw_mode, flush
};
use std::{ cmp, io };
use crossterm::style::Stylize;

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
pub fn render(config: &mut configdata::ConfigData, data: &sheetdata::SheetData, stdout: &mut io::Stdout) -> io::Result<()> {
    // Prep
    set_raw_mode(true)?;
    clear(stdout)?;

    // Render sheet title and info
    // TODO: print filename only (not full path; search backwards by / or \)
    printat(0, 0, &format!("{}{} ({} x {})", if data.unsaved { "*" } else { "" }, data.file_path, data.bounds().0, data.bounds().1), stdout)?;
    printat(0, 1, "----", stdout)?;

    // Determine sheet bounds
    let viewwidth: usize = config.get_value("viewcellswidth").unwrap_or(10).try_into().unwrap_or(10);
    let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
    let selectedcoords = data.selected.unwrap_or((0, 0));
    let vleft: usize = cmp::max(selectedcoords.1.saturating_sub(viewwidth / 2), 0);
    let vright: usize = cmp::min(vleft + viewwidth, data.bounds().1); // Non-inclusive bound
    let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
    let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);

    // Get config info
    let maxcellwidth: u16 = config.get_value("maxcellwidth").unwrap_or(5).try_into().unwrap_or(5);

    // Render row and column titles
    // TODO: more colors
    // TODO: display a warning/error/instructions if the terminal size is too small to fit the whole screen
    let mut xpos: u16 = 1;
    for col in vleft..vright {
        if selectedcoords.1 == col {
            printstyl((maxcellwidth + 2) * xpos, 2, format!("{}", col).dark_cyan(), stdout)?;
        } else {
            printstyl((maxcellwidth + 2) * xpos, 2, format!("{}", col).dark_grey(), stdout)?;
        }
        xpos += 1;
    }
    let mut ypos: u16 = 1;
    for row in vtop..vbottom {
        if selectedcoords.0 == row {
            printstyl(0, ypos + 2, format!("{}", row).dark_cyan(), stdout)?;
        } else {
            printstyl(0, ypos + 2, format!("{}", row).dark_grey(), stdout)?;
        }
        ypos += 1;
    }

    // Render cells
    for row in vtop..vbottom {
        for col in vleft..vright {
            // Do this
            let cellval = data.cell((row, col)).unwrap_or("");
            let fmtval = fmt_string_padding(cellval, maxcellwidth.into());
            // Render based on user selection
            if data.selected.is_some() && (row, col) == data.selected.unwrap() {
                printstyl(
                    // TODO: unwrap handling ?
                    ((maxcellwidth + 2) as usize * (col + 1 - vleft)).try_into().unwrap_or(0),
                    ((row + 3) - vtop).try_into().unwrap_or(0),
                    format!("[{}]", fmtval).cyan(),
                    stdout
                )?;
            } else {
                printstyl(
                    ((maxcellwidth + 2) as usize * (col + 1 - vleft)).try_into().unwrap_or(0),
                    ((row + 3) - vtop).try_into().unwrap_or(0),
                    format!(" {} ", fmtval).reset(),
                    stdout
                )?;
            }
        }
    }

    // TODO: put cursor on the cell ?

    printat(0, (vbottom - vtop + 3) as u16, "----", stdout)?;

    if data.selected.is_some() && data.selected_cell_value().is_some() {
        let selectedstr = format!("({}, {}):", data.selected.unwrap().0, data.selected.unwrap().1);
        printat(0, (vbottom - vtop + 4) as u16, &selectedstr, stdout)?;
        printat(15, (vbottom - vtop + 4) as u16, &format!("{}", data.selected_cell_value().unwrap()), stdout)?;
    } else {
        printat(0, (vbottom - vtop + 4) as u16, "no cell selected", stdout)?;
    }
    printat(0, 2, "", stdout)?;

    // Flush the buffer to finish
    flush(stdout)?;

    // Was successful
    io::Result::Ok(())
}
