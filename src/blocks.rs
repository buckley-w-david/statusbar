use std::time::Duration;

use crate::signal;

/// Left hand block separator
pub const LEFT: &str = "[";
/// right hand block separator
pub const RIGHT: &str = "]";
/// Element used to join blocks
pub const SEPARATOR: &str = "";
/// Element used when a block fails
pub const ERROR: &str = "Error...";

/// A block of output that will appear in the status bar
pub struct StatusBlock<'a> {
    /// A StatusBlock uses a Resource to produce the dynamic output by calling the `fetch` function
    pub resource: &'a dyn resource::Resource,
    /// A template specifies how a StatusBlock should be rendered in the bar
    /// Use {left} for the left hand separator
    /// Use {right} for the right hand separator
    /// Use {content} for the output of the block
    pub template: &'a str,
    /// The block name
    /// Each StatusBlock must be given a unique name within BLOCKS for templating to work correctly
    pub name: &'a str,
    /// Duration between updates
    /// statusbar will attempt to compensate for slow update loops by sleeping for a smaller duration
    /// based on how long an update actually took
    pub interval: Duration,
    pub signal_handler: &'a dyn signal::SignalHandler,
}

/// Slice of StatusBlocks to appear in the status bar
pub const BLOCKS: &[&StatusBlock] = &[
    &StatusBlock {
        resource: &filesystem::FileResource {
            file_path: "/home/david/.local/share/infod/mnt/rss",
        },
        template: "Articles: {content} | ",
        name: "rss",
        interval: Duration::from_millis(1000),
        signal_handler: &signal::ShHandler {
            code: "kitty newsboat",
        },
    },
    &StatusBlock {
        resource: &filesystem::FileResource {
            file_path: "/home/david/.local/share/infod/mnt/pacman",
        },
        template: "Updates: {content};",
        name: "pacman",
        interval: Duration::from_millis(1000),
        signal_handler: &signal::NoOpHandler,
    },
    &StatusBlock {
        resource: &system_resources::CpuResource,
        template: "Cpu: {content}% | ",
        name: "cpu",
        interval: Duration::from_millis(1000),
        signal_handler: &signal::ShHandler { code: "kitty htop" },
    },
    &StatusBlock {
        resource: &no_resource::NoResource,
        template: "-",
        name: "volume-down",
        interval: Duration::from_secs(86400),
        signal_handler: &signal::ShHandler {
            code: "vol-ctl down",
        },
    },
    &StatusBlock {
        resource: &volume::PulseVolumeResource { average: true },
        template: " {content}% ",
        name: "volume",
        interval: Duration::from_millis(1000),
        signal_handler: &signal::NoOpHandler,
    },
    &StatusBlock {
        resource: &no_resource::NoResource,
        template: "+ | ",
        name: "volume-up",
        interval: Duration::from_secs(86400),
        signal_handler: &signal::ShHandler { code: "vol-ctl up" },
    },
    &StatusBlock {
        resource: &date::DateResource {
            format: "%a %b %e %l:%M",
        },
        template: "{content}",
        name: "date",
        interval: Duration::from_millis(1000),
        signal_handler: &signal::ShHandler { code: "gsimplecal" },
    },
];
