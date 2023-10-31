-- This file should undo anything in `up.sql`
DROP TABLE freezers CASCADE;
DROP TABLE drawers CASCADE;

ALTER TABLE storage
    DROP COLUMN drawer_id CASCADE;
