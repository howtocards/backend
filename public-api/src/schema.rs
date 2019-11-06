table! {
    cards (id) {
        id -> Int4,
        author_id -> Int4,
        title -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        useful_for -> Int8,
        content -> Jsonb,
        content_for_search -> Varchar,
    }
}

table! {
    tokens (token) {
        token -> Varchar,
        user_id -> Int4,
    }
}

table! {
    useful_marks (card_id, user_id) {
        card_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        display_name -> Nullable<Varchar>,
        gravatar_email -> Nullable<Varchar>,
        username -> Varchar,
    }
}

joinable!(cards -> users (author_id));
joinable!(tokens -> users (user_id));
joinable!(useful_marks -> cards (card_id));
joinable!(useful_marks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cards,
    tokens,
    useful_marks,
    users,
);
