-- Your SQL goes here
CREATE TABLE freezers
(
    freezer_id SERIAL PRIMARY KEY,
    name       VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE drawers
(
    drawer_id  SERIAL PRIMARY KEY,
    name       VARCHAR(50) NOT NULL,
    freezer_id INT         NOT NULL REFERENCES freezers (freezer_id),
    -- Each freezer should never have duplicate drawer names.
    -- But multiple freezers might have the same drawer names.
    UNIQUE (freezer_id, name)
);

INSERT INTO freezers (name)
VALUES ('test_freezer');

INSERT INTO drawers (name, freezer_id)
VALUES ('Lade 1', 1);

ALTER TABLE storage
    ADD COLUMN drawer_id INT NOT NULL REFERENCES drawers (drawer_id) DEFAULT 1;
