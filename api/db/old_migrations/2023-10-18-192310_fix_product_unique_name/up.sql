-- Your SQL goes here
-- delete any duplicates found, preferring to delete the longest expiration time (safer).
DELETE FROM products a
    USING products b
    WHERE a.product_id < b.product_id
    AND a.name = b.name;

ALTER TABLE products
    ADD CONSTRAINT products_name_unique UNIQUE(name);