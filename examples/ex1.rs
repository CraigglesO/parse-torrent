extern crate parse_torrent;

use parse_torrent::Torrent;

/// cargo run --example ex1
fn main() {
    let test = Torrent::from_file("dev-screen.torrent");
    println!("test: {:?}", test);
}
