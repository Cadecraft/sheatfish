# Sheatfish

An incredibly lightweight, productive terminal-based spreadsheet/csv editor written in Rust!

![A demo of Sheatfish](./images/SheatfishDemo.gif)

Features:

- Easily open, edit, and save .csv files in your terminal
- Two keybind modes: Simple and Vim
- Easy to configure

## Installation

1. Download this source code
2. Build using `cargo` (`cargo build` or `cargo install --path .` in the directory)

Executable binaries will be added to a release eventually.

Sheatfish should work on all major platforms, and has currently been tested on Windows 10, Windows 11, Debian (WSL2), and Arch (WSL2).
Some rendering issues have been encountered when testing in tmux.

## Commands (in command prompt mode)

### System

- `quit`/`q` -
Quit \*

- `edit`/`e` -
Exit the command prompt (return to editing a file)

- `new` - Create a new blank file \*

- `open {filename or path}`/`e {filename or path}` -
Open a .csv file and start editing (you can also open a file with Sheatfish on the command line by putting its name as the first argument) \*

- `save {optional: filename or path}`/`w {optional: filename or path}` -
Save/write to a .csv file; if path not given, save to the current open file

- `path` -
Display the filename or path of the currently edited file

- `config {key} {value}` -
Set a config key (see below)

- `config` -
See all config keys and their current values (see below)

\* = add a `!` (ex. `quit!` or `q!`) to force this command, discarding current unsaved changes

### Editing

- `nav {column #} {row #}`/`g {column #} {row #}` -
Navigate to the cell at a coordinate

- `delete`/`d` (`"row"`/`"r"` or `"column"`/`"c"`) -
Delete the currently selected row or column (ex. `d c` -> delete the current column)

- `insert`/`o`/`i` (`"row"`/`"r"` or `"column"`/`"c"`) (optional: `post`/`p`) -
Insert ("open") a new row or column before (or after with `post`) the currently selected row or column (ex. `o c` -> insert a new column)
<!-- TODO: more ergonomic command? -->

- `sort` -
Sort the currently selected column
<!-- TODO: options for backwards, row sort, from cell->cell, sort multiple rows by a column, etc. -->
<!-- TODO: numeric sort -->

- `sort {row start} {row end}` -
Sort the currently selected column over the bounds from row start to row end, inclusive

- `undo`/`u` -
Undo the last action (see the `historysize` config option)

- `redo`/`r` -
Redo the last undone action (see the `historysize` config option)

<!-- TODO: command and keybind (vim mode) to add/remove single/double quotes around entries, including in a bulk fashion -->

<!-- TODO: undo tree? -->
<!-- TODO: allow ANY vim command sequence to be typed in to the commands?? -->

<!-- TODO: keybind while in command mode for previous commands, and a command history (up and down arrows) -->

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

<!-- TODO: cut and paste cells -->

### Vim Mode

- `[:]` - Exit a file (return to command prompt, ex. `[:][q][enter]` -> quit)

- `[h]`/`[j]`/`[k]`/`[l]` - Navigate left/down/up/right one cell

<!-- TODO: impl w and b -->

- `[w]` - Navigate to the next (right) set of filled-in cells

- `[b]` - Navigate to the previous (left) set of filled-in cells

- `[c]`/`[i]` - Change the value of a cell to the literal value typed immediately after

- `[esc]`/`[enter]` - Exit insert mode (go into "normal" mode, committing cell changes)

- `[a]` - Append into a cell (add characters at the end)

- `[x]` - Delete the value in the cell

<!-- TODO: cut and paste cells -->

- `[o] [c]` - Insert ("open") a column left of the current selection

- `[o] [C]` - Insert ("open") a column right of the current selection

<!-- TODO: support backspace key -->

<!-- TODO: 0 and gg should go to first column and first row, respectively -->

<!-- TODO: visual block mode -->

- `[o] [r]`/`[o] [o]` - Insert ("open") a row above the current selection

- `[o] [R]`/`[o] [O]` - Insert ("open") a row below the current selection

- `[d] [c]` - Delete the currently selected column

- `[d] [r]`/`[d] [d]` - Delete a row at the current selection

- `[0]`-`[9]` - Repeat the action (navigation) n times (repeat to type whole numbers, ex. `[2][5][j]` -> move down 25 cells, `[4][d][d]` -> delete 4 rows)

- `[u]` - Undo the last action

- `[r]` - Redo the last action
<!-- TODO: add simple evaluation functions (with parentheses/comma nesting), like `=SUM(3, MUL(4-5, 5-5)` adds 3 and the product of cell (4, 5) times cell (5, 5) -->

## Config

The configuration file, `.sheatfish_config.csv`, is stored in your home directory. If this fails, it will instead be placed inside your current working directory.

You can edit the configuration with the `config` command, but since it's a .csv file, you can also edit it using Sheatfish itself!

- `maxcellwidth` -
Max inner width of a cell (integer from 1.., default 5)

- `viewcellswidth` -
Max width of cells to show on screen at once before scrolling (integer from 1.., default 10)

- `viewcellsheight` -
Max height of cells to show on screen at once before scrolling (integer from 1.., default 10)

- `vimmode` -
Set to 1 to use the Vim Mode keybinds (see above) (integer from 0..=1, default 0)

- `historysize` -
Max number of prior states stored for the undo history (integer from 0.., default 100)

<!-- TODO: config option to save files without trailing commas -->

## Example

UI example "screenshot":

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
