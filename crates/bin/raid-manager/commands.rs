use serenity::{all::CreateActionRow, prelude::*};

use poise::CreateReply;

use futures::{Stream, StreamExt};
use std::fmt::Write as _;

use crate::{raid_team_io, Context, Error};

async fn autocomplete_name<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["show_raid_team_info"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// Show this help menu.
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a work-in-progress bot for raid team management.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn show_raid_team_info(
    ctx: Context<'_>,
    #[description = "Display the raid teams and all their info."]
    #[autocomplete = "autocomplete_name"]
    command: Option<String>,
) -> Result<(), Error> {
    let in_teams = raid_team_io::read_raid_teams();

    match in_teams {
        Ok(teams) => {
            let (request_embed, is_needed) = crate::raid_team::create_team_requests_embed(teams.clone());

            let reply: CreateReply;

            if is_needed == true {
                reply = CreateReply::default()
                    .embed(crate::raid_team::create_team_info_embed(teams.clone()))
                    .embed(crate::raid_team::create_team_Schedule_embed(teams.clone()))
                    .embed(crate::raid_team::create_team_status_embed(teams.clone()))
                    .embed(request_embed.unwrap());
            } else {
                reply = CreateReply::default()
                    .embed(crate::raid_team::create_team_info_embed(teams.clone()))
                    .embed(crate::raid_team::create_team_Schedule_embed(teams.clone()))
                    .embed(crate::raid_team::create_team_status_embed(teams.clone()))
            }
            
            ctx.send(reply).await?;
        },
        Err(e) => {
            ctx.send(
                CreateReply::default()
                    .content("Something has gone horribly wrong. Message FIX!")
            ).await?;
        }
    }

    Ok(())
}

