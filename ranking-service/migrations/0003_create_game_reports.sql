-- migrations/0003_create_game_reports.sql
CREATE TABLE game_reports (
    id UUID PRIMARY KEY,
    game_id UUID NOT NULL REFERENCES games(id),
    reporter_id UUID NOT NULL REFERENCES players(id),
    p1_score INTEGER NOT NULL,
    p2_score INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);