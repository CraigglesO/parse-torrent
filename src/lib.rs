#![allow(dead_code)]
mod torrent;

pub use torrent::{Torrent};

macro_rules! try_case (
  ($t:ident, $ex:expr, $err:ident) => (match $ex {
    &$t(ref x) => x,
    _          => return Err($err),
  })
);
