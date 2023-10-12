// @generated automatically by Diesel CLI.

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    storage (storage_id) {
        storage_id -> Int4,
        product_id -> Int4,
        weight_grams -> Numeric,
        date_in -> Date,
        date_out -> Nullable<Date>,
        available -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    storage,
);
