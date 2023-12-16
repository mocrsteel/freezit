-- This file should undo anything in `up.sql`
ALTER TABLE storage
    ALTER COLUMN available DROP NOT NULL;