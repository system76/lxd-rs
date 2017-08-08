//! A Rust library for controlling LXD

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process::{Command, Stdio};
use std::io;

pub use container::Container;
pub use info::Info;
pub use location::Location;

mod container;
mod info;
mod location;

fn lxc(args: &[&str]) -> io::Result<()> {
    let mut cmd = Command::new("lxc");
    for arg in args.iter() {
        cmd.arg(arg);
    }

    let status = cmd.spawn()?.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("LXD {:?} failed with {}", args, status)
        ))
    }
}

fn lxc_output(args: &[&str]) -> io::Result<Vec<u8>> {
    let mut cmd = Command::new("lxc");
    for arg in args.iter() {
        cmd.arg(arg);
    }
    cmd.stdout(Stdio::piped());

    let output = cmd.spawn()?.wait_with_output()?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("LXD {:?} failed with {}", args, output.status)
        ))
    }
}
