# statusbar

Statusbar for window managers that use the root windows `WM_TITLE`.

Inspired by:
 - [dwmblocks](https://github.com/torrinfail/dwmblocks)
 - [dwm-bar](https://github.com/joestandring/dwm-bar)
 - [slstatus](https://git.suckless.org/slstatus)

## Usage

Configuration is done via editing the [src/blocks.rs](src/blocks.rs) file and compiling with `cargo build`.

The toplevel [Cargo.toml](Cargo.toml) includes all available blocks by default in the dependencies list, but ones not in use can be removed.

## Adding new components

1. Add a new lib crate to the [components](components) directory.
2. Add crate to the workspace list in [components/Cargo.toml](components/Cargo.toml).
4. Add the crate to the dependencies list in [Cargo.toml](Cargo.toml).
4. Implement the new components. Follow the example of one of the others in [components](components). Most of them are very simple.
