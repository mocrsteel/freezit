// @generated automatically by Diesel CLI.

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        expiration_months -> Int4,
    }
}

diesel::table! {
    storage (storage_id) {
        storage_id -> Int4,
        product_id -> Int4,
        weight_grams -> Float4,
        date_in -> Date,
        date_out -> Nullable<Date>,
        available -> Bool,
    }
}

diesel::joinable!(storage -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(products, storage,);
