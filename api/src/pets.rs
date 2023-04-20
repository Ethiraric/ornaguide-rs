use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::ErrorKind,
    pets::admin::{AdminPet, CostType},
};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_locale_data,
    deref::deref_skills,
    error::{Error, MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
    make_post_impl,
    options::Options,
};

/// All the filters applicable on a pet.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
#[api_filter(AdminPet)]
pub struct PetFilters<'a> {
    /// Filter by id.
    pub id: Filter<'a, u32>,
    /// Filter by codex_uri.
    pub codex_uri: Filter<'a, String>,
    /// Filter by name.
    pub name: Filter<'a, String>,
    /// Filter by tier.
    pub tier: Filter<'a, u8>,
    /// Filter by image_name.
    pub image_name: Filter<'a, String>,
    /// Filter by description.
    pub description: Filter<'a, String>,
    /// Filter by attack.
    pub attack: Filter<'a, u8>,
    /// Filter by heal.
    pub heal: Filter<'a, u8>,
    /// Filter by buff.
    pub buff: Filter<'a, u8>,
    /// Filter by debuff.
    pub debuff: Filter<'a, u8>,
    /// Filter by spell.
    pub spell: Filter<'a, u8>,
    /// Filter by protect.
    pub protect: Filter<'a, u8>,
    /// Filter by cost.
    pub cost: Filter<'a, u64>,
    /// Filter by cost_type.
    pub cost_type: Filter<'a, CostType>,
    /// Filter by limited.
    pub limited: Filter<'a, bool>,
    /// Filter by limited_details.
    pub limited_details: Filter<'a, String>,
    /// Filter by skills.
    pub skills: Filter<'a, Vec<u32>>,
    /// Generic options.
    #[serde(rename = "_options")]
    pub options: Options,
}

impl PetFilters<'_> {
    /// Get the array of admin pets from the data structure.
    fn get_entities(data: &OrnaData) -> &Vec<AdminPet> {
        &data.guide.pets.pets
    }

    /// Dereference IDs to the name of the entity they refer to.
    fn deref(pets: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
        if let serde_json::Value::Array(pets) = pets {
            for pet in pets.iter_mut() {
                if let serde_json::Value::Object(pet) = pet {
                    if let Some(skills) = pet.get_mut("skills") {
                        deref_skills(skills, data)?;
                    }
                } else {
                    return Err(ErrorKind::Misc("Skill should be an object".to_string()).into_err())
                        .to_internal_server_error();
                }
            }
            Ok(())
        } else {
            Err(ErrorKind::Misc("Skills should be an array".to_string()).into_err())
                .to_internal_server_error()
        }
    }
}

make_post_impl!(PetFilters);

/// Query for pets.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/pets", format = "json", data = "<filters>")]
pub fn post(filters: Json<PetFilters>) -> MaybeResponse {
    MaybeResponse {
        contents: post_impl(filters.into_inner()),
    }
}

/// This route is needded when making a CORS call to the API.
#[options("/pets")]
pub fn options() -> &'static str {
    ""
}
