use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    schema::people::{self, dsl::*},
};

use super::{filters::PersonFilter, response::Page};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    