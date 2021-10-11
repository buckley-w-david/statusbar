use std::error::Error;

/// A resource to query information from for use in the status bar
pub trait Resource {
    /// The fetch function performs the the query for information from the resource.
    /// It should not return any extra formatting aside from the data itself.
    /// For example a volume resource should not include a '%' as part of it's output, Leave extra
    /// formatting decisions to those using the resouce.
    fn fetch(&self) -> Result<String, Box<dyn Error>>;
}
