-- Drop signature moves table
DROP TABLE signature_moves;

-- Remove added columns from wrestlers table
ALTER TABLE wrestlers DROP COLUMN trivia;
ALTER TABLE wrestlers DROP COLUMN biography;
ALTER TABLE wrestlers DROP COLUMN technique;
ALTER TABLE wrestlers DROP COLUMN charisma;
ALTER TABLE wrestlers DROP COLUMN stamina;
ALTER TABLE wrestlers DROP COLUMN agility;
ALTER TABLE wrestlers DROP COLUMN speed;
ALTER TABLE wrestlers DROP COLUMN strength;
ALTER TABLE wrestlers DROP COLUMN promotion;
ALTER TABLE wrestlers DROP COLUMN debut_year;
ALTER TABLE wrestlers DROP COLUMN weight;
ALTER TABLE wrestlers DROP COLUMN height;
ALTER TABLE wrestlers DROP COLUMN nickname;
ALTER TABLE wrestlers DROP COLUMN real_name;