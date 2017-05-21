#![allow(dead_code)]
extern crate chrono;
extern crate bencode;
extern crate rustc_serialize;

use std::fs;
use std::path::Path;
use std::io;
use std::io::Read;

use self::chrono::prelude::*;
use self::bencode::util::ByteString;
use self::bencode::{Bencode, ListVec, FromBencode, Decoder, NumFromBencodeError, StringFromBencodeError};
use self::bencode::Bencode::{Number, List, Dict};
use self::rustc_serialize::Decodable;

// use hash::{Sha1Hash, InvalidHashLength};

/// STRUCTS

#[derive(Debug)]
pub struct File {
    path:   String,
    name:   String,
    length: u64,
    offset: u64,
}

#[derive(Debug)]
pub struct Info {
    length:       u64,
    name:         Vec<u8>,
    piece_length: u32,
    pieces:       Vec<u8>,
    private:      bool,
}

#[derive(Debug)]
pub struct Torrent {
    // info:              Info,
    // info_buffer:       Vec<u8>,
    // info_hash:         String,
    // info_hash_buffer:  Vec<u8>,
    // name:              String,
    // private:           bool,
    // creation_date:     DateTime<UTC>,
    // created_by:        String,
    announce:          Vec<Vec<String>>,
    // url_list:          Vec<String>,
    // files:             Vec<File>,
    // length:            u64,
    piece_length:      u32,
    // last_piece_length: u32,
    // pieces:            Vec<String>,
}

/// ERRORS

#[derive(Debug)]
pub enum TorrentBencodeError {
    DecodeError,
    NotDecodable,
    LengthNotANumber(NumFromBencodeError),
    DoesntContainLength,
    AnnounceNotAString(StringFromBencodeError),
    DoesntContainAnnounce,
    PieceLengthNotANumber(NumFromBencodeError),
    DoesntContainPieceLength,
    // BoolFromBencodeError,
    // CharFromBencodeError,
    // FloatFromBencodeError,
    // MapFromBencodeError,
    // NumFromBencodeError,
    // StringFromBencodeError,
    // VecFromBencodeError,
}

#[derive(Debug)]
pub enum LoadFileError {
  Io(io::Error),
  FromBencode(FromBufferError),
}

#[derive(Debug)]
pub enum FromBufferError {
  InvalidBencode(bencode::streaming::Error),
  DecodeError(bencode::DecoderError),
  FromBencode(TorrentBencodeError),
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
            Ok(_) => (),
            Err(e) => return Err(LoadFileError::Io(e)),
        };
        Torrent::from_buffer(&buffer).map_err(LoadFileError::FromBencode)
    }

    pub fn from_buffer(buffer: &[u8]) -> Result<Torrent, FromBufferError> {
        let bencode = match bencode::from_buffer(buffer) {
            Ok(b)   => b,
            Err(e)  => return Err(FromBufferError::InvalidBencode(e)),
        };
        // println!("{:?}", bencode);
        FromBencode::from_bencode(&bencode).map_err(FromBufferError::FromBencode)
    }
}

impl FromBencode for Torrent {
    type Err = TorrentBencodeError;

    fn from_bencode(bencode: &Bencode) -> Result<Torrent, TorrentBencodeError> {
        use self::TorrentBencodeError::*;

        // Anounce
        let mut announce: Vec<Vec<String>> = Vec::new();
        let mut piece_length: u32 = 0;
        match bencode {
            &Bencode::Dict(ref d) => {
                match d.get(&ByteString::from_str("announce")) {
                    Some(s) => FromBencode::from_bencode(s).map(|s| {
                        announce.push(vec![s]);
                    }).map_err(AnnounceNotAString),
                    _ => Err(DoesntContainAnnounce),
                }
            },
            &Bencode::List(ref l) => {
                l.iter()
            },
            None => Err(NotDecodable),
        };
        Ok(Torrent{
            announce:     announce,
            piece_length: piece_length,
        })
    }
}
