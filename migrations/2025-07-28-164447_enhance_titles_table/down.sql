-- Revert titles table enhancements
ALTER TABLE titles DROP COLUMN title_type;
ALTER TABLE titles DROP COLUMN division;
ALTER TABLE titles DROP COLUMN prestige_tier;
ALTER TABLE titles DROP COLUMN gender;
ALTER TABLE titles DROP COLUMN show_id;
ALTER TABLE titles DROP COLUMN is_active;