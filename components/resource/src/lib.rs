use async_trait::async_trait;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ResourceError(String);

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for ResourceError {}

/// A resource to query information from for use in the status bar
#[async_trait]
pub trait Resource: Sync + Send {
    /// The fetch function performs the the query for information from the resource.
    /// It should not return any extra formatting aside from the data itself.
    /// For example a volume resource should not include a '%' as part of it's output, Leave extra
    /// formatting decisions to those using the resouce.
    async fn fetch(&self) -> Result<String, Box<dyn Error>>;
}
