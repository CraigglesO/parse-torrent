#![allow(dead_code)]
#![feature(proc_macro)] // Rust nightly
#[macro_use]
extern crate serde_derive;

mod torrent;

pub use torrent::{Torrent};
