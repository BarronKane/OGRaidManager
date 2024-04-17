use diesel::prelude::*;

use serenity::model::id;

use crate::guild::schema::guilds;
use crate::guild::schema::raidteams;

#[derive(Queryable, /*Selectable,*/ Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = guilds)]
pub struct Guilds {
    pub id: i64,

    pub guild_id: id::GuildId
}

#[derive(Queryable, /*Selectable,*/ Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = raidteams)]
pub struct RaidTeams {
    pub id: i64,

    pub guild_id: id::GuildId,

    pub team_name: String,
    /*
    team_role: Option<id::RoleId>,

    raid_lead_name: Option<String>,
    raid_lead_id: Option<id::UserId>,
    
    //raid_coleads: 

    raid_colead: Option<String>,
    raid_colead_id: Option<String>,
    raid_days: Option<String>,
    raid_time: Option<String>,
    achievement: Option<String>,
    status: Option<String>,
    team_type: Option<String>,
    trial_requests: Option<String>
    */
}
