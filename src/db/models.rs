use diesel::prelude::*;
use crate::schema::runs;

#[derive(Queryable, Selectable)]
#[diesel(table_name = runs)]
pub struct Run {
    pub id: i32,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = runs)]
pub struct NewRun {
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
} 