extern crate bincode;
extern crate rustc_serialize;

use std::io::{BufReader, BufWriter, Result, Error, ErrorKind};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use bincode::rustc_serialize::{EncodingError, encode_into, decode_from};
use bincode::SizeLimit;


pub fn perfect_hash(s: &str) -> u32 {
    s.as_bytes().iter().fold(0, |acc: u32, &b| acc + b as u32) % 2 ^ 16
}

pub fn open_file(s: String) -> Result<BufReader<File>> {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(s) {
        Ok(f) => Ok(BufReader::new(f)),
        Err(e) => Err(e),
    }
}

pub fn open_or_create(s: String) -> Result<Db> {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .open(s) {
        Ok(f) => {
            Ok(decode_from::<BufReader<File>, Db>(&mut BufReader::new(f), SizeLimit::Infinite)
                .unwrap_or(Db { map: HashMap::new() }))
        }
        Err(e) => Err(e),
    }
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
pub struct Db {
    pub map: HashMap<String, String>,
}

pub trait SaveToDisk {
    fn save_to_disk(&self, file_path: String) -> Result<bool>;
}

impl SaveToDisk for Db {
    fn save_to_disk(&self, file_path: String) -> Result<bool> {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path) {
            Ok(f) => {
                match encode_into(self, &mut BufWriter::new(f), SizeLimit::Infinite) {
                    Err(e) => {
                        match e {
                            EncodingError::IoError(e) => Err(e),
                            _ => Err(Error::new(ErrorKind::UnexpectedEof, "Size limit reached")),
                        }
                    }
                    _ => Ok(true),
                }
            }
            Err(e) => Err(e),
        }
    }
}
