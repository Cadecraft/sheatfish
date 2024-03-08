# Sheatfish

An incredibly simple terminal-based spreadsheet editor written in Rust!

<!-- todo: add image of a sheatfish and a spreadsheet screenshot -->

:warning: Currently supports .csv files only.

:warning: Currently a work in progress; not yet usable

## Commands

:warning: Currently a work in progress; these commands do not all exist yet

- `quit` -
Quit

- `open {file path}` -
Open a .csv file

- `save {file path}` - Save to a .csv file

- `{row #}{column letter}` -
Navigate to a specific cell

- `w`, `a`, `s`, `d` -
Navigate up/left/down/right one cell

- `:{literal value}` -
Overwrite the current cell with a value

- `config {key} {value}` -
Set a config key (see below)

## Config

- `maxcellwidth` - max inner width of a cell (integer from 1..=50, default 5)
