CREATE TABLE guilds (
    id BIGSERIAL PRIMARY KEY
);

CREATE TABLE raidteams (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGSERIAL NOT NULL REFERENCES guilds(id),
    team_name TEXT NOT NULL
);

