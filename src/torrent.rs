use std::fs;
use std::path::Path;
use std::io::{self, Read};
use std::collections::BTreeMap;

use bencode::{Bencode, ToBencode};
use bencode::util::ByteString;

use serde_bencode;
use serde_bencode::de;

use serde_bytes;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(default)]
    name: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    length: u64,
    #[serde(default)]
    offset: u64,
    #[serde(default)]
    md5sum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(default)]
    length: u64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    #[serde(rename="piece length")]
    piece_length: u64,
    #[serde(default)]
    private: u8,
}

#[derive(Serialize)]
pub struct InfoBuf {
    length: u64,
    name: String,
    pieces: Vec<u8>,
    #[serde(rename="piece length")]
    piece_length: u64,
    private: u8,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    #[serde(rename="infoBuffer")]
    pub info_buffer: Vec<u8>,
    #[serde(default)]
    #[serde(rename="infoHash")]
    pub info_hash: String,
    #[serde(default)]
    #[serde(rename="infoHashBuffer")]
    pub info_hash_buffer: Vec<u8>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub announce: String,
    #[serde(default)]
    #[serde(rename="announce-list")]
    pub announce_list: Vec<Vec<String>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    pub creation_date: u64,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    #[serde(rename="created by")]
    pub created_by: String,
    #[serde(default)]
    #[serde(rename="urlList")]
    pub url_list: String,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub length: u64,
    #[serde(default)]
    pub pieces: Vec<Vec<u8>>,
    #[serde(default)]
    #[serde(rename="lastPieceLength")]
    pub last_piece_length: u64,
    #[serde(default)]
    #[serde(rename="piece length")]
    pub piece_length: u64,
    #[serde(default)]
    pub files: Vec<File>,
}

#[derive(Debug)]
pub enum LoadFileError {
    Io(io::Error),
    DecodeError(serde_bencode::error::Error),
}

#[derive(Debug)]
pub enum FromBufferError {
    DecodeError(serde_bencode::error::Error),
}

#[derive(Debug)]
pub enum FromStringError {
    DecodeError(serde_bencode::error::Error),
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
                let mut torrent = Torrent::from_buffer(&buffer).unwrap();
                torrent.update_torrent();
                Ok(torrent)
            },
            Err(e) => Err(LoadFileError::Io(e)),
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Result<Torrent, FromBufferError> {
        match de::from_bytes::<Torrent>(&buffer) {
            Ok(t) => Ok(t),
            Err(e) => Err(FromBufferError::DecodeError(e)),
        }
    }

    pub fn from_string(string: &str) -> Result<Torrent, FromStringError> {
        match de::from_str::<Torrent>(&string) {
            Ok(t) => Ok(t),
            Err(e) => Err(FromStringError::DecodeError(e)),
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

        if self.pieces.len() == 0 {
            for x in self.info.pieces.chunks(20) {
                self.pieces.push(x.to_vec());
            }
        }

        let bencode: Bencode = self.to_bencode();
        self.info_buffer = bencode.to_bytes().unwrap();
        self.info_hash = sha1sync(&self.info_buffer);
    }
}

impl ToBencode for Torrent {
    fn to_bencode(&self) -> Bencode {
        let mut m = BTreeMap::new();
        m.insert(ByteString::from_str("length"), self.info.length.to_bencode());
        m.insert(ByteString::from_str("name"), self.info.name.to_bencode());
        m.insert(ByteString::from_str("pieces"), Bencode::ByteString(self.info.pieces.clone().into()));
        m.insert(ByteString::from_str("piece length"), self.info.piece_length.to_bencode());
        m.insert(ByteString::from_str("private"), self.info.private.to_bencode());
        Bencode::Dict(m)
    }
}

fn sha1sync(v: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.input(v);
    hasher.result_str()
}
