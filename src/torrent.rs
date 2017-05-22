#![allow(dead_code)]
#![feature(proc_macro)] // Rust nightly
extern crate serde_bencode;
extern crate serde;

use std::fs;
use std::path::Path;
use self::serde_bencode::decoder;
use std::io::{self, Read};
use self::serde::bytes::ByteBuf;

#[derive(Debug, Deserialize)]
pub struct Node(String, u64);

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    path: String,
    length: u64,
    offset: u64,
    #[serde(default)]
    md5sum: String,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    #[serde(default)]
    name: String,
    #[serde(default)]
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: u64,
    #[serde(default)]
    length: u64,
    #[serde(default)]
    private: u8,
    // #[serde(default)]
    // #[serde(rename="root hash")]
    // root_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    info: Info,
    #[serde(default)]
    #[serde(rename="infoBuffer")]
    info_buffer: ByteBuf,
    #[serde(default)]
    #[serde(rename="infoHash")]
    info_hash: String,
    #[serde(default)]
    #[serde(rename="infoHashBuffer")]
    info_hash_buffer: ByteBuf,
    #[serde(default)]
    name: String,
    #[serde(default)]
    announce: String,
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Vec<Vec<String>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    creation_date: u64,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: String,
    #[serde(default)]
    #[serde(rename="urlList")]
    url_list: String,
    #[serde(default)]
    private: bool,
    #[serde(default)]
    length: u64,
    #[serde(default)]
    pieces: Vec<String>,
    #[serde(default)]
    #[serde(rename="lastPieceLength")]
    last_piece_length: u64,
    #[serde(default)]
    #[serde(rename="piece length")]
    piece_length: u64,
    #[serde(default)]
    files: Vec<File>,
}

#[derive(Debug)]
pub enum LoadFileError {
  Io(io::Error),
  DecodeError(serde_bencode::error::BencodeError),
  UpdateTorrentError(UpdateTorrentError),
}

#[derive(Debug)]
pub enum FromBufferError {
  DecodeError(serde_bencode::error::BencodeError),
  ReadError,
}

#[derive(Debug)]
pub enum UpdateTorrentError {

}

impl Torrent {
    pub fn from_file(path: &str) -> Result<Torrent, LoadFileError> {
        let path = Path::new(path);
        let mut f = match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(LoadFileError::Io(e)),
        };
        let mut buffer: Vec<u8> = Vec::new();
        match f.read_to_end(&mut buffer) {
            Ok(_) => {
                let mut torrent = decoder::from_bytes::<Torrent>(&buffer).unwrap();
                torrent.update_torrent();
                Ok(torrent)
            },
            Err(e) => Err(LoadFileError::Io(e)),
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Result<Torrent, FromBufferError> {
        match decoder::from_bytes::<Torrent>(&buffer) {
            Ok(t) => Ok(t),
            Err(e) => Err(FromBufferError::DecodeError(e)),
        }
    }

    pub fn update_torrent(&mut self) {
        if self.name == "" {
            self.name = self.info.name.clone();
        }
        if self.length == 0 {
            self.length = self.info.length;
        }
        if self.piece_length == 0 {
            self.piece_length = self.info.piece_length;
        }
        if self.last_piece_length == 0 {
            self.last_piece_length = self.length % self.piece_length;
        }
        if self.files.len() == 0 {
            let mut path: String = "./".to_string();
            path.push_str(&self.info.name);

            self.files = vec![File {
                name: self.info.name.clone(),
                path: path,
                length: self.length,
                offset: 0,
                md5sum: String::new(),
            }];
        }
    }
}

// info_buffer: b"", info_hash: "", info_hash_buffer: b""
// pieces...
