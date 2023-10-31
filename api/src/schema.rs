// @generated automatically by Diesel CLI.

diesel::table! {
    drawers (drawer_id) {
        drawer_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        freezer_id -> Int4,
    }
}

diesel::table! {
    freezers (freezer_id) {
        freezer_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

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
        drawer_id -> Int4,
    }
}

diesel::joinable!(drawers -> freezers (freezer_id));
diesel::joinable!(storage -> drawers (drawer_id));
diesel::joinable!(storage -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    drawers,
    freezers,
    products,
    storage,
);
