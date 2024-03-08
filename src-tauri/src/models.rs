use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::config;

#[derive(Debug, Clone, Selectable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub uuid: String,
    pub config_data: String,
}