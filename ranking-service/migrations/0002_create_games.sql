CREATE TABLE games (
    id UUID PRIMARY KEY,
    p1_id UUID NOT NULL REFERENCES players(id),
    p2_id UUID NOT NULL REFERENCES players(id),
    state VARCHAR(20) NOT NULL DEFAULT 'InProgress',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);