# statusbar

Statusbar for window managers that use the root windows `WM_TITLE`.

## Usage

Configuration is done via editing the [src/blocks.rs](src/blocks.rs) file and compiling with `cargo build`.

The toplevel [Cargo.toml](Cargo.toml) includes all available blocks by default in the dependencies list, but ones not in use can be removed.

## Adding new blocks

1. Add a new lib crate to the [blocks](blocks) directory.
2. Add crate to the workspace list in [blocks/Cargo.toml](blocks/Cargo.toml).
4. Add the crate to the dependencies list in [Cargo.toml](Cargo.toml).
4. Implement the new block. Follow the example of one of the others in [blocks](blocks). Most of them are very simple.
