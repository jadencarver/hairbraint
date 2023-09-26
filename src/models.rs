use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ashes)]
pub struct Ash {
    pub id: i32,
    pub ash: String
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::aschanges)]
pub struct AsChange {
    pub id: i32,
    pub ash_id: i32,
    pub ante_id: i32,
    pub time: NaiveDateTime,
    pub sigma: i32,
    pub product_id: i32,
    pub alias: Option<String>,
    pub rate: Option<f32>
}

// ash_id -> Integer,
// ante_id -> Nullable<Integer>,
// time -> Timestamp,
// sigma -> Integer,
// product_id -> Integer,
// alias -> Nullable<Text>,
// rate -> Nullable<Float>,
