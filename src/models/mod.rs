use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32, // Use Option if id is nullable
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>,
    // add other fields as needed
}
