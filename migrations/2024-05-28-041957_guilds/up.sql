CREATE TABLE guilds (
    id BIGSERIAL PRIMARY KEY
);

CREATE TABLE raidteams (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGSERIAL NOT NULL REFERENCES guilds(id),
    team_name TEXT UNIQUE NOT NULL
);

CREATE TABLE raidleads (
    id BIGSERIAL PRIMARY KEY,
    team TEXT UNIQUE NOT NUll REFERENCES raidteams(team_name)
);

