-- Your SQL goes here
ALTER TABLE products
    ADD COLUMN IF NOT EXISTS expiration_months int NOT NULL DEFAULT 6;