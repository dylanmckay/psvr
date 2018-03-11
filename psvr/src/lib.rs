extern crate hmdee_core;

extern crate ahrs;
extern crate byteorder;
extern crate delta;
#[macro_use]
extern crate error_chain;
pub extern crate hidapi;
extern crate nalgebra as na;

pub use self::client::*;
pub use self::errors::{Error, ErrorKind, ResultExt};

mod errors;
mod client;
pub mod command;
pub mod inertia;
pub mod protocol;
pub mod sensor;
mod usb;


