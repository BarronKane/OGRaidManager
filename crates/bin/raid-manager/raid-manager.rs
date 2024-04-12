mod commands;
pub mod raid_team;
mod modals;
mod raid_team_io;
mod raid_application;

use ogrm;

use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;
use serenity::prelude::*;
use serenity::all::{CreateInteractionResponse, GatewayIntents};
use serenity::all::Interaction;
use serenity::model::id;
use serenity::all::ChannelType;

use serenity::http::Http;
use serenity::client::Cache;
use serenity::http::CacheHttp;

use std::any::Any;
use std::{
    collections::HashMap,
    env::var,
    sync::{Arc, Mutex},
    time::Duration,
};

// Types used by all command functions.
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions.
pub struct Data {
    raid_teams: Vec<raid_team::RaidTeam>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // Custom error handler.
    // Only handle ones we want to customize.
    // Forward the rest to the default handler.
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: serenity::all::Context, interaction: Interaction) {
        if let Interaction::Component(interaction) = interaction {
            // println! ("Received command interaction: {interaction:#?}");

            let id = &interaction.data.custom_id;

            if id == "application_button" {
                println! ("Received raid team application request!");

                let mut failed: bool = false;

                let in_user = &interaction.member;

                if in_user.is_none() {
                    failed = true;
                }

                if failed == false {
                    let id = in_user.clone().unwrap().user.id;
                    let applications = raid_application::read_applications();

                    let mut found: bool = false;

                    match applications {
                        Ok(apps) => {
                            for mut app in apps {
                                if id == app.id {
                                    found = true;
                                    let reply = raid_application::construct_reply(&mut app, None);

                                    let dm_check = interaction.member.clone().unwrap().user.direct_message(ctx.http.as_ref(), 
                                        CreateMessage::new()
                                        .add_embed(reply)
                                    ).await;                                    
                                }
                            }

                            if found == false {
                                let mut app = raid_application::RaidApplication::default();
                                app.id = id;

                                // TODO: Safe
                                let mut in_apps = raid_application::read_applications().unwrap();
                                in_apps.push(app.clone());
                                raid_application::write_applications(in_apps).unwrap();

                                let reply = raid_application::construct_reply(&mut app, None);
                                let dm_check = interaction.member.clone().unwrap().user.direct_message(ctx.http.as_ref(), 
                                        CreateMessage::new()
                                        .add_embed(reply)
                                    ).await; 
                            }
                        },
                        Err(e) => {
                            println!("Unable to read applications file: {}", e);
                            println!("Attempting to create...");
                            let _try = raid_application::run_raid_application();
                            let _attempt = vec![_try];
                            let err = raid_application::write_applications(_attempt);

                            match err {
                                Ok(e) => {
                                    println!("Success! Try again.")
                                },
                                Err(e) => {
                                    println!("Critical failure: {}", e);
                                }
                            }
                        }
                    }

                }
            }
            let _ = &interaction.create_response(ctx.http.as_ref(), CreateInteractionResponse::Acknowledge).await;
        }
    }

    async fn message(&self, ctx: serenity::all::Context, msg: Message) {
        let ctype = msg.channel(ctx.http.clone()).await;

        if msg.is_private() {
            let mut applicationss = raid_application::read_applications();

            let mut changed = false;

            match applicationss {
                Ok(mut apps) => {
                    for mut app in 1..apps.len() {
                        if msg.author.id == apps[app].id {
                            if apps[app].stage != raid_application::ApplicationStage::closed {
                                let value = msg.content.clone();

                                apps[app].stage = apps[app].stage.bump();
                                let reply = raid_application::construct_reply(&mut apps[app], Some(value));

                                let dmcheck = msg.author.direct_message(ctx.http.as_ref(), 
                                        CreateMessage::new()
                                        .add_embed(reply))
                                    .await;

                                if apps[app].stage == raid_application::ApplicationStage::finished {
                                    apps[app].stage = apps[app].stage.bump();

                                    let fin = raid_application::construct_application(&apps[app]);

                                    let chan = var("DISCORD_CHANNEL")
                                        .expect("Missing `DISCORD_CHANNEL` env var, see README for more information.");

                                    let thread = var("DISCORD_THREAD")
                                        .expect("Missing `DISCORD_CHANNEL` env var, see README for more information.");

                                    let thread_num = thread.parse::<u64>().unwrap();

                                    let chan_num = chan.parse::<u64>().unwrap();

                                    let role_id = var("DISCORD_ROLE")
                                        .expect("Missing `DISCORD_ROLE` env var, see README for more information.");

                                    let mut content = String::new();
                                    content.push_str("New Applicant: <@");
                                    content.push_str(apps[app].id.clone().to_string().as_str());
                                    content.push('>');

                                    let fin_reply = CreateMessage::new()
                                        .content(content)
                                        .add_embed(fin);

                                    let msgcheck = ctx.http.send_message(serenity::all::ChannelId::from(thread_num), vec![], 
                                        &fin_reply
                                    ).await;
                                }
                            }
                        }
                    }
                    let result = raid_application::write_applications(apps);
                },
                Err(e) => {
                    println!("Unable to open applications file: {}", e);
                }
            }
        }
    }
}

fn main() {
    //scoped_main();

    let test = ogrm::establish_connection();
}

#[tokio::main]
async fn scoped_main() {
    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::show_raid_team_info()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("/".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    raid_teams: Vec::new(),
                })
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::all::ClientBuilder::new(token.clone(), intents)
        .framework(framework)
        .event_handler(Handler)
        .await;

    client.unwrap().start().await.unwrap()
}
