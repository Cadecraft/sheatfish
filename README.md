# Sheatfish

An incredibly simple, lightweight terminal-based spreadsheet/csv editor written in Rust!

Features:

- Easily open, edit, and save .csv files
- Two keybind modes: Simple and Vim

<!-- todo: add image of a sheatfish and a spreadsheet screenshot -->

:warning: Currently a work in progress; not all features are implemented yet

![A sheatfish](sheatfish.jpg "A sheatfish")

UI example:

```
demo_file.csv (10 x 9)
----
        0      1      2      3      4      5      6      7      8
 0      X-VAL  Y-VAL  Z-VAL
 1      0      0      0
 2      1      1      0
 3     [2    ] 4      0
 4      3      9      0
 5      4      16     0
 6
 7
 8
 9
----
(3, 0): 2
```

## Installation

1. Download this source code
2. Build using `cargo`

## Commands (in command prompt)

System

- `quit`/`q` -
Quit

- `edit`/`e` -
Exit the command prompt (return to editing a file)

- `new` - Create a new blank file

- `open {filename or path}`/`e {filename or path}` -
Open a .csv file and start editing

- `save {optional: filename or path}`/`w {optional: filename or path}` -
Save/write to a .csv file; if path not given, save to the current open file

- `config {key} {value}` -
Set a config key (see below)

- `config` -
See all config keys (see below)

Editing

- `nav {column #} {row #}` -
Navigate to the cell at a coordinate

- `delete`/`d` (`"row"`/`"r"` or `"column"`/`"c"`) -
Delete the currently selected row or column (ex. `d c` -> delete the current column)

- `insert`/`o`/`i` (`"row"`/`"r"` or `"column"`/`"c"`) -
Insert ("open") a new row or column before the currently selected row or column (ex. `o c` -> insert a new column)

<!-- TODO: undo commands, undo tree? -->

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

- `[:]` - Exit a file (return to command prompt, ex. `[:][q][enter]` -> quit)

- `[h]`/`[j]`/`[k]`/`[l]` - Navigate left/down/up/right one cell

- `[w]` - Navigate to the next (right) set of filled-in cells

- `[b]` - Navigate to the previous (left) set of filled-in cells

- `[esc]` - Exit insert mode (go into "normal" mode)

- `[c]`/`[i]` - Change the value of a cell

- `[a]` - Append into a cell (add characters at the end)

- `[x]` - Delete the value in the cell

- `[o] [c]` - Insert ("open") a column at the current selection

- `[o] [r]`/`[o] [o]` - Insert ("open") a row at the current selection

- `[d] [c]` - Delete the currently selected column

- `[d] [r]`/`[d] [d]` - Delete a row at the current selection

- `[0]`-`[9]` - Repeat the action (navigation) n times (repeat to type whole numbers, ex. `[2][5][j]` -> move down 25 cells, `[4][d][d]` -> delete 4 rows)

## Config

- `maxcellwidth` -
Max inner width of a cell (integer from 1.., default 5)

- `viewcellswidth` -
Max width of cells to show on screen at once before scrolling (integer from 1.., default 10)

- `viewcellsheight` -
Max height of cells to show on screen at once before scrolling (integer from 1.., default 10)

- `vimmode` -
Set to 1 to use the Vim Mode keybinds (see above) (integer from 0..=1, default 0)

<!-- TODO: config option to save files without trailing commas>
