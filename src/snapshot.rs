use std::io;

use super::{lxc, Container};

/// An LXD ephemeral snapshot
pub struct Snapshot<'a> {
    _container: &'a mut Container,
    name: String
}

impl<'a> Snapshot<'a> {
    /// Create a snapshot of a container
    ///
    /// # Arguments
    ///
    /// * `name` - name of the new snapshot
    ///
    /// # Return
    ///
    /// An empty tuple on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while executing will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Container, Location, Snapshot};
    ///
    /// let mut container = Container::new(Location::Local, "test-snapshot", "ubuntu:16.04").unwrap();
    /// Snapshot::new(&mut container, "test-snapshot").unwrap();
    /// ```
    pub fn new(container: &'a mut Container, name: &str) -> io::Result<Snapshot<'a>> {
        lxc(&["snapshot", container.name(), name])?;

        let full_name = format!("{}/{}", container.name(), name);
        Ok(Snapshot {
            _container: container,
            name: full_name
        })
    }
}

impl<'a> Drop for Snapshot<'a> {
    fn drop(&mut self) {
        let _ = lxc(&["delete", &self.name]);
    }
}
