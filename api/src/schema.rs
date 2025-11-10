// @generated automatically by Diesel CLI.

diesel::table! {
    drawers (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        freezer_id -> Int4,
        user_id -> Uuid,
    }
}

diesel::table! {
    email_whitelist (id) {
        id -> Int4,
        #[max_length = 250]
        email -> Varchar,
    }
}

diesel::table! {
    freezers (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        expiration_months -> Int4,
        user_id -> Uuid,
    }
}

diesel::table! {
    storage (id) {
        id -> Int4,
        product_id -> Int4,
        drawer_id -> Int4,
        weight_grams -> Float4,
        date_in -> Date,
        date_out -> Nullable<Date>,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        permissions -> Int4,
    }
}

diesel::joinable!(drawers -> freezers (freezer_id));
diesel::joinable!(drawers -> users (user_id));
diesel::joinable!(freezers -> users (user_id));
diesel::joinable!(products -> users (user_id));
diesel::joinable!(storage -> drawers (drawer_id));
diesel::joinable!(storage -> products (product_id));
diesel::joinable!(storage -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    drawers,
    email_whitelist,
    freezers,
    products,
    storage,
    users,
);
