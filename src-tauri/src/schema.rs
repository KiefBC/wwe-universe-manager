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
    signature_moves (id) {
        id -> Nullable<Integer>,
        wrestler_id -> Integer,
        move_name -> Text,
        move_type -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    title_holders (id) {
        id -> Integer,
        title_id -> Integer,
        wrestler_id -> Integer,
        held_since -> Timestamp,
        held_until -> Nullable<Timestamp>,
        event_name -> Nullable<Text>,
        event_location -> Nullable<Text>,
        change_method -> Nullable<Text>,
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
        title_type -> Text,
        division -> Text,
        prestige_tier -> Integer,
        gender -> Text,
        show_id -> Nullable<Integer>,
        is_active -> Bool,
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
        real_name -> Nullable<Text>,
        nickname -> Nullable<Text>,
        height -> Nullable<Text>,
        weight -> Nullable<Text>,
        debut_year -> Nullable<Integer>,
        promotion -> Nullable<Text>,
        strength -> Nullable<Integer>,
        speed -> Nullable<Integer>,
        agility -> Nullable<Integer>,
        stamina -> Nullable<Integer>,
        charisma -> Nullable<Integer>,
        technique -> Nullable<Integer>,
        biography -> Nullable<Text>,
        is_user_created -> Nullable<Bool>,
    }
}

diesel::joinable!(signature_moves -> wrestlers (wrestler_id));
diesel::joinable!(title_holders -> titles (title_id));
diesel::joinable!(title_holders -> wrestlers (wrestler_id));
diesel::joinable!(titles -> wrestlers (current_holder_id));

diesel::allow_tables_to_appear_in_same_query!(
    shows,
    signature_moves,
    title_holders,
    titles,
    users,
    wrestlers,
);
