use itertools::Itertools;
use ornaguide_rs::{error::Error, monsters::admin::AdminMonster};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_data,
    error::{MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
    options::Options,
};

/// All the filters applicable on a monster.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
#[api_filter(AdminMonster)]
pub struct MonsterFilters<'a> {
    /// Filter by id.
    pub id: Filter<'a, u32>,
    /// Filter by codex_uri.
    pub codex_uri: Filter<'a, String>,
    /// Filter by name.
    pub name: Filter<'a, String>,
    /// Filter by tier.
    pub tier: Filter<'a, u8>,
    /// Filter by family.
    pub family: Filter<'a, Option<u32>>,
    /// Filter by image_name.
    pub image_name: Filter<'a, String>,
    /// Filter by boss.
    pub boss: Filter<'a, bool>,
    /// Filter by hp.
    pub hp: Filter<'a, u32>,
    /// Filter by level.
    pub level: Filter<'a, u32>,
    /// Filter by notes.
    pub notes: Filter<'a, String>,
    /// Filter by spawns.
    pub spawns: Filter<'a, Vec<u32>>,
    /// Filter by weak_to.
    pub weak_to: Filter<'a, Vec<u32>>,
    /// Filter by resistant_to.
    pub resistant_to: Filter<'a, Vec<u32>>,
    /// Filter by immune_to.
    pub immune_to: Filter<'a, Vec<u32>>,
    /// Filter by immune_to_status.
    pub immune_to_status: Filter<'a, Vec<u32>>,
    /// Filter by vulnerable_to_status.
    pub vulnerable_to_status: Filter<'a, Vec<u32>>,
    /// Filter by drops.
    pub drops: Filter<'a, Vec<u32>>,
    /// Filter by skills.
    pub skills: Filter<'a, Vec<u32>>,
    /// Generic options.
    #[serde(rename = "_options")]
    pub options: Options,
}

/// Implementation function just so I can return a `Result` and `?`.
pub fn post_impl(filters: MonsterFilters) -> Result<serde_json::Value, crate::error::Error> {
    let options = filters.options.clone();
    with_data(|data| {
        if filters.is_none() {
            Ok(data.guide.monsters.monsters.clone())
        } else {
            let filters = filters.compiled().to_bad_request()?.into_fn_vec();
            Ok(data
                .guide
                .monsters
                .monsters
                .iter()
                .filter(|monster| filters.iter().map(|f| f(monster)).all(|x| x))
                .cloned()
                .collect_vec())
        }
    })
    .and_then(|mut items| {
        MonsterFilters::apply_sort(&options, &mut items).to_bad_request()?;
        Ok(items)
    })
    .and_then(|items| {
        serde_json::to_value(items)
            .map_err(ornaguide_rs::error::Error::from)
            .to_internal_server_error()
    })
}

/// Query for monsters.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/monsters", format = "json", data = "<filters>")]
pub fn post(filters: Json<MonsterFilters>) -> MaybeResponse {
    MaybeResponse {
        contents: post_impl(filters.into_inner()),
    }
}

/// This route is needded when making a CORS call to the API.
#[options("/monsters")]
pub fn options() -> &'static str {
    ""
}
