// Imports
pub mod remdata;
pub mod sheetdata;
pub mod sheet;
pub mod configdata;
pub mod render;
pub mod ioutils;
pub mod command;
use ioutils::{
    printat, clear, read_key, set_raw_mode, flush
};
use std::{ cmp, io, env };

/*
TODOS:
    - Git ignore editorconfig
    - All commands and features
    - Colors
    - Zoom features (expand and contract cells from one small width to a bigger one?)
    - Misc. scattered todos
    - Rerender after ALL commands like save, delete, etc.
    - Refactor the main file
    - Performance lag in large window
    - Icon for the app exe
    - Create a release on GitHub with binaries (release build)
    - Fix: tmux rendering issues (tmux on arch on wsl on windows terminal)
*/

/// Main function
fn main() -> io::Result<()> {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.4.0",
        "2024/12/16",
        true
    );
    // First, enable raw mode and create the stdout
    let mut stdout = io::stdout();
    set_raw_mode(true)?;
    clear(&mut stdout)?;
    printat(0, 0, "SHEATFISH by Cadecraft", &mut stdout)?;
    printat(0, 1, &rem.fmt(false), &mut stdout)?;
    printat(0, 2, "====", &mut stdout)?;

    // Initialize data
    let mut config = configdata::ConfigData::new();
    let mut data = sheetdata::SheetData::new();

    // If there is a command line argument, try to load that file
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        data.load_file(&args[1]);
    }

    // Start the command cycle
    loop {
        // TODO: better cycle appearance
        // Disable raw mode for commands
        let selectedcoords = data.selected().unwrap_or((0, 0));
        let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
        let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
        let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);
        // Display
        printat(0, (vbottom - vtop + 5) as u16, "                                                                                   ", &mut stdout)?;
        printat(0, (vbottom - vtop + 5) as u16, "Enter a command (see README.md for commands): ", &mut stdout)?;
        flush(&mut stdout)?;

        set_raw_mode(false)?;
        let mut uin = String::new();
        std::io::stdin().read_line(&mut uin).expect("Failed to read line");
        let user_command = command::Command::from(&uin);
        match user_command.len() {
            1 => {
                match user_command.term(0) {
                    "quit" | "q" => {
                        // Quit
                        if data.unsaved {
                            // Quit confirmation
                            print_status_message(&config, &data, &mut stdout, concat!(
                                "You have unsaved changes to this file.\n",
                                "If you want to quit without saving, ",
                                "use \"quit!\" or \"q!\" instead"
                            ))?;
                        } else {
                            // Able to quit
                            break;
                        }
                    },
                    "quit!" | "q!" => {
                        // Force quit
                        break;
                    }
                    "edit" | "e" => {
                        // Back to editing the file
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "new" => {
                        // New file: load a blank default vector
                        if data.unsaved {
                            // New confirmation
                            print_status_message(&config, &data, &mut stdout, concat!(
                                "You have unsaved changes to this file.\n",
                                "If you want to replace it with a new file without saving, ",
                                "use \"new!\" instead"
                            ))?;
                        } else {
                            // Able to load
                            data.load_vector(&vec![vec!["".to_string(); 16]; 16]);
                            // Start control cycle
                            control_cycle(&mut config, &mut data, &mut stdout)?;
                        }
                    },
                    "new!" => {
                        // New file: load a blank default vector
                        data.load_vector(&vec![vec!["".to_string(); 16]; 16]);
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "save" | "w" => {
                        // Save the file to the same path, if possible
                        // TODO: override not-saving-if-unedited with '!'
                        // TODO: re-render to show updated filename, etc.
                        let save_success = data.save_file(&data.file_path.clone());
                        if !save_success {
                            print_status_message(&config, &data, &mut stdout, "Error saving file.")?;
                        } else {
                            print_status_message(&config, &data, &mut stdout, "Saved file.")?;
                            render::render(&mut config, &data, &mut stdout)?;
                        }
                    },
                    "path" => {
                        // Display the file path (filename with path as entered)
                        print_status_message(&config, &data, &mut stdout, &data.file_path)?;
                    },
                    "config" => {
                        // Display all the config items
                        print_status_message(&config, &data, &mut stdout, &config.display())?;
                    },
                    "sort" => {
                        // Sort
                        data.sort_column(data.selected().unwrap_or((0, 0)).1, &config);
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "undo" | "u" => {
                        // Undo
                        data.undo();
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "redo" | "r" => {
                        // Redo
                        data.redo();
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    _ => {
                        print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                    }
                }
            },
            2 => {
                match user_command.term(0) {
                    "open" | "e" => {
                        // Load the file
                        if data.unsaved {
                            // Load confirmation
                            print_status_message(&config, &data, &mut stdout, concat!(
                                "You have unsaved changes to this file.\n",
                                "If you want to switch to a new file without saving, ",
                                "use \"open!\" or \"e!\" instead"
                            ))?;
                        } else {
                            let load_success = data.load_file(user_command.term(1));
                            if !load_success {
                                print_status_message(&config, &data, &mut stdout, "Error opening file.")?;
                                // TODO: handle error better
                            } else {
                                // Start the control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            }
                        }
                    },
                    "open!" | "e!" => {
                        // Force load the file
                        let load_success = data.load_file(user_command.term(1));
                        if !load_success {
                            print_status_message(&config, &data, &mut stdout, "Error opening file.")?;
                        } else {
                            // Start the control cycle
                            control_cycle(&mut config, &mut data, &mut stdout)?;
                        }
                    },
                    "save" | "w" => {
                        // Save the file
                        let save_success = data.save_file(user_command.term(1));
                        if !save_success {
                            print_status_message(&config, &data, &mut stdout, "Error saving file.")?;
                        } else {
                            print_status_message(&config, &data, &mut stdout, "Saved file.")?;
                            // TODO: implement the rerender after save (remove *) for all/most commands
                            render::render(&mut config, &data, &mut stdout)?;
                        }
                    },
                    "delete" | "d" => {
                        match user_command.term(1) {
                            "row" | "r" => {
                                data.delete_row(data.selected().unwrap_or((0, 0)).0, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.delete_column(data.selected().unwrap_or((0, 0)).1, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            _ => {
                                print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                            }
                        }
                    },
                    "insert" | "o" | "i" => {
                        match user_command.term(1) {
                            "row" | "r" => {
                                data.insert_row(data.selected().unwrap_or((0, 0)).0, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.insert_column(data.selected().unwrap_or((0, 0)).1, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            _ => {
                                print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                            }
                        }
                    },
                    _ => {
                        print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                    }
                }
            },
            3 => {
                match user_command.term(0) {
                    "nav" | "g" => {
                        // Navigate to a cell (command[2], command[1])
                        data.set_selected_coords((user_command.term(2).parse().unwrap_or(0), user_command.term(1).parse().unwrap_or(0)));
                        // Start the control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "config" => {
                        // Set a config to a value
                        config.set_value(user_command.term(1), user_command.term(2).parse().unwrap_or(2));
                        // Display all the config items
                        print_status_message(&config, &data, &mut stdout, &config.display())?;
                    },
                    "sort" => {
                        // Sort column over region command[1]..=command[2]
                        data.sort_column_bounded(data.selected().unwrap_or((0, 0)).1, user_command.term(1).parse().unwrap_or(0), user_command.term(2).parse().unwrap_or(data.bounds().0 - 1), &config);
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "insert" | "o" | "i" => {
                        match user_command.term(2) {
                            "post" | "p" => {
                                match user_command.term(1) {
                                    "row" | "r" => {
                                        data.insert_row(data.selected().unwrap_or((0, 0)).0 + 1, &config);
                                        // Start control cycle
                                        control_cycle(&mut config, &mut data, &mut stdout)?;
                                    },
                                    "column" | "col" | "c" => {
                                        data.insert_column(data.selected().unwrap_or((0, 0)).1 + 1, &config);
                                        // Start control cycle
                                        control_cycle(&mut config, &mut data, &mut stdout)?;
                                    },
                                    _ => {
                                        print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                                    }
                                }
                            },
                            _ => {
                                print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                            }
                        }
                    },
                    _ => {
                        print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
                    }
                }
            }
            _ => {
                print_status_message(&config, &data, &mut stdout, "Unknown command.")?;
            }
        }
    }

    io::Result::Ok(())
}

/// Get the vertical coordinate of the first line below the main sheet (the input line)
/// (Used for printing)
fn vertical_coord_of_input(config: &configdata::ConfigData, data: &sheetdata::SheetData) -> u16 {
    let selectedcoords = data.selected().unwrap_or((0, 0));
    let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
    let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
    let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);
    // TODO: magic number 4
    (vbottom - vtop + 4) as u16
}

// TODO: refactor ?
/// Utility to print an input word
fn print_input_word(config: &configdata::ConfigData, data: &sheetdata::SheetData, stdout: &mut io::Stdout, inputword: &str) -> io::Result<()> {
    // TODO: move the whole inputword display feature into render (along with bool for isInputting) ?
    let vstart = vertical_coord_of_input(config, data);
    // TODO: magic number 15
    printat(15, vstart, "                              ", stdout)?;
    printat(15, vstart, inputword, stdout)?;
    flush(stdout)?;
    io::Result::Ok(())
}

/// Utility to print a status message
fn print_status_message(config: &configdata::ConfigData, data: &sheetdata::SheetData, stdout: &mut io::Stdout, msg: &str) -> io::Result<()> {
    clear_input_region(config, data, stdout)?;
    let vstart = vertical_coord_of_input(config, data);
    printat(0, vstart + 2, msg, stdout)?;
    flush(stdout)?;
    //println!("{}", msg);
    io::Result::Ok(())
}

/// Utility to clear the area below the input line
fn clear_input_region(config: &configdata::ConfigData, data: &sheetdata::SheetData, stdout: &mut io::Stdout) -> io::Result<()> {
    let vstart = vertical_coord_of_input(config, data);
    // TODO: magic number 83, 8
    let clearingstring = &(0..83).map(|_| " ").collect::<String>();
    for i in (1..8).rev() {
        // TODO: more refactoring of this clear section?
        printat(0, vstart + 1 + i, clearingstring, stdout)?;
    }
    printat(0, vstart + 2, "", stdout)?;
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
        let mut priorcapture: char = ' ';
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
                            data.set_selected_cell_value(String::new(), &config); // Cleared; rerender
                        }
                    }
                    crossterm::event::KeyCode::Enter => {
                        // Enter the data if it exists, then move down
                        if !inputword.is_empty() {
                            // Already typed a word: enter it and move down
                            data.set_selected_cell_value(inputword.clone(), &config);
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
                                    print_input_word(config, data, stdout, &inputword)?;
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
                        print_input_word(config, data, stdout, &inputword)?;
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
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Enter => {
                        if insertmode {
                            // Exit insert mode, saving changes to the cell if needed
                            data.set_selected_cell_value(inputword.clone(), &config);
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
                            print_input_word(config, data, stdout, &inputword)?;
                            endinput = false;
                        } else {
                            let real_repeat_times = cmp::max(1, repeat_times as isize);
                            // Normal mode command?
                            match c {
                                ':' => {
                                    // Quit out of the command cycle
                                    return io::Result::Ok(());
                                }
                                'h' => data.move_selected_coords((0, -1 * real_repeat_times)),
                                'j' => data.move_selected_coords((real_repeat_times, 0)),
                                'k' => data.move_selected_coords((-1 * real_repeat_times, 0)),
                                'l' => data.move_selected_coords((0, real_repeat_times)),
                                'x' => data.set_selected_cell_value(String::new(), &config), // Cleared; rerender
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
                                        print_input_word(config, data, stdout, &inputword)?;
                                        endinput = false;
                                        insertmode = true;
                                    } else {
                                        // No cell: do nothing
                                    }
                                },
                                'c' if priorcapture == 'd' => {
                                    // Delete a column
                                    for _i in 0..real_repeat_times {
                                        data.delete_column(data.selected().unwrap_or((0, 0)).1, &config);
                                    }
                                },
                                'c' if priorcapture == 'o' => {
                                    // Insert a column left
                                    for _i in 0..real_repeat_times {
                                        data.insert_column(data.selected().unwrap_or((0, 0)).1, &config);
                                    }
                                },
                                'C' if priorcapture == 'o' => {
                                    // Insert a column right
                                    for _i in 0..real_repeat_times {
                                        data.insert_column(data.selected().unwrap_or((0, 0)).1 + 1, &config);
                                    }
                                }
                                'd' | 'r' if priorcapture == 'd' => {
                                    // Delete a row
                                    for _i in 0..real_repeat_times {
                                        data.delete_row(data.selected().unwrap_or((0, 0)).0, &config);
                                    }
                                },
                                'o' | 'r' if priorcapture == 'o' => {
                                    // Insert a row left
                                    for _i in 0..real_repeat_times {
                                        data.insert_row(data.selected().unwrap_or((0, 0)).0, &config);
                                    }
                                },
                                'O' | 'R' if priorcapture == 'o' => {
                                    // Insert a row right
                                    for _i in 0..real_repeat_times {
                                        data.insert_row(data.selected().unwrap_or((0, 0)).0 + 1, &config);
                                    }
                                },
                                'c' | 'i' => {
                                    // Change the cell's value
                                    if data.selected_cell_value().is_some() {
                                        // Exists: start editing
                                        inputword.clear();
                                        print_input_word(config, data, stdout, &inputword)?;
                                        endinput = false;
                                        insertmode = true;
                                    }
                                },
                                'u' => {
                                    // Undo the last action
                                    for _i in 0..real_repeat_times {
                                        data.undo();
                                    }
                                },
                                'r' => {
                                    // TODO: impl, check this does not conflict with the 'r' ifs above
                                    for _i in 0..real_repeat_times {
                                        data.redo();
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
    //io::Result::Ok(())
}
