extern crate bincode;
extern crate data_structures;

use std::io::BufReader;
use std::fs::File;
use data_structures::{Db, SaveToDisk};

#[test]
fn test_file_with_content() {
    match data_structures::open_file("/tmp/something2".to_string()) {
        Ok(mut buf) => {
            match bincode::rustc_serialize::decode_from::<BufReader<File>,
                                                          Db>(&mut buf,
                                                              bincode::SizeLimit::Infinite) {
                Ok(_) => println!("Got decodable!"),
                Err(e) => println!("Error trying to write {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}

#[test]
fn test_write_to_file() {
    let mut db = match data_structures::open_or_create("/tmp/something".to_string()) {
        Ok(_db) => _db,
        Err(e) => {
            println!("Error reading db, returinng empty default. {}", e);
            data_structures::Db { map: std::collections::HashMap::new() }
        }
    };

    for i in 0..10 {
        db.map.insert(i.to_string(), "again".to_string());
    }
    db.map.insert("hello2".to_string(), "world".to_string());

    match db.save_to_disk("/tmp/something".to_string()) {
        Ok(true) => println!("Saved to disk"),
        Ok(false) => println!("Not saved"),
        Err(e) => println!("Error: {}", e),
    }

    println!("PRINTING ALL");
    for (book, review) in &db.map {
        println!("{}: \"{}\"", book, review);
    }
}
