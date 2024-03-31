use serenity::all::CreateEmbed;
use serenity::all::CreateMessage;
use serenity::all::CreateButton;
use serenity::all::CreateEmbedFooter;
use serenity::model::Colour;
use serenity::all::EmbedField;

use serenity::model::id;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct RaidTeam {
    team_role: Option<id::RoleId>,

    team_name: Option<String>,
    raid_lead: Option<String>,
    raid_colead: Option<String>,
    raid_days: Option<String>,
    raid_time: Option<String>,
    achievement: Option<String>,
    status: Option<String>,
    team_type: Option<String>,
    trial_requests: Option<String>
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct CreateRaidTeam(RaidTeam);

impl CreateRaidTeam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team_name(mut self, team_name: impl Into<String>) -> Self {
        self.0.team_name = Some(team_name.into());
        self
    }

    pub fn raid_lead(mut self, raid_lead: impl Into<String>) -> Self {
        self.0.raid_lead = Some(raid_lead.into());
        self
    }

    pub fn raid_colead(mut self, raid_colead: impl Into<String>) -> Self {
        self.0.raid_colead = Some(raid_colead.into());
        self
    }

    pub fn raid_days(mut self, raid_days: impl Into<String>) -> Self {
        self.0.raid_days = Some(raid_days.into());
        self
    }

    pub fn raid_time(mut self, raid_time: impl Into<String>) -> Self {
        self.0.raid_time = Some(raid_time.into());
        self
    }

    pub fn achievement(mut self, achievement: impl Into<String>) -> Self {
        self.0.achievement = Some(achievement.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.0.status = Some(status.into());
        self
    }

    pub fn team_type(mut self, team_type: impl Into<String>) -> Self {
        self.0.team_type = Some(team_type.into());
        self
    }

    pub fn trial_requests(mut self, trial_requests: impl Into<String>) -> Self {
        self.0.trial_requests = Some(trial_requests.into());
        self
    }
}

fn scheck(s: Option<String>) -> String {
    match s {
        Some(o) => {
            return o;
        },
        None => return "".to_string(),
    }
}

fn subject_scheck(subject: String, s: Option<String>) -> String {
    match s {
        Some(o) => {
            let mut in_subject = subject;
            in_subject.push_str(":");

            let mut out = String::new();
            out.push_str(boldscheck(Some(in_subject)).as_str());
            out.push(' ');
            out.push_str(&o);
            out.push_str("\n");
            return out;
        },
        None => return "".to_string(),
    }
}

fn nlscheck(s: Option<String>) -> String {
    match s {
        Some(o) => {
            let mut out = o;
            out.push_str("\n");
            return out;
        },
        None => return " \n".to_string(),
    }
}

fn boldscheck(s: Option<String>) -> String {
    match s {
        Some(o) => {
            let mut out: String = "**".to_string();
            out.push_str(&o);
            out.push_str("**");
            return out;
        },
        None => return "".to_string(),
    }
}

fn underlinescheck(s: Option<String>) -> String {
    match s {
        Some(o) => {
            let mut out: String = "__".to_string();
            out.push_str(&o);
            out.push_str("__");
            return out;
        },
        None => return "".to_string(),
    }
}

pub fn create_team_info_aggregate(teams: Vec<CreateRaidTeam>) -> CreateEmbed {
    
    let mut fields: Vec<(String, String, bool)> = Vec::new();
    
    for team in teams{

        let mut body = String::new();

        let mut lead_aggregate = scheck(team.0.raid_lead.clone());
        lead_aggregate.push_str(" **|** ");
        lead_aggregate.push_str(scheck(team.0.raid_colead.clone()).as_str());

        body.push_str(&subject_scheck("Lead | Colead".to_string(), Some(lead_aggregate)));
        //body.push_str(&subject_scheck("Raid Colead".to_string(), team.0.raid_colead));

        let mut day_aggregate = scheck(team.0.raid_days.clone());
        day_aggregate.push_str(" - ");
        day_aggregate.push_str(scheck(team.0.raid_time.clone()).as_str());

        body.push_str(&subject_scheck("Raiding Days".to_string(), Some(day_aggregate)));
        //body.push_str(&subject_scheck("Raid Time".to_string(), team.0.raid_time));

        let mut status_agregate = scheck(team.0.status.clone());
        status_agregate.push_str(" - ");
        status_agregate.push_str(scheck(team.0.achievement.clone()).as_str());

        //body.push_str(&subject_scheck("Achievement".to_string(), team.0.achievement));
        body.push_str(&subject_scheck("Status".to_string(), Some(status_agregate)));

        fields.push((
            underlinescheck(team.0.team_name),
            body, 
            false
        ));
    }

    CreateEmbed::new()
        .title("Old Gods Raid Group Info")
        .color(Colour::from_rgb(166, 0, 255))
        .fields(fields)
}

pub fn create_team_info_embed(teams: Vec<CreateRaidTeam>) -> CreateEmbed {

    let mut team_names: String = String::new();
    let mut team_leads: String = String::new();
    let mut team_coleads: String = String::new();

    for team in teams {
        team_names.push_str(nlscheck(team.0.team_name).as_str());
        team_leads.push_str(nlscheck(team.0.raid_lead).as_str());
        team_coleads.push_str(nlscheck(team.0.raid_colead).as_str());
    }    

    CreateEmbed::new()
        .title("Old Gods Raid Group Info")
        .color(Colour::from_rgb(166, 0, 255))
        .field("Team Name", team_names, true)
        .field("Raid Lead", team_leads, true)
        .field("Raid Colead", team_coleads, true)
}

pub fn create_team_Schedule_embed(teams: Vec<CreateRaidTeam>) -> CreateEmbed {

    let mut team_names: String = String::new();
    let mut team_days: String = String::new();
    let mut team_times: String = String::new();

    for team in teams {
        team_names.push_str(nlscheck(team.0.team_name).as_str());
        team_days.push_str(nlscheck(team.0.raid_days).as_str());
        team_times.push_str(nlscheck(team.0.raid_time).as_str());
    }

    let mut info_embed = CreateEmbed::new();
    info_embed.color(Colour::from_rgb(166, 0, 255));

    CreateEmbed::new()
        .title("Raid Team Schedule")
        .color(Colour::from_rgb(51, 255, 0))
        .field("Team Name", team_names, true)
        .field("Raiding Days", team_days, true)
        .field("Raid Time (Est.)", team_times, true)
}

pub fn create_team_status_embed(teams: Vec<CreateRaidTeam>) -> CreateEmbed {

    let mut team_names: String = String::new();
    let mut team_achievement: String = String::new();
    let mut team_status: String = String::new();

    for team in teams {
        team_names.push_str(nlscheck(team.0.team_name).as_str());
        team_achievement.push_str(nlscheck(team.0.achievement).as_str());
        team_status.push_str(nlscheck(team.0.status).as_str());
    }    

    CreateEmbed::new()
        .title("Raid Team Status")
        .color(Colour::from_rgb(211, 29, 29))
        .field("Team Name", team_names, true)
        .field("Achievement", team_achievement, true)
        .field("Status", team_status, true)
        .footer(CreateEmbedFooter::new("(Don't hurt me I'm a new bot. Message FIX if I misbehave.)"))
}

pub fn create_team_requests_embed(teams: Vec<CreateRaidTeam>) -> (Option<CreateEmbed>, bool) {

    let mut is_needed = false;

    let mut fields: Vec<(String, String, bool)> = Vec::new();

    for team in teams {
        if team.0.trial_requests.is_some() {
            is_needed = true;
            
            fields.push((nlscheck(team.0.team_name), nlscheck(team.0.trial_requests), false));
        }
    }   

    if is_needed == false {
        return (None, is_needed);
    }

    
    let embed = CreateEmbed::new()
        .color(Colour::from_rgb(2, 135, 123))
        .title("Raid Team Trial Requests")
        .fields(fields);

    (Some(embed), is_needed)
}


pub fn create_app_button() -> CreateButton {
    CreateButton::new("application_button")
        .label("Raid Application")
}
