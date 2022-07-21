#[macro_use]
extern crate rocket;

extern crate lazy_static;

use itertools::Itertools;
use ornaguide_rs::{error::Error, items::admin::AdminItem};
use rocket::{routes, serde::json::Json, Config};
use serde::{Deserialize, Serialize};

mod data;
mod filter;

use crate::{data::DATA, filter::Filter};

/// All the filters applicable on an item.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ItemFilters<'a> {
    /// Filter by id.
    pub id: Filter<'a, u32>,
    /// Filter by codex_uri.
    pub codex_uri: Filter<'a, String>,
    /// Filter by attack.
    pub attack: Filter<'a, i16>,
}

impl<'a> ItemFilters<'a> {
    /// Compile all filters within `self`.
    pub fn compiled(self) -> Result<Self, Error> {
        Ok(Self {
            id: self.id.compiled()?,
            codex_uri: self.codex_uri.compiled()?,
            attack: self.attack.compiled()?,
        })
    }
}

#[post("/items", format = "json", data = "<filters>")]
fn post_items(filters: Json<ItemFilters>) -> Json<Vec<AdminItem>> {
    let lock = DATA.as_ref().unwrap();
    let lock = lock.read();
    let data = lock.as_ref().unwrap();
    let filters = filters.into_inner().compiled().unwrap();
    Json(
        data.guide
            .items
            .items
            .iter()
            .filter(|item| filters.id.filter(&item.id))
            .filter(|item| filters.codex_uri.filter(&item.codex_uri))
            .filter(|item| filters.attack.filter(&item.attack))
            .cloned()
            .collect_vec(),
    )
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 12346,
        ..Config::debug_default()
    };

    if let Err(e) = DATA.as_ref() {
        panic!("{}", e);
    }

    rocket::custom(&config).mount("/api/v1", routes![post_items])
}
