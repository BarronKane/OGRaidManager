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

use std::env::var;

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
    finished,
    closed
}

impl ApplicationStage {
    pub fn bump(&self) -> Self {
        use ApplicationStage::*;
        match *self {
            name => realm,
            realm => class,
            class => spec,
            spec => avail,
            avail => pref,
            pref => under,
            under => finished,
            finished => closed,
            closed => closed
        }
    }
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
    pub understood: String,

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

pub fn create_embed(s: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(" ")
        .color(Colour::from_rgb(166, 0, 255))
        .description(s)
}


pub fn construct_reply(app: &mut RaidApplication, s: Option<String>) -> CreateEmbed {

    match app.stage {
        ApplicationStage::name => {
            let mut desc = String::new();
            desc.push_str("Thank you for your interest in joining an OG Raid Team!\n\n");
            desc.push_str("To start, please tell me your **character name:**");

            return CreateEmbed::new()
                .title("Old Gods Raid Application Form")
                .color(Colour::from_rgb(166, 0, 255))
                .description(desc);
        },

        ApplicationStage::realm => {
            if s.is_some() {
                app.character_name = s.unwrap();
            }

            let desc = "Character's **realm:**";
            return create_embed(desc);
        }

        ApplicationStage::class => {
            if s.is_some() {
                app.realm_name = s.unwrap();
            }

            let desc = "Character's **class:**";
            return create_embed(desc);
        }

        ApplicationStage::spec => {
            if s.is_some() {
                app.class = s.unwrap();
            }

            let desc = "Character's **specialization:**";
            return create_embed(desc);
        }

        ApplicationStage::avail => {
            if s.is_some() {
                app.specialization = s.unwrap();
            }

            let desc = "What is your raiding **availability in EST?**";
            return create_embed(desc);
        }

        ApplicationStage::pref => {
            if s.is_some() {
                app.availability = s.unwrap();
            }

            let desc = "Do you have a **raid team preference?**";
            return create_embed(desc);
        }

        ApplicationStage::under => {
            if s.is_some() {
                app.preferred_application = s.unwrap();
            }

            let desc = "**Do you understand this is an application of interest, and it is Raid leaders' option to reach out to you?\n\n Please type 'I understand'.**";
            return create_embed(desc);
        }

        ApplicationStage::finished => {
            if s.is_some() {
                app.understood = s.unwrap();
            }

            let desc = "**Thank you for your intereste!\n\nThis is still a new bot and will be improved upon over time.**";
            return create_embed(desc);
        }

        ApplicationStage::closed => {
            return CreateEmbed::new();
        }
    };
}

fn sstring(s: &str, o: String) -> String {
    let mut out = String::new();
    out.push_str("**");
    out.push_str(s);
    out.push_str(": **");
    out.push_str(o.as_str());
    out.push_str("\n");

    return out;
}

pub fn construct_application(app: &RaidApplication) -> CreateEmbed {
    let mut applicant = String::new();
    applicant.push_str("Applicant: <@");
    applicant.push_str(app.id.to_string().as_str());
    applicant.push('>');

    let role_id = var("DISCORD_ROLE")
        .expect("Missing `DISCORD_ROLE` env var, see README for more information.");

    let mut role = String::new();
    role.push_str("<@&");
    role.push_str(role_id.as_str());
    role.push('>');

    let mut wowlog = String::new();
    wowlog.push_str("[Generated Wowlogs](https://www.warcraftlogs.com/character/us/");
    wowlog.push_str(&app.realm_name.clone());
    wowlog.push_str("/");
    wowlog.push_str(&app.character_name.clone());
    wowlog.push_str(")");
    wowlog.retain(|c| !c.is_whitespace());
    wowlog.push_str("\n");

    let mut raiderio = String::new();
    raiderio.push_str("[Generated Raider.IO](https://raider.io/characters/us/");
    raiderio.push_str(&app.realm_name.clone());
    raiderio.push_str("/");
    raiderio.push_str(&app.character_name.clone());
    raiderio.push_str(")");
    raiderio.retain(|c| !c.is_whitespace());
    raiderio.push_str("\n");

    let mut desc = String::new();
    desc.push_str(sstring("Character Name", app.character_name.clone()).as_str());
    desc.push_str(sstring("Realm", app.realm_name.clone()).as_str());
    desc.push_str(wowlog.as_str());
    desc.push_str(raiderio.as_str());
    desc.push_str(sstring("Class", app.class.clone()).as_str());
    desc.push_str(sstring("Spec", app.specialization.clone()).as_str());
    desc.push_str(sstring("Availability: ", app.availability.clone()).as_str());
    desc.push_str(sstring("Preferred Raid Team", app.preferred_application.clone()).as_str());
    desc.push_str(sstring("Acknowledgement", app.understood.clone()).as_str());

    CreateEmbed::new()
        .color(Colour::from_rgb(166, 0, 255))
        .title("New Raid Applicant!")
        .field("Applicant info:", desc, false)
        .footer(serenity::all::CreateEmbedFooter::new("I'm a new bot, be gentle. I will improve over time. Message FIX if I misbehave."))
}
