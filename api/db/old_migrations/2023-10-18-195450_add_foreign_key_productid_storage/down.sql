-- This file should undo anything in `up.sql`
ALTER TABLE storage
    DROP CONSTRAINT storage_product_id_fkey;
