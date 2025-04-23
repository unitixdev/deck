// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Text,
        set_id -> Text,
        name -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sets (id) {
        id -> Text,
        name -> Text,
        count -> Int4,
        total -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_cards (user_id, card_id) {
        user_id -> Int4,
        card_id -> Text,
        amount -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        public_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(cards -> sets (set_id));
diesel::joinable!(user_cards -> cards (card_id));
diesel::joinable!(user_cards -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    sets,
    user_cards,
    users,
);
