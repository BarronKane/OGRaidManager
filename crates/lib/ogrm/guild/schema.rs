// @generated automatically by Diesel CLI.

diesel::table! {
    guilds (id) {
        id -> Int8,
    }
}

diesel::table! {
    raidleads (id) {
        id -> Int8,
        team -> Text,
    }
}

diesel::table! {
    raidteams (id) {
        id -> Int8,
        guild_id -> Int8,
        team_name -> Text,
    }
}

diesel::joinable!(raidteams -> guilds (guild_id));

diesel::allow_tables_to_appear_in_same_query!(
    guilds,
    raidleads,
    raidteams,
);
