// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id, expansion) {
        id -> Text,
        expansion -> Text,
        name -> Text,
    }
}

diesel::table! {
    collection_cards (card_id, card_expansion, collections_id) {
        card_id -> Text,
        card_expansion -> Text,
        collections_id -> Int4,
        amount -> Int2,
        rating -> Int2,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    collections (id) {
        id -> Int4,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
    }
}

diesel::joinable!(collection_cards -> collections (collections_id));
diesel::joinable!(collections -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    collection_cards,
    collections,
    users,
);
