use itertools::Itertools;
use ornaguide_rs::{
    error::Error,
    pets::admin::{AdminPet, CostType},
};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_data,
    error::{MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
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

/// Implementation function just so I can return a `Result` and `?`.
pub fn post_impl(filters: PetFilters) -> Result<serde_json::Value, crate::error::Error> {
    let options = filters.options.clone();
    with_data(|data| {
        if filters.is_none() {
            Ok(data.guide.pets.pets.clone())
        } else {
            let filters = filters.compiled().to_bad_request()?.into_fn_vec();
            Ok(data
                .guide
                .pets
                .pets
                .iter()
                .filter(|pet| filters.iter().map(|f| f(pet)).all(|x| x))
                .cloned()
                .collect_vec())
        }
    })
    .and_then(|mut items| {
        PetFilters::apply_sort(&options, &mut items).to_bad_request()?;
        Ok(items)
    })
    .and_then(|items| {
        serde_json::to_value(items)
            .map_err(ornaguide_rs::error::Error::from)
            .to_internal_server_error()
    })
}

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
