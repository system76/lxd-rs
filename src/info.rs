use serde_json;
use std::collections::BTreeMap;
use std::io;

use super::{lxc_output, Location};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Snapshot {
    pub architecture: String,
    pub config: BTreeMap<String, String>,
    pub created_at: String,
    //pub devices: TODO,
    pub ephemeral: bool,
    pub expanded_config: BTreeMap<String, String>,
    pub expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    pub last_used_at: String,
    pub name: String,
    pub profiles: Vec<String>,
    pub stateful: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub status: String,
    pub status_code: usize,
    //pub disk: TODO,
    pub memory: BTreeMap<String, usize>,
    //pub network: TODO,
    pub pid: usize,
    pub processes: usize,
    pub cpu: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
/// LXD container information
pub struct Info {
    pub architecture: String,
    pub config: BTreeMap<String, String>,
    pub devices: BTreeMap<String, BTreeMap<String, String>>,
    pub ephemeral: bool,
    pub profiles: Vec<String>,
    pub created_at: String,
    pub expanded_config: BTreeMap<String, String>,
    pub expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    pub name: String,
    pub stateful: bool,
    pub status: String,
    pub status_code: usize,
    pub last_used_at: String,
    pub state: Option<State>,
    pub snapshots: Option<Vec<Snapshot>>,
}

impl Info {
    /// Retrieve LXD container information from all containers
    ///
    /// # Arguments
    ///
    /// * `location` - The location of the host
    ///
    /// # Return
    ///
    /// The LXD container information
    ///
    /// # Errors
    ///
    /// Errors that are encountered while retrieving info will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Info, Location};
    ///
    /// let info = Info::all(Location::Local).unwrap();
    /// ```
    pub fn all(location: Location) -> io::Result<Vec<Self>> {
        let json = match location {
            Location::Local => lxc_output(&["list", "--format", "json"])?,
            Location::Remote(remote) => lxc_output(&["list", &format!("{}:", remote), "--format", "json"])?
        };

        serde_json::from_slice::<Vec<Self>>(&json).map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("LXD info: failed to parse json: {}", err)
            )
        })
    }

    /// Retrieve LXD container information from one container
    ///
    /// # Arguments
    ///
    /// * `location` - The location of the host
    /// * `name` - The name of the container
    ///
    /// # Return
    ///
    /// The LXD container information
    ///
    /// # Errors
    ///
    /// Errors that are encountered while retrieving info will be returned
    ///
    /// # Example
    ///
    /// ```
    /// use lxd::{Container, Info, Location};
    ///
    /// let mut container = Container::new(Location::Local, "test-info", "ubuntu:16.04").unwrap();
    /// let info = Info::new(Location::Local, "test-info").unwrap();
    /// ```
    pub fn new(location: Location, name: &str) -> io::Result<Self> {
        let json = match location {
            Location::Local => lxc_output(&["list", &format!("{}$", name), "--format", "json"])?,
            Location::Remote(remote) => lxc_output(&["list", &format!("{}:", remote), &format!("{}$", name), "--format", "json"])?
        };

        match serde_json::from_slice::<Vec<Self>>(&json) {
            Ok(mut list) => if list.len() == 1 {
                Ok(list.remove(0))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("LXD info: {} not found", name)
                ))
            },
            Err(err) => {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("LXD info: failed to parse json: {}", err)
                ))
            }
        }
    }
}
