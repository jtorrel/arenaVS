-- migrations/0004_alter_games_add_reports.sql
ALTER TABLE games 
ADD COLUMN p1_report_id UUID REFERENCES game_reports(id),
ADD COLUMN p2_report_id UUID REFERENCES game_reports(id);