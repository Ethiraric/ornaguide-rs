use itertools::Itertools;
use ornaguide_rs::{data::OrnaData, error::Kind, skills::admin::AdminSkill};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_locale_data,
    deref,
    error::{Error, MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
    make_post_impl,
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

impl SkillFilters<'_> {
    /// Get the array of admin skills from the data structure.
    pub fn get_entities(data: &OrnaData) -> &Vec<AdminSkill> {
        &data.guide.skills.skills
    }

    /// Dereference IDs to the name of the entity they refer to.
    pub fn deref(skills: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
        if let serde_json::Value::Array(skills) = skills {
            for skill in skills.iter_mut() {
                if let serde_json::Value::Object(skill) = skill {
                    if let Some(type_) = skill.get_mut("type_") {
                        deref::skill_type(type_, data)?;
                    }
                    if let Some(element) = skill.get_mut("element") {
                        if !element.is_null() {
                            deref::element(element, data)?;
                        }
                    }
                    if let Some(buffed_by) = skill.get_mut("buffed_by") {
                        deref::monsters(buffed_by, data)?;
                    }
                    if let Some(causes) = skill.get_mut("causes") {
                        deref::status_effects(causes, data)?;
                    }
                    if let Some(cures) = skill.get_mut("cures") {
                        deref::status_effects(cures, data)?;
                    }
                    if let Some(gives) = skill.get_mut("gives") {
                        deref::status_effects(gives, data)?;
                    }
                } else {
                    return Err(Kind::Misc("Skill should be an object".to_string()).into_err())
                        .to_internal_server_error();
                }
            }
            Ok(())
        } else {
            Err(Kind::Misc("Skills should be an array".to_string()).into_err())
                .to_internal_server_error()
        }
    }
}

make_post_impl!(SkillFilters);

/// Query for skills.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/skills", format = "json", data = "<filters>")]
pub fn post(filters: Json<SkillFilters>) -> MaybeResponse {
    MaybeResponse {
        contents: post_impl(filters.into_inner()),
    }
}

/// This route is needded when making a CORS call to the API.
#[options("/skills")]
pub fn options() -> &'static str {
    ""
}
