mod query_helper;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;

use std::io::{Read, Write};
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

fn config_file() -> PathBuf {
    let path = util::get_cwd().unwrap().join("config.json");
    println!("Config file: {}", path.display());

    path
}

fn read_config() -> Config {
    let mut file_content = File::open(config_file()).expect("Unable to open config file.");
    let mut contents = String::new();
    file_content.read_to_string(&mut contents).expect("Unable to read config file.");

    let config: Config = serde_json::from_str(&contents).expect("Unable to populate config struct.");

    config
}

fn write_config() {
    let path = config_file();
    let config = Config {
        host: "localhost".to_string(),
        port: 5555,
        username: "postgres".to_string(),
        password: "develop".to_string(),
        name: "ogrm".to_string()
    };

    let contents = serde_json::to_string_pretty(&config).unwrap();

    let mut file = File::create(path).expect("Unable to create file.");
    file.write_all(contents.as_bytes()).expect("Couldn't write file.");
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
    db_url.push_str(&config.port.to_string().as_str());
    db_url.push('/');

    let mut default_url = db_url.clone();
    default_url.push_str("postgres");
    
    db_url.push_str(&config.name);



    let db_test = PgConnection::establish(&db_url);

    match db_test {
        Ok(db) => {
            return db;
        },
        Err(e) => {
            let mut conn = PgConnection::establish(&default_url).expect("Could not connect to default db.");
            query_helper::create_database(&config.name).execute(&mut conn).expect("Could not create database!");

            let db_test_2 = PgConnection::establish(&db_url);

            return db_test_2.expect("Something went wrong connecting to the newly created database!");
        }
    }
}
