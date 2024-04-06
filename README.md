# Sheatfish

An incredibly simple, lightweight terminal-based spreadsheet editor written in Rust!

Features:

- Easily open, edit, and save .csv files
- Two keybind modes: Simple and Vim

<!-- todo: add image of a sheatfish and a spreadsheet screenshot -->

:warning: Currently supports .csv files only.

:warning: Currently a work in progress; not yet fully usable

## Commands (in command prompt)

:warning: Currently a work in progress; these commands do not all exist yet

System

- `quit`/`q` -
Quit

- `edit` -
Exit the command prompt (return to editing a file)

- `new` - Create a new blank file

- `open {filename or path}`/`e {filename or path}` -
Open a .csv file

- `save {optional: filename or path}`/`w {optional: filename or path}` -
Save/write to a .csv file; if path not given, save to the current open file

- `config {key} {value}` -
Set a config key (see below)

- `config` -
See all config keys (see below)

Editing

- `nav {column #} {row #}` -
Navigate to the cell at a coordinate

## Keybinds (while editing)

### Simple Mode

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

### Vim Mode

- `[:]` - Exit a file (return to command prompt)

- `[h]`/`[j]`/`[k]`/`[l]` - Navigate left/down/up/right one cell

- `[w]` - Navigate to the next (right) set of filled-in cells

- `[b]` - Navigate to the previous (left) set of filled-in cells

- `[esc]` - Exit insert mode (go into "normal" mode)

- `[c]` - Change the value of a cell

- `[i]` - Insert into a cell (first character)

- `[a]` - Append into a cell (add characters at the end)

- `[x]` - Delete the value in the cell

## Config

- `maxcellwidth` -
Max inner width of a cell (integer from 1..=50, default 5)

- `vimmode` -
Set to 1 to use the Vim Mode keybinds (see above) (integer from 0..=1, default 0)
