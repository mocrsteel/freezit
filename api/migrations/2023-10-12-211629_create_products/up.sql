-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products
(
    product_id
        SERIAL
        PRIMARY KEY,
    name
        VARCHAR(50) NOT NULL
);