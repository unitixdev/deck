use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::store::schema;


#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i64,
    pub public_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub public_id: Uuid,
}
