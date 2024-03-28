use serenity::{all::CreateActionRow, prelude::*};

use poise::CreateReply;

use futures::{Stream, StreamExt};
use std::fmt::Write as _;

use crate::{Context, Error};

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
    ctx.send( 
        CreateReply::default()
            .embed(crate::raid_team::create_team_embed())
            .components(vec![CreateActionRow::Buttons(vec![crate::raid_team::create_app_button()])])
    )
    .await?;

    Ok(())
}

