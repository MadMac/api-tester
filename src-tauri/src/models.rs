use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::config;
use crate::schema::requesttabs;

#[derive(Debug, Clone, Selectable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub uuid: String,
    pub config_data: String,
}

#[derive(Debug, Clone, Selectable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = requesttabs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RequestTabs {
    pub uuid: String,
    pub tabdata: String,
    pub tabdata_saved: Option<String>,
    pub saved_timestamp: Option<NaiveDateTime>
}