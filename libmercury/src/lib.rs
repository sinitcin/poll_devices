#![feature(int_to_from_bytes)]
// #[macro_use]
extern crate byteorder;
extern crate crc;
extern crate serde;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate libengine;

pub mod device;
pub mod iface;
