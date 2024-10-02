use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::NaiveDateTime;
use crate::schema::ashes::dsl::{ashes, ash};
use crate::schema::aschanges::dsl::{aschanges, ash_id, ante_id, product_id};

#[derive(Debug, Queryable, Selectable, Clone, PartialEq)]
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

impl Ash {
    pub fn anteschanges(self, db: &mut SqliteConnection) -> Vec<AsChange> {
        aschanges.filter(ante_id.eq(self.id)).load::<AsChange>(db).unwrap()
    }
    pub fn aschanges(self, db: &mut SqliteConnection) -> Vec<AsChange> {
        aschanges.filter(ash_id.eq(self.id)).load::<AsChange>(db).unwrap()
    }
}

impl AsChange {

    pub fn ash(self, db: &mut SqliteConnection) -> Ash {
        ashes.find(self.ash_id).first::<Ash>(db).unwrap()
    }
    pub fn ante(self, db: &mut SqliteConnection) -> Ash {
        ashes.find(self.ante_id).first::<Ash>(db).unwrap()
    }
    pub fn product(self, db: &mut SqliteConnection) -> Ash {
        ashes.find(self.product_id).first::<Ash>(db).unwrap()
    }

}

