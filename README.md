# Sheatfish

An incredibly simple terminal-based spreadsheet editor written in Rust!

<!-- todo: add image of a sheatfish and a spreadsheet screenshot -->

:warning: Currently supports .csv files only.

:warning: Currently a work in progress; not yet fully usable

## Commands (in command prompt)

:warning: Currently a work in progress; these commands do not all exist yet

System

- `quit` -
Quit

- `edit` -
Exit the command prompt (return to editing a file)

- `new` - Create a new blank file

- `open {filename or path}` -
Open a .csv file

- `save {filename or path}` -
Save to a .csv file

- `config {key} {value}` -
Set a config key (see below)

Editing

- `nav {column #} {row #}` -
Navigate to the cell at a coordinate

## Keybinds (while editing)

- `[esc]` -
Exit a file (return to command prompt)

- `[arrow keys]` -
Navigate up/left/down/right one cell

- `{literal value}` -
Overwrite the current cell with this new value by pressing enter

- `[enter]` -
Commit the new value to the current cell, or edit the current cell's value if there is no new value

- `[backspace]` -
Delete the last character of the new value, or clear the current cell if there is no new value

## Config

- `maxcellwidth` -
Max inner width of a cell (integer from 1..=50, default 5)
