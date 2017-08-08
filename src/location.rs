/// LXD host location
pub enum Location {
    /// Local host
    Local,
    /// Remote host
    Remote(String),
}
