// @generated automatically by Diesel CLI.

diesel::table! {
    card_sets (id) {
        id -> Varchar,
        name -> Varchar,
        count -> Int8,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cards (id) {
        id -> Varchar,
        card_set -> Varchar,
        url -> Varchar,
        image -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Varchar,
    }
}

diesel::table! {
    collection_cards (card_id, collection_id) {
        card_id -> Varchar,
        collection_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    collections (id) {
        id -> Uuid,
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cards -> card_sets (card_set));
diesel::joinable!(collection_cards -> cards (card_id));
diesel::joinable!(collection_cards -> collections (collection_id));

diesel::allow_tables_to_appear_in_same_query!(
    card_sets,
    cards,
    collection_cards,
    collections,
);
