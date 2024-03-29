use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

use crate::raid_team::RaidTeam;
use crate::raid_team::CreateRaidTeam;

use crate::Error;

use fusion_util as util;

// TODO: Error propogation

pub fn raid_file() -> PathBuf {
    let path = util::get_cwd().unwrap().join("teams.json");
    println!("Raid teams file: {}", path.display());

    path
}

pub fn read_raid_teams() -> Result<Vec<CreateRaidTeam>, Error> {
    let mut raid_teams: Vec<CreateRaidTeam> = Vec::new();

    let mut file_content = File::open(raid_file())?;
    let mut contents = String::new();
    let file_contents = file_content.read_to_string(&mut contents)?;

    raid_teams = serde_json::from_str(&contents)?;

    Ok(raid_teams)
}

pub fn write_raid_teams(teams: Vec<CreateRaidTeam>) -> Result<(), Error> {
    let path = raid_file();
    let out = serde_json::to_string_pretty(&teams).unwrap();
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(out.as_bytes()).expect("Coulden't write file.");


    Ok(())
}
