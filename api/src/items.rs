use itertools::Itertools;
use ornaguide_rs::{data::OrnaData, error::Error, items::admin::AdminItem};
use proc_macros::api_filter;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    data::with_data,
    error::{MaybeResponse, ToErrorable},
    filter::{compilable::Compilable, Filter},
    make_post_impl,
    options::Options,
};

/// All the filters applicable on an item.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
#[api_filter(AdminItem)]
pub struct ItemFilters<'a> {
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
    /// Filter by image_name.
    pub image_name: Filter<'a, String>,
    /// Filter by description.
    pub description: Filter<'a, String>,
    /// Filter by notes.
    pub notes: Filter<'a, String>,
    /// Filter by hp.
    pub hp: Filter<'a, i16>,
    /// Filter by hp_affected_by_quality.
    pub hp_affected_by_quality: Filter<'a, bool>,
    /// Filter by mana.
    pub mana: Filter<'a, i16>,
    /// Filter by mana_affected_by_quality.
    pub mana_affected_by_quality: Filter<'a, bool>,
    /// Filter by attack.
    pub attack: Filter<'a, i16>,
    /// Filter by attack_affected_by_quality.
    pub attack_affected_by_quality: Filter<'a, bool>,
    /// Filter by magic.
    pub magic: Filter<'a, i16>,
    /// Filter by magic_affected_by_quality.
    pub magic_affected_by_quality: Filter<'a, bool>,
    /// Filter by defense.
    pub defense: Filter<'a, i16>,
    /// Filter by defense_affected_by_quality.
    pub defense_affected_by_quality: Filter<'a, bool>,
    /// Filter by resistance.
    pub resistance: Filter<'a, i16>,
    /// Filter by resistance_affected_by_quality.
    pub resistance_affected_by_quality: Filter<'a, bool>,
    /// Filter by dexterity.
    pub dexterity: Filter<'a, i16>,
    /// Filter by dexterity_affected_by_quality.
    pub dexterity_affected_by_quality: Filter<'a, bool>,
    /// Filter by ward.
    pub ward: Filter<'a, i8>,
    /// Filter by ward_affected_by_quality.
    pub ward_affected_by_quality: Filter<'a, bool>,
    /// Filter by crit.
    pub crit: Filter<'a, u8>,
    /// Filter by crit_affected_by_quality.
    pub crit_affected_by_quality: Filter<'a, bool>,
    /// Filter by foresight.
    pub foresight: Filter<'a, i8>,
    /// Filter by view_distance.
    pub view_distance: Filter<'a, u32>,
    /// Filter by follower_stats.
    pub follower_stats: Filter<'a, u32>,
    /// Filter by follower_act.
    pub follower_act: Filter<'a, i32>,
    /// Filter by status_infliction.
    pub status_infliction: Filter<'a, u32>,
    /// Filter by status_protection.
    pub status_protection: Filter<'a, u32>,
    /// Filter by mana_saver.
    pub mana_saver: Filter<'a, i8>,
    /// Filter by has_slots.
    pub has_slots: Filter<'a, bool>,
    /// Filter by base_adornment_slots.
    pub base_adornment_slots: Filter<'a, u8>,
    /// Filter by rarity.
    pub rarity: Filter<'a, String>,
    /// Filter by element.
    pub element: Filter<'a, Option<u32>>,
    /// Filter by equipped_by.
    pub equipped_by: Filter<'a, Vec<u32>>,
    /// Filter by two_handed.
    pub two_handed: Filter<'a, bool>,
    /// Filter by orn_bonus.
    pub orn_bonus: Filter<'a, f32>,
    /// Filter by gold_bonus.
    pub gold_bonus: Filter<'a, f32>,
    /// Filter by drop_bonus.
    pub drop_bonus: Filter<'a, f32>,
    /// Filter by spawn_bonus.
    pub spawn_bonus: Filter<'a, f32>,
    /// Filter by exp_bonus.
    pub exp_bonus: Filter<'a, f32>,
    // Filter by boss.
    pub boss: Filter<'a, bool>,
    /// Filter by arena.
    pub arena: Filter<'a, bool>,
    /// Filter by category.
    pub category: Filter<'a, Option<u32>>,
    /// Filter by causes.
    pub causes: Filter<'a, Vec<u32>>,
    /// Filter by cures.
    pub cures: Filter<'a, Vec<u32>>,
    /// Filter by gives.
    pub gives: Filter<'a, Vec<u32>>,
    /// Filter by prevents.
    pub prevents: Filter<'a, Vec<u32>>,
    /// Filter by materials.
    pub materials: Filter<'a, Vec<u32>>,
    /// Filter by price.
    pub price: Filter<'a, u32>,
    /// Filter by ability.
    pub ability: Filter<'a, Option<u32>>,
    /// Generic options.
    #[serde(rename = "_options")]
    pub options: Options,
}
impl ItemFilters<'_> {
    /// Get the array of admin items from the data structure.
    fn get_entities(data: &OrnaData) -> &Vec<AdminItem> {
        &data.guide.items.items
    }
}

make_post_impl!(ItemFilters);

/// Query for items.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/items", format = "json", data = "<filters>")]
pub fn post(filters: Json<ItemFilters>) -> MaybeResponse {
    MaybeResponse {
        contents: post_impl(filters.into_inner()),
    }
}

/// This route is needded when making a CORS call to the API.
#[options("/items")]
pub fn options() -> &'static str {
    ""
}
