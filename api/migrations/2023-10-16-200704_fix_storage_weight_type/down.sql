-- This file should undo anything in `up.sql`
ALTER TABLE storage
    ALTER COLUMN weight_grams TYPE numeric;
