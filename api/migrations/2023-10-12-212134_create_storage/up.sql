-- Your SQL goes here
CREATE TABLE IF NOT EXISTS storage (
    storage_id SERIAL PRIMARY KEY ,
    product_id INT NOT NULL,
    weight_grams NUMERIC NOT NULL,
    date_in DATE NOT NULL DEFAULT now(),
    date_out DATE,
    available BOOLEAN DEFAULT TRUE
);