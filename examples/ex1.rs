extern crate parse_torrent;

use parse_torrent::Torrent;

/// cargo run --example ex1
fn main() {
    let test = Torrent::from_file("screen.torrent");
    println!("test: {:?}", test);
}
