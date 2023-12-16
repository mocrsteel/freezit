-- Your SQL goes here
CREATE TABLE IF NOT EXISTS freezers
(
    freezer_id SERIAL PRIMARY KEY,
    name       VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS drawers
(
    drawer_id  SERIAL PRIMARY KEY,
    name       VARCHAR(50) NOT NULL,
    freezer_id INT         NOT NULL REFERENCES freezers (freezer_id) ON DELETE CASCADE,
    UNIQUE (freezer_id, name)
);

CREATE TABLE IF NOT EXISTS products
(
    product_id        SERIAL PRIMARY KEY,
    name              VARCHAR(50) UNIQUE NOT NULL,
    expiration_months INT                NOT NULL DEFAULT (6)
);

CREATE TABLE IF NOT EXISTS storage
(
    storage_id   SERIAL PRIMARY KEY,
    product_id   INT     NOT NULL REFERENCES products (product_id) ON DELETE CASCADE,
    drawer_id    INT     NOT NULL REFERENCES drawers (drawer_id) ON DELETE CASCADE,
    weight_grams float4  NOT NULL,
    date_in      DATE    NOT NULL DEFAULT (now()),
    date_out     DATE,
    available    BOOLEAN NOT NULL DEFAULT (TRUE)
);