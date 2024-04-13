use serde::{Deserialize, Serialize};
use serde_json;

use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use fusion_util as util;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String
}

fn config_file() -> PathBuf {
    let path = util::get_cwd().unwrap().join("config.json");
    println!("Config file: {}", path.display());

    path
}

pub fn read_config() -> Config {
    let mut file_content = File::open(config_file()).expect("Unable to open config file.");
    let mut contents = String::new();
    file_content.read_to_string(&mut contents).expect("Unable to read config file.");

    let config: Config = serde_json::from_str(&contents).expect("Unable to populate config struct.");

    config
}