diesel::table! {
    guilds {
        id -> BigInt,
        guild_id -> BigInt,
    }
}

diesel::table! {
    raidteams (id) {
        id -> BigInt,
        guild_id -> BigInt,
        team_name -> Text,
    }
}

diesel::joinable!(raidteams -> guilds (guild_id));

diesel::allow_tables_to_appear_in_same_query!(guilds, raidteams,);
