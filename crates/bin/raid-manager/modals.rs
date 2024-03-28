use crate::{Data, Error};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct TeamAppModal {
    first_input: String,
    second_input: Option<String>,
}


