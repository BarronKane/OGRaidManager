use serenity::all::CreateEmbed;
use serenity::all::CreateMessage;
use serenity::all::CreateButton;
use serenity::all::CreateEmbedFooter;
use serenity::model::Colour;

use serenity::model::id;

pub struct RaidTeam {
    team_role: id::RoleId,

    team_name: String,
    raid_lead: String,
    raid_colead: String,
    raid_days: String,
    raid_time: String,
    Achievement: String,
    Status: String
}

pub fn create_team_embed() -> CreateEmbed {
    CreateEmbed::new()
        .color(Colour::from_rgb(185, 0, 179))
        .title("Old Gods Raid Groups Info")
        .field("Team Name", 
            "`Basic Training`

            `New Gods on the Block`
            `Bad Mafia`
            `Spazzynauts`
            `Family Business`
            `Kiss`
            `Oldish Gods`
            `OG After Dark`", 
            true)
        .field("Raid Lead", 
            "`Fix`

            `Dumai/Thugas`
            `Rafiki`
            `Spazzy`
            `Felrelic`
            `Doomerkin`
            `Triphos`
            `Stichy`", 
            true)
        .field("Raid Colead",
            "`Cub`

            `Skatha/Maynea`
            `CvTours`
            `Torm`
            `Beanyah`
            `Kruzelak`
            `Myschyf`",
            true)
        .footer(CreateEmbedFooter::new("(Don't hurt me I'm a new bot. Message FIX if I misbehave.)"))
}


pub fn create_app_button() -> CreateButton {
    CreateButton::new("app_button")
        .label("Raid Application")
}

pub fn create_team_message() -> CreateMessage {
    CreateMessage::new()
        .add_embed(create_team_embed())
        .button(create_app_button())
}
