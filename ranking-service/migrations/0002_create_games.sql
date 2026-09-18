-- migrations/0002_create_games.sql
CREATE TABLE games (
    id UUID PRIMARY KEY,
    p1_id UUID NOT NULL REFERENCES players(id),
    p2_id UUID NOT NULL REFERENCES players(id),
    p1_score SMALLINT,
    p2_score SMALLINT,
    state VARCHAR(20) NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);