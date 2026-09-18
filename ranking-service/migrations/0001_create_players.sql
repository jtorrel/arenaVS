-- migrations/0001_create_players.sql
CREATE TABLE players (
    id UUID PRIMARY KEY,
    nickname VARCHAR(50) NOT NULL UNIQUE,
    elo INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);