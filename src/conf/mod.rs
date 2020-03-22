extern crate serde;
extern crate toml;

use self::serde::{Deserialize};
use std::path;
use std::fs;
use std::io::Read;

#[derive(Deserialize)]
pub struct Conf {
    db_dir: String,
    node_store: String,
    relationship_store: String,
    properties_store: String,
}

pub fn load_conf(main_dir: &str) -> Conf {
    let mut file_path = path::PathBuf::new();
    file_path.push(main_dir);
    file_path.push("orange-db.toml");
    let mut configuration_file = fs::OpenOptions::new()
        .read(true)
        .open(file_path.as_path())
        .expect("Cannot open the configuration file");
    
    let mut content = String::new();

    match configuration_file.read_to_string(&mut content) {
        Ok(bytes) => println!("{} bytes has been appended to buffer.", bytes),
        Err(error) => panic!(
            "
            The data in this stream is not valid UTF-8.\n
            See error: '{}'
            ",
            error
        ),
    }
    toml::from_str(content.as_str()).expect("Something went wrong")
}