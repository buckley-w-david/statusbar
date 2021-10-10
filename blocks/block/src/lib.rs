use std::error::Error;

/// A block is a resource to query information from for use in the status bar
pub trait Block {
    /// The perform function performs the blocks query.
    /// It should not return any extra formatting aside from the data itself.
    /// For example a volume block should not include a '%' as part of it's output, Leave extra
    /// formatting decisions to those using the block.
    fn perform(&self) -> Result<String, Box<dyn Error>>;
}
