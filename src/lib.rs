#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;

mod torrent;

pub use torrent::{Torrent};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() -> () {
        Torrent::from_file("screen.torrent").unwrap();
        ()
    }

    #[bench]
    fn bench_test1(b: &mut Bencher) {
        b.iter(|| {
            Torrent::from_file("screen.torrent").unwrap()
        });
    }
}
