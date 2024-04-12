// Imports
pub mod remdata;
pub mod sheetdata;
pub mod configdata;
pub mod render;
pub mod ioutils;
use ioutils::{
    printat, clear
};
use std::{
    cmp, io, io::Write
};
use crossterm::{
    execute, queue, cursor, terminal, style::{self, Stylize}
};

/*
TODOS:
    - Git ignore editorconfig
    - Render fully in Crossterm for colors and smooth frame transitions
    - All commands and features
    - Colors
    - Modified marker (*) next to filename and warning on quit
    - Command line arguments?
    - Zoom features
*/

// TODO: refactor input functions to another file

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

// TODO: refactor this too

/// Main function
fn main() -> io::Result<()> {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.2.0",
        "2024/04/11",
        true
    );
    // First, enable raw mode and create the stdout
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().expect("ERR: Crossterm could not enable Raw Mode");
    clear(&mut stdout)?;
    printat(0, 0, "SHEATFISH by Cadecraft", &mut stdout)?;
    printat(0, 1, &rem.fmt(false), &mut stdout)?;
    printat(0, 2, "====", &mut stdout)?;
    /*queue!(stdout, cursor::MoveTo(0,0), style::PrintStyledContent("SHEATFISH by Cadecraft".reset()))?;
    println!("SHEATFISH by Cadecraft");
    println!("{}", rem.fmt(false));
    println!("====");
    println!();*/

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
        // Disable raw mode for commands
        let selectedcoords = data.selected.unwrap_or((0, 0));
        let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
        let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
        let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);
        // Display
        printat(0, (vbottom - vtop + 5) as u16, "                                                                                   ", &mut stdout)?;
        printat(0, (vbottom - vtop + 5) as u16, "Enter a command (see README.md for commands): ", &mut stdout)?;
        stdout.flush()?;

        terminal::disable_raw_mode().expect("ERR: Crossterm could not disable Raw Mode");
        // todo: command class
        let mut uin = String::new();
        std::io::stdin().read_line(&mut uin).expect("Failed to read line");
        let command: Vec<&str> = uin.trim().split(' ').collect();
        // TODO: ignore a : at the start of a command (vim thing)
        match command.len() {
            1 => {
                match command[0].trim() {
                    "quit" | "q" => {
                        // Quit
                        // TODO: quit confirmation if unsaved (cancel with '!')
                        break;
                    },
                    "edit" | "e" => {
                        // Back to editing the file
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "new" => {
                        // New file: load a blank default vector
                        data.load_vector(&vec![vec!["".to_string(); 16]; 16]);
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "save" | "w" => {
                        // Save the file to the same path, if possible
                        let save_success = data.save_file(&data.file_path.clone());
                        if !save_success {
                            println!("Error saving file.");
                        } else {
                            println!("Saved file.");
                        }
                    },
                    "config" => {
                        // Display all the config items
                        println!("{}", config.display());
                    },
                    _ => {
                        println!("Unknown command."); // todo: refactor unknown ?
                    }
                }
            },
            2 => {
                match command[0].trim() {
                    "open" | "e" => {
                        // Load the file
                        let load_success = data.load_file(command[1].trim());
                        if !load_success {
                            println!("Error opening file.");
                        } else {
                            // Start the control cycle
                            control_cycle(&mut config, &mut data, &mut stdout)?;
                        }
                    },
                    "save" | "w" => {
                        // Save the file
                        let save_success = data.save_file(command[1].trim());
                        if !save_success {
                            println!("Error saving file.");
                        } else {
                            println!("Saved file.");
                        }
                    },
                    "delete" | "d" => {
                        match command[1].trim() {
                            "row" | "r" => {
                                data.delete_row(data.selected.unwrap_or((0, 0)).0);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.delete_column(data.selected.unwrap_or((0, 0)).1);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            _ => {
                                println!("Unknown command.");
                            }
                        }
                    },
                    "insert" | "o" | "i" => {
                        match command[1].trim() {
                            "row" | "r" => {
                                data.insert_row(data.selected.unwrap_or((0, 0)).0);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.insert_column(data.selected.unwrap_or((0, 0)).1);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
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
            },
            3 => {
                match command[0].trim() {
                    // TODO: row/column deletion, insertion, etc. with key repeating -->
                    "nav" => {
                        // Navigate to a cell (command[2], command[1])
                        data.set_selected_coords((command[2].parse().unwrap_or(0), command[1].parse().unwrap_or(0)));
                        // Start the control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "config" => {
                        // Set a config to a value
                        config.set_value(command[1], command[2].parse().unwrap_or(2));
                    }
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

    io::Result::Ok(())
}

/// Command cycle function
fn control_cycle(config: &mut configdata::ConfigData, data: &mut sheetdata::SheetData, stdout: &mut io::Stdout) -> io::Result<()> {
    loop {
        // Render
        render::render(config, &data, stdout)?;

        // Input loop until a rerender
        let mut inputword: String = String::new();
        let mut insertmode: bool = false;
        let mut priorcapture: char = ' '; // TODO: use
        let mut repeat_times: u32 = 0;
        loop {
            let mut endinput: bool = true;
            // Get and take action on input
            let ink = read_key(); // Read from Crossterm
            if config.get_value("vimmode").unwrap_or(0) == 0 {
                // NORMAL MODE KEYBINDS
                match ink {
                    crossterm::event::KeyCode::Esc => {
                        // Quit out of the command cycle
                        return io::Result::Ok(());
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
                        // Null or irrelevant key: do nothing
                        endinput = false;
                    }
                }
            } else {
                // VIM MODE KEYBINDINGS
                // TODO: impl all
                match ink {
                    crossterm::event::KeyCode::Esc => {
                        if insertmode {
                            // Exit insert mode, saving changes to the cell if needed
                            data.set_selected_cell_value(inputword.clone());
                            inputword.clear();
                            insertmode = false;
                            endinput = true;
                        }
                    },
                    crossterm::event::KeyCode::Char(c) => {
                        // Char c has been typed
                        if insertmode {
                            // Insert this character
                            inputword.push(c);
                            // todo: print properly for displaying
                            print!("{}", c); 
                            endinput = false;
                        } else {
                            let real_repeat_times = cmp::max(1, repeat_times as isize);
                            // Normal mode command?
                            // TODO: switch based on the character c (normal mode)
                            match c {
                                ':' => {
                                    // Quit out of the command cycle
                                    return io::Result::Ok(());
                                }
                                'h' => data.move_selected_coords((0, -1 * real_repeat_times)),
                                'j' => data.move_selected_coords((real_repeat_times, 0)),
                                'k' => data.move_selected_coords((-1 * real_repeat_times, 0)),
                                'l' => data.move_selected_coords((0, real_repeat_times)),
                                'x' => data.set_selected_cell_value(String::new()), // Cleared; rerender
                                'd' | 'o' if priorcapture != 'd' && priorcapture != 'o' => {
                                    // Delete or open: followed by a 'c', 'd', 'o', or 'r', so do not exit yet
                                    priorcapture = c;
                                    endinput = false;
                                }
                                'a' => {
                                    // Append
                                    if let Some(cellval) = data.selected_cell_value() {
                                        // Exists: start editing
                                        inputword = cellval.to_string();
                                        endinput = false;
                                        insertmode = true;
                                    } else {
                                        // No cell: do nothing
                                    }
                                },
                                'c' if priorcapture == 'd' => {
                                    // Delete a column
                                    for _i in 0..real_repeat_times {
                                        data.delete_column(data.selected.unwrap_or((0, 0)).1);
                                    }
                                },
                                'c' if priorcapture == 'o' => {
                                    // Insert a column
                                    for _i in 0..real_repeat_times {
                                        data.insert_column(data.selected.unwrap_or((0, 0)).1);
                                    }
                                },
                                'd' | 'r' if priorcapture == 'd' => {
                                    // Delete a row
                                    for _i in 0..real_repeat_times {
                                        data.delete_row(data.selected.unwrap_or((0, 0)).0);
                                    }
                                },
                                'o' | 'r' if priorcapture == 'o' => {
                                    // Insert a row
                                    for _i in 0..real_repeat_times {
                                        data.insert_row(data.selected.unwrap_or((0, 0)).0);
                                    }
                                },
                                'c' | 'i' => {
                                    // Change the cell's value
                                    if data.selected_cell_value().is_some() {
                                        // Exists: start editing
                                        inputword.clear();
                                        endinput = false;
                                        insertmode = true;
                                    }
                                },
                                '0'..='9' => {
                                    repeat_times *= 10;
                                    repeat_times += char::to_digit(c, 10).unwrap_or(0);
                                    endinput = false;
                                },
                                _ => {
                                    // Irrelevant character: do nothing
                                    endinput = false;
                                }
                            }
                        }
                    }
                    _ => {
                        // Null or irrelevant key: do nothing
                        endinput = false;
                    }
                }
            }
            // End input if necessary (triggers ending stuff and rerender)
            if endinput {
                break;
            }
        }
    }

    // Finished successfully
    io::Result::Ok(())
}
