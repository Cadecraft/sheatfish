use std::{
    cmp, io, io::Write
};
use crossterm::{
    execute, queue, cursor, terminal, style::{self, Stylize, StyledContent}
};

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

/// Clear the screen
pub fn clear(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::Purge))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    io::Result::Ok(())
}
