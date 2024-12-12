use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::schema::{card_sets, cards, collection_cards, collections};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, PartialEq, Debug)]
#[diesel(table_name = card_sets)]
pub struct Set {
    pub id: String,
    pub name: String,
    pub count: i64,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq, Insertable, Associations, Debug)]
#[diesel(belongs_to(Set, foreign_key = card_set))]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: String,
    pub card_set: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = collections)]
pub struct Collection {
    pub id: Uuid,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Card))]
#[diesel(belongs_to(Collection))]
#[diesel(table_name = collection_cards)]
#[diesel(primary_key(card_id, collection_id))]
pub struct CollectionCard {
    pub card_id: String,
    pub collection_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}