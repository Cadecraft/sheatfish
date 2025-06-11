use std::{ io, io::Write };
use crossterm::{
    execute, queue, cursor, terminal, style::{self, Stylize, StyledContent}
};

/// Read inputted character
pub fn _read_char() -> char {
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

/// Wait for and read an inputted key code from crossterm
pub fn read_key() -> crossterm::event::KeyCode {
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

/// Print plain text at a coordinate
pub fn printat(x: u16, y: u16, contents: &str, stdout: &mut io::Stdout) -> io::Result<()> {
    queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent(contents.reset()))?;
    io::Result::Ok(())
}

/// Print stylized text at a coordinate
pub fn printstyl(x: u16, y: u16, contents: StyledContent<String>, stdout: &mut io::Stdout) -> io::Result<()> {
    queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent(contents))?;
    io::Result::Ok(())
}

/// Purge the screen
// TODO: impl this (to separate Purge from Clear, for perf.)

/// Clear the screen
pub fn clear(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::Purge))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    io::Result::Ok(())
}

/// Switch between raw and normal mode
pub fn set_raw_mode(use_raw_mode: bool) -> io::Result<()> {
    if use_raw_mode {
        terminal::enable_raw_mode()?;
    } else {
        terminal::disable_raw_mode()?;
    }
    io::Result::Ok(())
}

/// Flush the stdout
pub fn flush(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.flush()?;
    io::Result::Ok(())
}

const CLEAR_WIDTH: u16 = 83;

/// Print an input word (the value of the cell currently being edited/viewed)
pub fn print_input_word(vstart: u16, stdout: &mut io::Stdout, inputword: &str) -> io::Result<()> {
    const LEFT_INPUT_WORD_BOUND: u16 = 15;
    let clearing_string: &str = &(0..(CLEAR_WIDTH - LEFT_INPUT_WORD_BOUND)).map(|_| " ").collect::<String>();
    printat(LEFT_INPUT_WORD_BOUND, vstart, clearing_string, stdout)?;
    printat(LEFT_INPUT_WORD_BOUND, vstart, inputword, stdout)?;
    flush(stdout)?;
    io::Result::Ok(())
}

/// Display the command prompt
pub fn print_command_prompt(vstart: u16, stdout: &mut io::Stdout) -> io::Result<()> {
    let clearing_string: &str = &(0..CLEAR_WIDTH).map(|_| " ").collect::<String>();
    printat(0, vstart + 1, clearing_string, stdout)?;
    printat(0, vstart + 1, "Enter a command (see README.md for commands): ", stdout)?;
    flush(stdout)?;
    io::Result::Ok(())
}

/// Clear the area below the command prompt
fn clear_status_region(vstart: u16, stdout: &mut io::Stdout) -> io::Result<()> {
    const CLEAR_HEIGHT: u16 = 8;
    let clearing_string: &str = &(0..CLEAR_WIDTH).map(|_| " ").collect::<String>();
    for i in (1..CLEAR_HEIGHT).rev() {
        printat(0, vstart + 1 + i, clearing_string, stdout)?;
    }
    printat(0, vstart + 2, "", stdout)?;
    io::Result::Ok(())
}

/// Print a status message
pub fn print_status_message(vstart: u16, stdout: &mut io::Stdout, msg: &str) -> io::Result<()> {
    clear_status_region(vstart, stdout)?;
    printat(0, vstart + 2, msg, stdout)?;
    flush(stdout)?;
    io::Result::Ok(())
}
