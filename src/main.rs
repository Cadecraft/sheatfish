// Imports
pub mod remdata;
pub mod sheetdata;
pub mod configdata;
pub mod render;

/*
TODOS:
    - Pan/zoom (only display a 16x16 area by default)
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
fn _read_char() -> char {
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

    // Dbg: load a testing vector/file
    /*data.load_vector(&vec![
        vec!["xasdfasdfsfs".to_string(), "yaa".to_string(), "z".to_string(), "more".to_string()],
        vec!["u".to_string(), "v".to_string(), "w".to_string(), "even".to_string(), "mas".to_string()],
        vec!["end".to_string()]
    ]);*/

    // Load a blank default vector
    data.load_vector(&vec![vec!["".to_string(); 16]; 16]);

    // Start the command cycle
    loop {
        // todo: better cycle appearance
        println!("Enter a command (see README.md for commands):");

        // todo: command class
        // todo: branch logic using match
        let mut uin = String::new();
        std::io::stdin().read_line(&mut uin).expect("Failed to read line");
        let command: Vec<&str> = uin.trim().split(' ').collect();
        match command.len() {
            1 => {
                match command[0].trim() {
                    "quit" => {
                        // Quit
                        break;
                    },
                    "edit" => {
                        // Back to editing the file
                        // Start control cycle
                        control_cycle(&mut config, &mut data);
                    },
                    "new" => {
                        // New file: load a blank default vector
                        data.load_vector(&vec![vec!["".to_string(); 10]; 10]);
                        // Start control cycle
                        control_cycle(&mut config, &mut data);
                    },
                    _ => {
                        println!("Unknown command."); // todo: refactor unknown
                    }
                }
            },
            2 => {
                match command[0].trim() {
                    "open" => {
                        // Load the file
                        let load_success = data.load_file(command[1].trim());
                        if !load_success {
                            println!("Error opening file.");
                        } else {
                            // Start the control cycle
                            control_cycle(&mut config, &mut data);
                        }
                    },
                    "save" => {
                        // Save the file
                        let save_success = data.save_file(command[1].trim());
                        if !save_success {
                            println!("Error saving file.");
                        } else {
                            println!("Saved file.");
                        }
                    },
                    _ => {
                        println!("Unknown command.");
                    }
                }
            },
            3 => {
                match command[0].trim() {
                    "nav" => {
                        // Navigate to a cell (command[2], command[1])
                        data.set_selected_coords((command[2].parse().unwrap_or(0), command[1].parse().unwrap_or(0)));
                        // Start the control cycle
                        control_cycle(&mut config, &mut data);
                    },
                    _ => {
                        println!("Unknown command.");
                    }
                }
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
}

/// Command cycle function
fn control_cycle(config: &mut configdata::ConfigData, data: &mut sheetdata::SheetData) {
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
                crossterm::event::KeyCode::Backspace => {
                    // Delete the last char in inputword if it exists; otherwise, clear the cell
                    if !inputword.is_empty() {
                        inputword.pop();
                        print!("<");
                        endinput = false;
                    } else {
                        data.set_selected_cell_value(String::new()); // Cleared; rerender
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    // todo: change command:
                    // todo: if no inputword, edit curr cell data (set inputword)
                    // Enter the data if it exists, then move down
                    if !inputword.is_empty() {
                        // Already typed a word: enter it and move down
                        data.set_selected_cell_value(inputword.clone());
                        data.move_selected_coords((1, 0));
                    } else {
                        // Did not type a word yet
                        if let Some(cellval) = data.selected_cell_value() {
                            if cellval.is_empty() {
                                // Empty: move down
                                data.move_selected_coords((1, 0));
                            } else {
                                // Not empty: start editing
                                inputword = cellval.to_string();
                                endinput = false;
                            }
                        } else {
                            // No cell: do nothing
                        }
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    // Char c has been typed
                    inputword.push(c);
                    // todo: print properly for displaying
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
