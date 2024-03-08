// Imports
pub mod remdata;
pub mod sheetdata;
pub mod configdata;

use std::io;

/*
TODOS:
    - All commands and features
    - File loading
    - Colors
*/

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
fn render(config: &configdata::ConfigData, data: &sheetdata::SheetData) {
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
    if data.selected.is_some() && data.selected_cell().is_some() {
        println!("({}, {}): {}", data.selected.unwrap().0, data.selected.unwrap().1, data.selected_cell().unwrap());
    } else {
        println!("no cell selected");
    }
}

/// Main function
fn main() {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.1.0",
        "2024/03/08",
        true
    );
    println!("SHEATFISH by Cadecraft");
    println!("{}", rem.fmt(false));
    println!("====");
    println!();

    // Initialize data
    let mut config = configdata::ConfigData::new();
    let mut data = sheetdata::SheetData::new();

    // Dbg: load a testing vector
    /*data.load_vector(&vec![
        vec!["xasdfasdfsfs".to_string(), "yaa".to_string(), "z".to_string(), "more".to_string()],
        vec!["u".to_string(), "v".to_string(), "w".to_string(), "even".to_string(), "mas".to_string()],
        vec!["end".to_string()]
    ]);*/

    // Load a blank default vector
    data.load_vector(&vec![vec!["".to_string(); 10]; 10]);

    // Start the command loop cycle
    loop {
        // Render
        render(&config, &data);
        // Get and take action on input
        let mut uin = String::new();
        io::stdin()
            .read_line(&mut uin)
            .expect("ERROR: failed to read line");
        uin = uin.trim().to_string();
        match uin.as_str() {
            "quit" => {
                // Quit
                break;
            },
            "w" => {
                // Up
                data.move_selected((-1, 0));
            },
            "a" => {
                // Left
                data.move_selected((0, -1));
            },
            "s" => {
                // Down
                data.move_selected((1, 0));
            }
            "d" => {
                // Right
                data.move_selected((0, 1));
            },
            _ if uin.starts_with(":") => {
                // Set the cell
                data.set_selected_cell(uin.chars().skip(1).collect());
            },
            _ => {
                // nothing
            }
        }
    }
}
