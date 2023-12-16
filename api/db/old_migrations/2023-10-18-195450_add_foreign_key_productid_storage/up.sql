-- Your SQL goes here
ALTER TABLE storage
    ADD FOREIGN KEY(product_id) REFERENCES products(product_id);