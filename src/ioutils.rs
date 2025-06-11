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
