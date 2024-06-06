// Imports
pub mod remdata;
pub mod sheetdata;
pub mod configdata;
pub mod render;
pub mod ioutils;
use ioutils::{
    printat, clear, read_key, set_raw_mode, flush
};
use std::{ cmp, io, env };

/*
TODOS:
    - Git ignore editorconfig
    - All commands and features
    - Colors
    - Modified marker (*) next to filename and warning on quit
    - Command line arguments to open files?
    - Zoom features
    - Rerender after commands like save, delete, etc.
    - Refactor the main file
    - Performance lag in large window
    - Icon for the app exe
    - Create a release on GitHub with binaries
*/

/// Main function
fn main() -> io::Result<()> {
    // Initialize REM, introductions
    let rem = remdata::RemData::new(
        "0.2.0",
        "2024/04/13",
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

    // Depends on if there is a command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        // No file passed in: load a blank default vector
        data.load_vector(&vec![vec!["".to_string(); 16]; 16]);
    } else {
        // File passed in: try to load; otherwise, just default vector
        data.load_vector(&vec![vec!["".to_string(); 16]; 16]);
        data.load_file(&args[1]);
    }

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
        flush(&mut stdout)?;

        set_raw_mode(false)?;
        // TODO: refactor: command class
        let mut uin = String::new();
        std::io::stdin().read_line(&mut uin).expect("Failed to read line");
        let mut command: Vec<&str> = uin.trim().split(' ').collect();
        // Remove leading ':' (for vim users)
        if command.len() > 0 && command[0].starts_with(":") {
            command[0] = command[0].strip_prefix(':').unwrap_or(command[0]);
        }
        match command.len() {
            1 => {
                match command[0].trim() {
                    "quit" | "q" => {
                        // Quit
                        if data.unsaved {
                            // Quit confirmation
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?; // TODO: heavy refactoring of this clear section (repeated a lot)
                            println!("You have unsaved changes to this file.");
                            println!("If you want to quit without saving, use \"quit!\" or \"q!\" instead");
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
                        // TODO: implement the same confirmation system if unsaved as with `quit` (cancel with '!')
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
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                            println!("Error saving file.");
                        } else {
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                            println!("Saved file.");
                        }
                    },
                    "filename" => {
                        // Display the filename
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("{}", data.file_path);
                    },
                    "config" => {
                        // Display all the config items
                        for i in (1..8).rev() {
                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                        }
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("{}", config.display());
                    },
                    "sort" => {
                        // Sort
                        data.sort_column(data.selected.unwrap_or((0, 0)).1, &config);
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
                        for i in (1..8).rev() {
                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                        }
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("Unknown command."); // todo: refactor unknown ?
                    }
                }
            },
            2 => {
                match command[0].trim() {
                    "open" | "e" => {
                        // Load the file
                        // TODO: quit confirmation if unsaved (cancel with '!')
                        let load_success = data.load_file(command[1].trim());
                        if !load_success {
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
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
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                            println!("Error saving file.");
                        } else {
                            for i in (1..8).rev() {
                                printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                            }
                            printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                            println!("Saved file.");
                            // TODO: implement the rerender after save (remove *) for all/most commands
                            render::render(&mut config, &data, &mut stdout)?;
                        }
                    },
                    "delete" | "d" => {
                        match command[1].trim() {
                            "row" | "r" => {
                                data.delete_row(data.selected.unwrap_or((0, 0)).0, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.delete_column(data.selected.unwrap_or((0, 0)).1, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            _ => {
                                for i in (1..8).rev() {
                                    printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                                }
                                printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                                println!("Unknown command.");
                            }
                        }
                    },
                    "insert" | "o" | "i" => {
                        match command[1].trim() {
                            "row" | "r" => {
                                data.insert_row(data.selected.unwrap_or((0, 0)).0, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            "column" | "col" | "c" => {
                                data.insert_column(data.selected.unwrap_or((0, 0)).1, &config);
                                // Start control cycle
                                control_cycle(&mut config, &mut data, &mut stdout)?;
                            },
                            _ => {
                                for i in (1..8).rev() {
                                    printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                                }
                                printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                                println!("Unknown command.");
                            }
                        }
                    },
                    _ => {
                        for i in (1..8).rev() {
                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                        }
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("Unknown command.");
                    }
                }
            },
            3 => {
                match command[0].trim() {
                    "nav" | "g" => {
                        // Navigate to a cell (command[2], command[1])
                        data.set_selected_coords((command[2].parse().unwrap_or(0), command[1].parse().unwrap_or(0)));
                        // Start the control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "config" => {
                        // Set a config to a value
                        config.set_value(command[1], command[2].parse().unwrap_or(2));
                        // Display all the config items
                        for i in (1..8).rev() {
                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                        }
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("{}", config.display()); // TODO: CLEAR WHILE PRINTING (ex. prevent historysize: 10 -> 9 show as 90)
                    },
                    "sort" => {
                        // Sort column over region command[1]..=command[2]
                        data.sort_column_bounded(data.selected.unwrap_or((0, 0)).1, command[1].parse().unwrap_or(0), command[2].parse().unwrap_or(data.bounds().0 - 1), &config);
                        // Start control cycle
                        control_cycle(&mut config, &mut data, &mut stdout)?;
                    },
                    "insert" | "o" | "i" => {
                        match command[2].trim() {
                            "post" | "p" => {
                                match command[1].trim() {
                                    "row" | "r" => {
                                        data.insert_row(data.selected.unwrap_or((0, 0)).0 + 1, &config);
                                        // Start control cycle
                                        control_cycle(&mut config, &mut data, &mut stdout)?;
                                    },
                                    "column" | "col" | "c" => {
                                        data.insert_column(data.selected.unwrap_or((0, 0)).1 + 1, &config);
                                        // Start control cycle
                                        control_cycle(&mut config, &mut data, &mut stdout)?;
                                    },
                                    _ => {
                                        for i in (1..8).rev() {
                                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                                        }
                                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                                        println!("Unknown command.");
                                    }
                                }
                            },
                            _ => {
                                for i in (1..8).rev() {
                                    printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                                }
                                printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                                println!("Unknown command.");
                            }
                        }
                    },
                    _ => {
                        for i in (1..8).rev() {
                            printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                        }
                        printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                        println!("Unknown command.");
                    }
                }
            }
            _ => {
                for i in (1..8).rev() {
                    printat(0, (vbottom - vtop + 5 + i) as u16, "                                                                                   ", &mut stdout)?;
                }
                printat(0, (vbottom - vtop + 6) as u16, "", &mut stdout)?;
                println!("Unknown command.");
            }
        }
    }

    io::Result::Ok(())
}

// todo: refactor ?
/// Utility to print an input word
fn print_input_word(config: &mut configdata::ConfigData, data: &mut sheetdata::SheetData, stdout: &mut io::Stdout, inputword: &str) -> io::Result<()> {
    // TODO: move the whole inputword display feature into render (along with bool for isInputting) ?
    let selectedcoords = data.selected.unwrap_or((0, 0));
    let viewheight: usize = config.get_value("viewcellsheight").unwrap_or(10).try_into().unwrap_or(10);
    let vtop: usize = cmp::max(selectedcoords.0.saturating_sub(viewheight / 2), 0);
    let vbottom: usize = cmp::min(vtop + viewheight, data.bounds().0);
    printat(15, (vbottom - vtop + 4) as u16, "                              ", stdout)?;
    printat(15, (vbottom - vtop + 4) as u16, inputword, stdout)?;
    flush(stdout)?;
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
                    crossterm::event::KeyCode::Esc => {
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
                                        data.delete_column(data.selected.unwrap_or((0, 0)).1, &config);
                                    }
                                },
                                'c' if priorcapture == 'o' => {
                                    // Insert a column left
                                    for _i in 0..real_repeat_times {
                                        data.insert_column(data.selected.unwrap_or((0, 0)).1, &config);
                                    }
                                },
                                'C' if priorcapture == 'o' => {
                                    // Insert a column right
                                    for _i in 0..real_repeat_times {
                                        data.insert_column(data.selected.unwrap_or((0, 0)).1 + 1, &config);
                                    }
                                }
                                'd' | 'r' if priorcapture == 'd' => {
                                    // Delete a row
                                    for _i in 0..real_repeat_times {
                                        data.delete_row(data.selected.unwrap_or((0, 0)).0, &config);
                                    }
                                },
                                'o' | 'r' if priorcapture == 'o' => {
                                    // Insert a row left
                                    for _i in 0..real_repeat_times {
                                        data.insert_row(data.selected.unwrap_or((0, 0)).0, &config);
                                    }
                                },
                                'O' | 'R' if priorcapture == 'o' => {
                                    // Insert a row right
                                    for _i in 0..real_repeat_times {
                                        data.insert_row(data.selected.unwrap_or((0, 0)).0 + 1, &config);
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
