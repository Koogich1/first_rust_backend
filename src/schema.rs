// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(messages, posts, users,);
