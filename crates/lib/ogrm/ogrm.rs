use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;

use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use fusion_util as util;

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    username: String,
    password: String,
    name: String
}

pub fn config_file() -> PathBuf {
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

pub fn establish_connection() -> PgConnection {
    let config = read_config();

    let mut db_url = String::new();
    db_url.push_str("postgres://");
    db_url.push_str(&config.username);
    db_url.push(':');
    db_url.push_str(&config.password);
    db_url.push('@');
    db_url.push_str(&config.host);
    db_url.push(':');
    db_url.push_str(config.port.to_string().as_str());
    db_url.push('/');
    db_url.push_str(&config.name);

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
