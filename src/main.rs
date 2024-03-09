// Imports
pub mod remdata;
pub mod sheetdata;
pub mod configdata;
pub mod render;

use std::io;
//use crossterm::input;

/*
TODOS:
    - Git ignore editorconfig
    - All commands and features
    - File loading
    - Implement instant input with arrow keys, etc. (ncurses or crossterm)
    - Colors
    - Modified marker (*) next to filename and warning on quit
    - Command line arguments?
    - Zoom features
*/

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
        render::render(&config, &data);
        // Get and take action on input
        //let input = crossterm::input::input(); // todo: crossterm
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
