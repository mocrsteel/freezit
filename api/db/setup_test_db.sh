#!/bin/bash
# Get environment variables defined to set up database in docker
source .env

# Connection string.
#PSQL="psql -h ${POSTGRES_HOST} -p ${POSTGRES_PORT} -U ${POSTGRES_USER} -d ${POSTGRES_DB} -c"
PSQL="psql postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB} -c"

# cd ..
# DIESEL="$(diesel migration run)"
# if [[ -z "$(diesel)" ]]
# then
#   cargo install diesel_cli
# fi

# diesel setup

echo -e "\nPerforming reset of database ${POSTGRES_DB}"

# Clear the database before re-inserting data.
echo -e "\nTruncating table 'storage'"
$PSQL "TRUNCATE TABLE IF EXISTS storage RESTART IDENTITY CASCADE;"
echo -e "\nTruncating table 'drawers'"
$PSQL "TRUNCATE TABLE IF EXISTS drawers RESTART IDENTITY CASCADE;"
echo -e "\nTruncating table 'freezers'"
$PSQL "TRUNCATE TABLE IF EXISTS freezers RESTART IDENTITY CASCADE;"
echo -e "\nTruncating table 'products'"
$PSQL "TRUNCATE TABLE IF EXISTS products RESTART IDENTITY CASCADE;"



eeding test data to the database
# -- products
echo -e "Feeding products to database"
$PSQL "
INSERT INTO products
  (name, expiration_months)
VALUES
  ('Brocoli', 12),
  ('Groentensoep', 16),
  ('Spruiten', 12),
  ('Pastinaaksoep', 12),
  ('Puree', 18),
  ('Kippenballetjes', 6),
  ('Spaghettisaus', 12),
  ('Hamburgers', 6);
"
$PSQL "SELECT * FROM products"

# -- freezers
echo -e "Feeding freezers to database"
$PSQL "
INSERT INTO freezers
  (name)
VALUES
  ('Berging'),
  ('Garage'),
  ('Kelder');
"
$PSQL "SELECT * FROM freezers;"

# -- drawers
echo -e "Feeding drawers to database"
$PSQL "
INSERT INTO drawers
  (name, freezer_id)
VALUES
  ('Schuif 1', 1),
  ('Schuif 2', 1),
  ('Schuif 3', 1),
  ('Schuif 4', 1),
  ('Schuif 5', 1),
  ('Schuif 1', 2),
  ('Schuif 2', 2),
  ('Schuif 3', 2),
  ('Schuif 1', 3),
  ('Schuif 2', 3),
  ('Schuif 3', 3);
"
$PSQL "SELECT * FROM drawers LEFT JOIN freezers USING(freezer_id);"

# -- storage
echo -e "Feeding storage items to database"
$PSQL "
INSERT INTO storage
  (product_id, weight_grams, drawer_id, date_in, date_out)
VALUES
  (1, 400, 1, '2023-11-08', NULL),
  (1, 400, 1, '2023-11-08', NULL),
  (1, 400, 1, '2023-11-08', NULL),
  (1, 400, 1, '2023-11-08', NULL),
  (1, 400, 1, '2023-08-10', NULL),
  (1, 600, 1, '2023-08-10', NULL),
  (1, 600, 1, '2023-08-10', NULL),
  (1, 600, 1, '2023-08-10', NULL),
  (2, 300, 6, '2019-08-10', NULL),
  (2, 300, 6, '2023-08-10', NULL),
  (2, 300, 6, '2023-08-10', NULL),
  (4, 150, 8, '2023-05-21', NULL),
  (4, 150, 8, '2023-05-21', NULL),
  (4, 150, 8, '2018-05-21', NULL),
  (3, 300, 9, '2023-05-21', NULL),
  (3, 300, 9, '2023-05-21', NULL),
  (3, 300, 9, '2023-05-21', NULL),
  (8, 200, 10, '2018-06-02', NULL),
  (8, 200, 10, '2023-10-4', NULL),
  (8, 200, 10, '2023-10-4', NULL),
  (8, 200, 10, '2023-10-4', NULL),
  (8, 200, 10, '2023-10-4', NULL),
  (8, 200, 10, '2023-10-4', NULL),
  (6, 350, 4, '2023-07-13', NULL),
  (5, 600, 7, '2023-07-13', NULL),
  (3, 400, 6, '2023-07-13', NULL),
  (4, 500, 4, '2023-07-13', NULL),
  (4, 500, 4, '2023-07-13', NULL),
  (4, 500, 4, '2023-07-13', NULL),
  (4, 500, 4, '2023-07-13', NULL),
  (7, 643.3, 2, '2023-08-10', NULL),
  (7, 653.3, 2, '2023-08-12', NULL),
  (7, 633.3, 2, '2023-08-14', NULL),
  (7, 623.3, 2, '2023-08-31', NULL),
  (7, 663.3, 2, '2023-09-10', NULL),
  (7, 653.3, 2, '2023-09-10', '2024-7-1'),
  (7, 663.3, 2, '2023-09-10', '2023-12-23')
"

# shellcheck disable=SC2005
echo "$($PSQL "SELECT * FROM storage
  LEFT JOIN products USING(product_id)
  LEFT JOIN drawers USING(drawer_id)
  LEFT JOIN freezers USING(freezer_id);
")"

