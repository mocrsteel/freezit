-- This file should undo anything in `up.sql`
ALTER TABLE products
    DROP CONSTRAINT products_name_unique;