use std::io;
use std::path::Path;

use super::{lxc, Location};

/// An LXD container
pub struct Container(String);

impl Container {
    /// Create a new LXD container
    ///
    /// # Arguments
    ///
    /// * `location` - The location of the host
    /// * `name` - The name of the container
    /// * `base` - The base distribution to use, `ubuntu:16.04` for example
    ///
    /// # Return
    ///
    /// The newly created LXD container
    ///
    /// # Errors
    ///
    /// Errors that are encountered while creating will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Container, Location};
    ///
    /// let mut container = Container::new(Location::Local, "test-new", "ubuntu:16.04").unwrap();
    /// ```
    pub fn new(location: Location, name: &str, base: &str) -> io::Result<Self> {
        let full_name = match location {
            Location::Local => format!("{}", name),
            Location::Remote(remote) => format!("{}:{}", remote, name)
        };

        lxc(&["launch", base, &full_name, "-e", "-n", "lxdbr0"])?;

        // Hack to wait for network up and running
        lxc(&["exec", &full_name, "--mode=non-interactive", "-n", "--", "dhclient"])?;

        Ok(Container(full_name))
    }

    /// Run a command in an LXD container
    ///
    /// # Arguments
    ///
    /// * `command` - An array of command arguments
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
    /// use lxd::{Container, Location};
    ///
    /// let mut container = Container::new(Location::Local, "test-exec", "ubuntu:16.04").unwrap();
    /// container.exec(&["echo", "hello"]).unwrap();
    /// ```
    pub fn exec(&mut self, command: &[&str]) -> io::Result<()> {
        let mut args = vec!["exec", &self.0, "--"];
        for arg in command.as_ref().iter() {
            args.push(arg.as_ref());
        }
        lxc(&args)
    }

    /// Mount a path in an LXD container
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the mount
    /// * `source` - The source path to mount
    /// * `dest` - The destination of the mount
    ///
    /// # Return
    ///
    /// An empty tuple on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while mounting will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Container, Location};
    ///
    /// let mut container = Container::new(Location::Local, "test-mount", "ubuntu:16.04").unwrap();
    /// container.mount("source", ".", "/root/source").unwrap();
    /// ```
    pub fn mount<P: AsRef<Path>>(&mut self, name: &str, source: P, dest: &str) -> io::Result<()> {
        lxc(&["config", "device", "add", &self.0, name, "disk", &format!("source={}", source.as_ref().display()), &format!("path={}", dest)])
    }

    /// Push a file to the LXD container
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the file in the host
    /// * `dest` - The destination of the file in the container
    /// * `recursive` - The source is a directory
    ///
    /// # Return
    ///
    /// An empty tuple on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while mounting will be returned
    ///
    /// # Example
    ///
    /// ```
    /// extern crate lxd;
    /// extern crate tempdir;
    ///
    /// use lxd::{Container, Location};
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let mut container = Container::new(Location::Local, "test-push", "ubuntu:16.04").unwrap();
    ///     let tmp = TempDir::new("").unwrap();
    ///     container.push(tmp.path(), "/root", true).unwrap();
    /// }
    /// ```
    pub fn push<P: AsRef<Path>>(&mut self, source: P, dest: &str, recursive: bool) -> io::Result<()> {
        if recursive {
            lxc(&["file", "push", "--uid=0", "--gid=0", "-r", &format!("{}", source.as_ref().display()), &format!("{}/{}", self.0, dest)])
        } else {
            lxc(&["file", "push", "--uid=0", "--gid=0", &format!("{}", source.as_ref().display()), &format!("{}/{}", self.0, dest)])
        }
    }

    /// Pull a file from the LXD container
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the file in the container
    /// * `dest` - The destination of the file in the host
    /// * `recursive` - The source is a directory
    ///
    /// # Return
    ///
    /// An empty tuple on success
    ///
    /// # Errors
    ///
    /// Errors that are encountered while mounting will be returned
    ///
    /// # Example
    ///
    /// ```
    /// extern crate lxd;
    /// extern crate tempdir;
    ///
    /// use lxd::{Container, Location};
    /// use tempdir::TempDir;
    ///
    /// fn main() {
    ///     let mut container = Container::new(Location::Local, "test-pull", "ubuntu:16.04").unwrap();
    ///     container.exec(&["mkdir", "artifacts"]).unwrap();
    ///     let tmp = TempDir::new("").unwrap();
    ///     container.pull("/root/artifacts", tmp.path(), true).unwrap();
    /// }
    /// ```
    pub fn pull<P: AsRef<Path>>(&mut self, source: &str, dest: P, recursive: bool) -> io::Result<()> {
        if recursive {
            lxc(&["file", "pull", "-r", &format!("{}/{}", self.0, source), &format!("{}", dest.as_ref().display())])
        } else {
            lxc(&["file", "pull", &format!("{}/{}", self.0, source), &format!("{}", dest.as_ref().display())])
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        let _ = lxc(&["stop", &self.0]);
    }
}
