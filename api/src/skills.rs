use itertools::Itertools;
use lazy_static::__Deref;
use ornaguide_rs::{error::Error, skills::admin::AdminSkill};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::DATA,
    filter::{compilable::Compilable, Filter},
    options::Options,
};

/// All the filters applicable on a skill.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
#[api_filter(AdminSkill)]
pub struct SkillFilters<'a> {
    /// Filter by id.
    pub id: Filter<'a, u32>,
    /// Filter by codex_uri.
    pub codex_uri: Filter<'a, String>,
    /// Filter by name.
    pub name: Filter<'a, String>,
    /// Filter by tier.
    pub tier: Filter<'a, u8>,
    /// Filter by type_.
    pub type_: Filter<'a, u32>,
    /// Filter by is_magic.
    pub is_magic: Filter<'a, bool>,
    /// Filter by mana_cost.
    pub mana_cost: Filter<'a, u32>,
    /// Filter by description.
    pub description: Filter<'a, String>,
    /// Filter by element.
    pub element: Filter<'a, Option<u32>>,
    /// Filter by offhand.
    pub offhand: Filter<'a, bool>,
    /// Filter by cost.
    pub cost: Filter<'a, u64>,
    /// Filter by bought.
    pub bought: Filter<'a, bool>,
    /// Filter by skill_power.
    pub skill_power: Filter<'a, f32>,
    /// Filter by strikes.
    pub strikes: Filter<'a, u8>,
    /// Filter by modifier_min.
    pub modifier_min: Filter<'a, f32>,
    /// Filter by modifier_max.
    pub modifier_max: Filter<'a, f32>,
    /// Filter by extra.
    pub extra: Filter<'a, String>,
    /// Filter by buffed_by.
    pub buffed_by: Filter<'a, Vec<u32>>,
    /// Filter by causes.
    pub causes: Filter<'a, Vec<u32>>,
    /// Filter by cures.
    pub cures: Filter<'a, Vec<u32>>,
    /// Filter by gives.
    pub gives: Filter<'a, Vec<u32>>,
    /// Generic options.
    #[serde(rename = "_options")]
    pub options: Options,
}

/// Query for skills.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/skills", format = "json", data = "<filters>")]
pub fn post(filters: Json<SkillFilters>) -> Json<Vec<AdminSkill>> {
    let lock = DATA.as_ref().unwrap();
    let lock = lock.read();
    let data = lock.as_ref().unwrap().deref();

    if filters.is_none() {
        Json(data.guide.skills.skills.clone())
    } else {
        let filters = filters.into_inner().compiled().unwrap().into_fn_vec();
        Json(
            data.guide
                .skills
                .skills
                .iter()
                .filter(|skill| filters.iter().map(|f| f(skill)).all(|x| x))
                .cloned()
                .collect_vec(),
        )
    }
}

/// This route is needded when making a CORS call to the API.
#[options("/skills")]
pub fn options() -> &'static str {
    ""
}
