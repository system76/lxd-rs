/// LXD host location
#[derive(Clone, Debug)]
pub enum Location {
    /// Local host
    Local,
    /// Remote host
    Remote(String),
}
