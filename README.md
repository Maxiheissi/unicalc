# unicalc
## general
a tui general purpose calculator. Supports most classical funktions which are found on a normal calulater. 
Supports binary(0b), hexadecimal(0x) and decimal.

## files
### main.rs 
Handles app initialisation, input via crossterm and application loop.
### app.rs
Handles application backend.
### ui.rs
Handles frontend using ratatui.
### eval.rs
Handles input tokenizing, Abstract Syntax Tree parsing and calculations.
