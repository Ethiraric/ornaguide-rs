use itertools::Itertools;
use ornaguide_rs::{data::OrnaData, error::Error as OError, monsters::admin::AdminMonster};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_locale_data,
    deref::{
        deref_elements, deref_items, deref_monster_family, deref_skills, deref_spawns,
        deref_status_effects,
    },
    error::{Error, MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
    make_post_impl,
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

impl MonsterFilters<'_> {
    /// Get the array of admin monsters from the data structure.
    fn get_entities(data: &OrnaData) -> &Vec<AdminMonster> {
        &data.guide.monsters.monsters
    }

    /// Dereference IDs to the name of the entity they refer to.
    fn deref(monsters: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
        if let serde_json::Value::Array(monsters) = monsters {
            for monster in monsters.iter_mut() {
                if let serde_json::Value::Object(monster) = monster {
                    if let Some(family) = monster.get_mut("family") {
                        if !family.is_null() {
                            deref_monster_family(family, data)?;
                        }
                    }
                    if let Some(spawns) = monster.get_mut("spawns") {
                        deref_spawns(spawns, data)?;
                    }
                    if let Some(weak_to) = monster.get_mut("weak_to") {
                        deref_elements(weak_to, data)?;
                    }
                    if let Some(resistant_to) = monster.get_mut("resistant_to") {
                        deref_elements(resistant_to, data)?;
                    }
                    if let Some(immune_to) = monster.get_mut("immune_to") {
                        deref_elements(immune_to, data)?;
                    }
                    if let Some(immune_to_status) = monster.get_mut("immune_to_status") {
                        deref_status_effects(immune_to_status, data)?;
                    }
                    if let Some(vulnerable_to_status) = monster.get_mut("vulnerable_to_status") {
                        deref_status_effects(vulnerable_to_status, data)?;
                    }
                    if let Some(drops) = monster.get_mut("drops") {
                        deref_items(drops, data)?;
                    }
                    if let Some(skills) = monster.get_mut("skills") {
                        deref_skills(skills, data)?;
                    }
                } else {
                    return Err(OError::Misc("Skill should be an object".to_string()))
                        .to_internal_server_error();
                }
            }
            Ok(())
        } else {
            Err(OError::Misc("Skills should be an array".to_string())).to_internal_server_error()
        }
    }
}

make_post_impl!(MonsterFilters);

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
