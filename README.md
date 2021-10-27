# statusbar

Statusbar for window managers that use the root windows `WM_TITLE`.

Inspired by:
 - [dwmblocks](https://github.com/torrinfail/dwmblocks)
 - [dwm-bar](https://github.com/joestandring/dwm-bar)
 - [slstatus](https://git.suckless.org/slstatus)

## Usage

Configuration is done via editing the [src/blocks.rs](src/blocks.rs) file and compiling with `cargo build`.

The toplevel [Cargo.toml](Cargo.toml) includes all available blocks by default in the dependencies list, but ones not in use can be removed.

## Features
 - `date` - Current date/time with configurable format
 - `filesystem`
   - Contents of a file
   - Number of files in a directory
 - `sh` - Output of `sh -c "code"`
 - `system-resources` - Resource utilization of the system
   - CPU Usage
 - `volume` - Volume of default audio device (Currently PulseAudio only)
 - `keyboard-indicators` - Symbols to indicate caps/scroll lock

### statuscmd

Some initial support for statuscmd compatability has been implemented, but since I don't actually use that patch myself it's not very fleshed out. Blocks can be given a signal handler, which will get called when `statusbar` recieved a signal for that blocks index in the blocks array. You can embded the required raw byte in the template string `\x01`, `\x02`, etc.

## Adding new components

1. Add a new lib crate to the [components](components) directory.
2. Add crate to the workspace list in [components/Cargo.toml](components/Cargo.toml).
4. Add the crate to the dependencies list in [Cargo.toml](Cargo.toml).
4. Implement the new components. Follow the example of one of the others in [components](components). Most of them are very simple.
