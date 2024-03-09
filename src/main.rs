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

/// Read inputted character
fn read_char() -> char {
    if let Ok(crossterm::event::Event::Key(crossterm::event::KeyEvent {
        code: crossterm::event::KeyCode::Char(c),
        kind: crossterm::event::KeyEventKind::Press,
        modifiers: _,
        state: _,
    })) = crossterm::event::read() {
        return c;
    }
    // Could not read the event
    '!'
}

/// Wait for and read an inputted key code
fn read_key() -> crossterm::event::KeyCode {
    // Read the event
    match crossterm::event::read() {
        // Only return a code on the Ok key event
        Ok(crossterm::event::Event::Key(k)) => {
            // Only return if the key is pressed
            if k.kind == crossterm::event::KeyEventKind::Press {
                k.code
            } else {
                crossterm::event::KeyCode::Null
            }
        }
        _ => {
            crossterm::event::KeyCode::Null
        }
    }
}

/// Main function
fn main() {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.1.1",
        "2024/03/09",
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
    command_cycle(&mut config, &mut data);
}

/// Command cycle function
fn command_cycle(config: &mut configdata::ConfigData, data: &mut sheetdata::SheetData) {
    loop {
        // Render
        render::render(&config, &data);

        // Input loop until a rerender
        let mut inputword: String = String::new();
        loop {
            let mut endinput: bool = true;
            // Get and take action on input
            let ink = read_key(); // Read from Crossterm
            match ink {
                crossterm::event::KeyCode::Esc => {
                    // Quit out of the command cycle
                    return;
                }
                crossterm::event::KeyCode::Up => data.move_selected_coords((-1, 0)),
                crossterm::event::KeyCode::Left => data.move_selected_coords((0, -1)),
                crossterm::event::KeyCode::Down => data.move_selected_coords((1, 0)),
                crossterm::event::KeyCode::Right => data.move_selected_coords((0, 1)),
                crossterm::event::KeyCode::Enter => {
                    // Enter the data, if it exists, and move down
                    if inputword.chars().count() > 0 {
                        data.set_selected_cell_value(inputword.clone());
                    }
                    data.move_selected_coords((1, 0));
                }
                crossterm::event::KeyCode::Char(c) => {
                    // Char c has been typed
                    inputword.push(c);
                    print!("{}", c);
                    endinput = false;
                }
                _ => {
                    // Null or irrelevant code: do nothing
                    endinput = false;
                }
            }
            // End input if necessary (triggers ending stuff and rerender)
            if endinput {
                break;
            }
        }
    }
}
