// @generated automatically by Diesel CLI.

diesel::table! {
    match_participants (id) {
        id -> Integer,
        match_id -> Integer,
        wrestler_id -> Integer,
        team_number -> Nullable<Integer>,
        entrance_order -> Nullable<Integer>,
    }
}

diesel::table! {
    matches (id) {
        id -> Integer,
        show_id -> Integer,
        match_name -> Nullable<Text>,
        match_type -> Text,
        match_stipulation -> Nullable<Text>,
        scheduled_date -> Nullable<Date>,
        match_order -> Nullable<Integer>,
        winner_id -> Nullable<Integer>,
        is_title_match -> Bool,
        title_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    show_rosters (id) {
        id -> Integer,
        show_id -> Integer,
        wrestler_id -> Integer,
        assigned_at -> Nullable<Timestamp>,
        is_active -> Bool,
    }
}

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
        is_user_created -> Nullable<Bool>,
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

diesel::joinable!(match_participants -> matches (match_id));
diesel::joinable!(match_participants -> wrestlers (wrestler_id));
diesel::joinable!(matches -> shows (show_id));
diesel::joinable!(matches -> titles (title_id));
diesel::joinable!(matches -> wrestlers (winner_id));
diesel::joinable!(show_rosters -> shows (show_id));
diesel::joinable!(show_rosters -> wrestlers (wrestler_id));
diesel::joinable!(signature_moves -> wrestlers (wrestler_id));
diesel::joinable!(title_holders -> titles (title_id));
diesel::joinable!(title_holders -> wrestlers (wrestler_id));
diesel::joinable!(titles -> wrestlers (current_holder_id));

diesel::allow_tables_to_appear_in_same_query!(
    match_participants,
    matches,
    show_rosters,
    shows,
    signature_moves,
    title_holders,
    titles,
    users,
    wrestlers,
);
