use serde::{Deserialize, Serialize};
use serde_json;

use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use fusion_util as util;

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    pub token: String,
    pub applicant_channel: String,
    pub raid_lead_role: String,
}

fn config_file() -> PathBuf {
    let path = util::get_cwd().unwrap().join("secrets.json");
    println!("Secrets file: {}", path.display());

    path
}

pub fn read_config() -> Secrets {
    let mut file_content = File::open(config_file()).expect("Unable to open secrets file.");
    let mut contents = String::new();
    file_content.read_to_string(&mut contents).expect("Unable to read secrets file.");

    let secrets: Secrets = serde_json::from_str(&contents).expect("Unable to populate secrets struct.");

    secrets
}
