use std::io;

use super::{lxc, Container};

/// An LXD ephemeral snapshot
pub struct Snapshot<'a> {
    _container: &'a Container,
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
    /// A new snapshot on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while creating snapshot will be returned
    /// ```
    pub fn new(container: &'a Container, name: &str) -> io::Result<Snapshot<'a>> {
        lxc(&["snapshot", container.name(), name])?;

        let full_name = format!("{}/{}", container.name(), name);
        Ok(Snapshot {
            _container: container,
            name: full_name
        })
    }

    /// Publish snapshot as an image
    ///
    /// # Arguments
    ///
    /// * `alias` - alias of the new image
    ///
    /// # Return
    ///
    /// An empty tuple on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while publishing will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Container, Location, Snapshot};
    ///
    /// let container = Container::new(Location::Local, "test-snapshot-publish", "ubuntu:16.04").unwrap();
    /// let snapshot = Snapshot::new(&container, "test-snapshot-publish").unwrap();
    /// snapshot.publish("test-publish").unwrap();
    /// ```
    pub fn publish(&self, alias: &str) -> io::Result<()> {
        lxc(&["publish", &self.name, "--alias", alias])
    }
}

impl<'a> Drop for Snapshot<'a> {
    fn drop(&mut self) {
        let _ = lxc(&["delete", &self.name]);
    }
}
