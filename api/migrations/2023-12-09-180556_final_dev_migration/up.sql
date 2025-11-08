-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users
(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    email VARCHAR(200) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS freezers
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(50) UNIQUE NOT NULL,
    user_id    uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS drawers
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(50) NOT NULL,
    freezer_id INT         NOT NULL REFERENCES freezers (id) ON DELETE CASCADE,
    user_id     uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    UNIQUE (freezer_id, name)
);

CREATE TABLE IF NOT EXISTS products
(
    id                SERIAL PRIMARY KEY,
    name              VARCHAR(50) UNIQUE NOT NULL,
    expiration_months INT                NOT NULL DEFAULT (6),
    user_id           uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS storage
(
    id           SERIAL PRIMARY KEY,
    product_id   INT     NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    drawer_id    INT     NOT NULL REFERENCES drawers (id) ON DELETE CASCADE,
    weight_grams float4  NOT NULL,
    date_in      DATE    NOT NULL DEFAULT (now()),
    date_out     DATE,
    user_id      uuid    NOT NULL REFERENCES users (id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS email_whitelist
(
    id      SERIAL PRIMARY KEY,
    email   VARCHAR(250) UNIQUE NOT NULL
);