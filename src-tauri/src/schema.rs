// @generated automatically by Diesel CLI.

diesel::table! {
    shows (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    titles (id) {
        id -> Integer,
        name -> Text,
        current_holder_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    wrestlers (id) {
        id -> Integer,
        name -> Text,
        gender -> Text,
        wins -> Integer,
        losses -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(titles -> wrestlers (current_holder_id));

diesel::allow_tables_to_appear_in_same_query!(shows, titles, users, wrestlers,);
