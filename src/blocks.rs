/// Left hand block separator
pub const LEFT: &str = "[";
/// right hand block separator
pub const RIGHT: &str = "]";
/// Element used to join blocks
pub const SEPARATOR: &str = " | ";
/// Element used when a block fails
pub const ERROR: &str = "Error...";

/// A block of output that will appear in the status bar
pub struct StatusBlock<'a> {
    /// A StatusBlock uses a Block to produce the dynamic output by calling the `perform` function
    pub block: &'a dyn block::Block,
    /// A template specifies how a StatusBlock should be rendered in the bar
    /// Use {left} for the left hand separator
    /// Use {right} for the right hand separator
    /// Use {content} for the output of the block
    pub template: &'a str,
    /// The block name
    /// Each StatusBlock must be given a unique name within BLOCKS for templating to work correctly
    pub name: &'a str,
}

/// Slice of StatusBlocks to appear in the status bar
pub const BLOCKS: &'static [&'static StatusBlock] = &[
    &StatusBlock {
        block: &file::FileBlock {
            file_path: "/home/david/.local/share/infod/mnt/rss",
        },
        template: "Articles: {content}",
        name: "rss",
    },
    &StatusBlock {
        block: &file::FileBlock {
            file_path: "/home/david/.local/share/infod/mnt/pacman",
        },
        template: "Updates: {content}",
        name: "pacman",
    },
    &StatusBlock {
        block: &system_resources::CpuBlock,
        template: "CPU: {content}%",
        name: "cpu",
    },
    &StatusBlock {
        block: &volume::PulseVolumeBlock { average: true },
        template: "{content}%",
        name: "volume",
    },
    &StatusBlock {
        block: &date::DateBlock {
            format: "%a %b %e %l:%M",
        },
        template: "{content}",
        name: "date",
    },
];
