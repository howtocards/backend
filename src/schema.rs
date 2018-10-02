#![allow(proc_macro_derive_resolution_fallback)]

table! {
    cards (id) {
        id -> Int4,
        author_id -> Int4,
        title -> Varchar,
        content -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    tokens (token) {
        token -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(cards -> users (author_id));
joinable!(tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(cards, tokens, users,);
