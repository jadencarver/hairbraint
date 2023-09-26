// @generated automatically by Diesel CLI.

diesel::table! {
    aschanges (id) {
        id -> Integer,
        ash_id -> Integer,
        ante_id -> Integer,
        time -> Timestamp,
        sigma -> Integer,
        product_id -> Integer,
        alias -> Nullable<Text>,
        rate -> Nullable<Float>,
    }
}

diesel::table! {
    ashes (id) {
        id -> Integer,
        ash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    aschanges,
    ashes,
);
