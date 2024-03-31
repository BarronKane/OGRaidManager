use crate::{Context, Error};
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::model::id;
use serenity::all::EventHandler;
use serenity::all::CreateEmbed;
use serenity::all::Colour;
use serde::Serialize;
use serde::Deserialize;

use std::default;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

use fusion_util as util;


#[derive(Clone, Copy, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum ApplicationStage {
    #[default]
    name,
    realm,
    class,
    spec,
    avail,
    pref,
    under,
    finished
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RaidApplication {
    pub id: id::UserId,

    pub character_name: String,
    pub realm_name: String,
    pub class: String,
    pub specialization: String,
    pub availability: String,
    pub preferred_application: String,
    pub understood: bool,

    pub stage: ApplicationStage
}

// IO

pub fn app_file() -> PathBuf {
    let path = util::get_cwd().unwrap().join("applications.json");
    println!("Applications file: {}", path.display());

    path
}

pub fn read_applications() -> Result<Vec<RaidApplication>, Error> {
    let mut applications: Vec<RaidApplication> = Vec::new();

    let mut file_content = File::open(app_file())?;
    let mut contents = String::new();
    let file_contents = file_content.read_to_string(&mut contents)?;

    applications = serde_json::from_str(&contents)?;

    Ok(applications)
}

pub fn write_applications(application: Vec<RaidApplication>) -> Result<(), Error> {
    let path = app_file();
    let out = serde_json::to_string_pretty(&application).unwrap();
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(out.as_bytes()).expect("Coulden't write file.");

    Ok(())
}

// IO

pub fn run_raid_application() -> RaidApplication {
    RaidApplication::default()
}


pub fn construct_reply(stage: ApplicationStage) -> CreateEmbed {

    match stage {
        ApplicationStage::name => {
            let mut desc = String::new();
            desc.push_str("Thank you for your interest in joining an OG Raid Team!\n\n");
            desc.push_str("To start, please tell me your __**character name:**__");

            return CreateEmbed::new()
                .title("__Old Gods Raid Application Form__")
                .color(Colour::from_rgb(166, 0, 255))
                .description(desc);
        },
        _ => {
            return CreateEmbed::new();
        }
    };
}
