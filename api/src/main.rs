#[macro_use]
extern crate rocket;

extern crate lazy_static;

use itertools::Itertools;
use lazy_static::__Deref;
use ornaguide_rs::{error::Error, items::admin::AdminItem};
use rocket::{routes, serde::json::Json, Config};
use serde::{Deserialize, Serialize};

mod data;
mod filter;

use crate::{
    data::DATA,
    filter::{compilable::Compilable, Filter},
};

/// All the filters applicable on an item.
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
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
    // /// Filter by element.
    // pub element: Filter<'a, Option<u32>>,
    // /// Filter by equipped_by.
    // pub equipped_by: Filter<'a, Vec<u32>>,
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
    // /// Filter by exp_bonus.
    // pub exp_bonus: Filter<'a, Vec<f32>>,
    /// Filter by boss.
    pub boss: Filter<'a, bool>,
    /// Filter by arena.
    pub arena: Filter<'a, bool>,
    /// Filter by category.
    // pub category: Filter<'a, Option<u32>>,
    // /// Filter by causes.
    // pub causes: Filter<'a, Vec<u32>>,
    // /// Filter by cures.
    // pub cures: Filter<'a, Vec<u32>>,
    // /// Filter by gives.
    // pub gives: Filter<'a, Vec<u32>>,
    // /// Filter by prevents.
    // pub prevents: Filter<'a, Vec<u32>>,
    // /// Filter by materials.
    // pub materials: Filter<'a, Vec<u32>>,
    /// Filter by price.
    pub price: Filter<'a, u32>,
    // /// Filter by ability.
    // pub ability: Filter<'a, Option<u32>>,
}

impl<'a> ItemFilters<'a> {
    /// Compile all filters within `self`.
    pub fn compiled(self) -> Result<Self, Error> {
        Ok(Self {
            id: self.id.compiled()?,
            codex_uri: self.codex_uri.compiled()?,
            attack: self.attack.compiled()?,
            name: self.name.compiled()?,
            tier: self.tier.compiled()?,
            type_: self.type_.compiled()?,
            image_name: self.image_name.compiled()?,
            description: self.description.compiled()?,
            notes: self.notes.compiled()?,
            hp: self.hp.compiled()?,
            hp_affected_by_quality: self.hp_affected_by_quality.compiled()?,
            mana: self.mana.compiled()?,
            mana_affected_by_quality: self.mana_affected_by_quality.compiled()?,
            attack_affected_by_quality: self.attack_affected_by_quality.compiled()?,
            magic: self.magic.compiled()?,
            magic_affected_by_quality: self.magic_affected_by_quality.compiled()?,
            defense: self.defense.compiled()?,
            defense_affected_by_quality: self.defense_affected_by_quality.compiled()?,
            resistance: self.resistance.compiled()?,
            resistance_affected_by_quality: self.resistance_affected_by_quality.compiled()?,
            dexterity: self.dexterity.compiled()?,
            dexterity_affected_by_quality: self.dexterity_affected_by_quality.compiled()?,
            ward: self.ward.compiled()?,
            ward_affected_by_quality: self.ward_affected_by_quality.compiled()?,
            crit: self.crit.compiled()?,
            crit_affected_by_quality: self.crit_affected_by_quality.compiled()?,
            foresight: self.foresight.compiled()?,
            view_distance: self.view_distance.compiled()?,
            follower_stats: self.follower_stats.compiled()?,
            follower_act: self.follower_act.compiled()?,
            status_infliction: self.status_infliction.compiled()?,
            status_protection: self.status_protection.compiled()?,
            mana_saver: self.mana_saver.compiled()?,
            has_slots: self.has_slots.compiled()?,
            base_adornment_slots: self.base_adornment_slots.compiled()?,
            rarity: self.rarity.compiled()?,
            // element: self.element.compiled()?,
            // equipped_by: self.equipped_by.compiled()?,
            two_handed: self.two_handed.compiled()?,
            orn_bonus: self.orn_bonus.compiled()?,
            gold_bonus: self.gold_bonus.compiled()?,
            drop_bonus: self.drop_bonus.compiled()?,
            spawn_bonus: self.spawn_bonus.compiled()?,
            // exp_bonus: self.exp_bonus.compiled()?,
            boss: self.boss.compiled()?,
            arena: self.arena.compiled()?,
            // category: self.category.compiled()?,
            // causes: self.causes.compiled()?,
            // cures: self.cures.compiled()?,
            // gives: self.gives.compiled()?,
            // prevents: self.prevents.compiled()?,
            // materials: self.materials.compiled()?,
            price: self.price.compiled()?,
            // ability: self.ability.compiled()?,
        })
    }

    // Run an item through all filters to see if it matches it.
    pub fn filter(&self, item: &AdminItem) -> bool {
        self.id.filter(&item.id) &&
            self.codex_uri.filter(&item.codex_uri) &&
            self.attack.filter(&item.attack) &&
            self.name.filter(&item.name) &&
            self.tier.filter(&item.tier) &&
            self.type_.filter(&item.type_) &&
            self.image_name.filter(&item.image_name) &&
            self.description.filter(&item.description) &&
            self.notes.filter(&item.notes) &&
            self.hp.filter(&item.hp) &&
            self.hp_affected_by_quality.filter(&item.hp_affected_by_quality) &&
            self.mana.filter(&item.mana) &&
            self.mana_affected_by_quality.filter(&item.mana_affected_by_quality) &&
            self.attack_affected_by_quality.filter(&item.attack_affected_by_quality) &&
            self.magic.filter(&item.magic) &&
            self.magic_affected_by_quality.filter(&item.magic_affected_by_quality) &&
            self.defense.filter(&item.defense) &&
            self.defense_affected_by_quality.filter(&item.defense_affected_by_quality) &&
            self.resistance.filter(&item.resistance) &&
            self.resistance_affected_by_quality.filter(&item.resistance_affected_by_quality) &&
            self.dexterity.filter(&item.dexterity) &&
            self.dexterity_affected_by_quality.filter(&item.dexterity_affected_by_quality) &&
            self.ward.filter(&item.ward) &&
            self.ward_affected_by_quality.filter(&item.ward_affected_by_quality) &&
            self.crit.filter(&item.crit) &&
            self.crit_affected_by_quality.filter(&item.crit_affected_by_quality) &&
            self.foresight.filter(&item.foresight) &&
            self.view_distance.filter(&item.view_distance) &&
            self.follower_stats.filter(&item.follower_stats) &&
            self.follower_act.filter(&item.follower_act) &&
            self.status_infliction.filter(&item.status_infliction) &&
            self.status_protection.filter(&item.status_protection) &&
            self.mana_saver.filter(&item.mana_saver) &&
            self.has_slots.filter(&item.has_slots) &&
            self.base_adornment_slots.filter(&item.base_adornment_slots) &&
            self.rarity.filter(&item.rarity) &&
            // self.element.filter(&item.element) &&
            // self.equipped_by.filter(&item.equipped_by) &&
            self.two_handed.filter(&item.two_handed) &&
            self.orn_bonus.filter(&item.orn_bonus) &&
            self.gold_bonus.filter(&item.gold_bonus) &&
            self.drop_bonus.filter(&item.drop_bonus) &&
            self.spawn_bonus.filter(&item.spawn_bonus) &&
            // self.exp_bonus.filter(&item.exp_bonus) &&
            self.boss.filter(&item.boss) &&
            self.arena.filter(&item.arena) &&
            // self.category.filter(&item.category) &&
            // self.causes.filter(&item.causes) &&
            // self.cures.filter(&item.cures) &&
            // self.gives.filter(&item.gives) &&
            // self.prevents.filter(&item.prevents) &&
            // self.materials.filter(&item.materials) &&
            self.price.filter(&item.price)
        // self.ability.filter(&item.ability)
    }

    /// Check whether all filters are set to `Filter::None`.
    pub fn is_none(&self) -> bool {
        self.id.is_none() &&
            self.codex_uri.is_none() &&
            self.attack.is_none() &&
            self.name.is_none() &&
            self.tier.is_none() &&
            self.type_.is_none() &&
            self.image_name.is_none() &&
            self.description.is_none() &&
            self.notes.is_none() &&
            self.hp.is_none() &&
            self.hp_affected_by_quality.is_none() &&
            self.mana.is_none() &&
            self.mana_affected_by_quality.is_none() &&
            self.attack_affected_by_quality.is_none() &&
            self.magic.is_none() &&
            self.magic_affected_by_quality.is_none() &&
            self.defense.is_none() &&
            self.defense_affected_by_quality.is_none() &&
            self.resistance.is_none() &&
            self.resistance_affected_by_quality.is_none() &&
            self.dexterity.is_none() &&
            self.dexterity_affected_by_quality.is_none() &&
            self.ward.is_none() &&
            self.ward_affected_by_quality.is_none() &&
            self.crit.is_none() &&
            self.crit_affected_by_quality.is_none() &&
            self.foresight.is_none() &&
            self.view_distance.is_none() &&
            self.follower_stats.is_none() &&
            self.follower_act.is_none() &&
            self.status_infliction.is_none() &&
            self.status_protection.is_none() &&
            self.mana_saver.is_none() &&
            self.has_slots.is_none() &&
            self.base_adornment_slots.is_none() &&
            self.rarity.is_none() &&
            // self.element.is_none() &&
            // self.equipped_by.is_none() &&
            self.two_handed.is_none() &&
            self.orn_bonus.is_none() &&
            self.gold_bonus.is_none() &&
            self.drop_bonus.is_none() &&
            self.spawn_bonus.is_none() &&
            // self.exp_bonus.is_none() &&
            self.boss.is_none() &&
            self.arena.is_none() &&
            // self.category.is_none() &&
            // self.causes.is_none() &&
            // self.cures.is_none() &&
            // self.gives.is_none() &&
            // self.prevents.is_none() &&
            // self.materials.is_none() &&
            self.price.is_none()
        // self.ability.is_none()
    }
}

#[post("/items", format = "json", data = "<filters>")]
fn post_items(filters: Json<ItemFilters>) -> Json<Vec<AdminItem>> {
    let lock = DATA.as_ref().unwrap();
    let lock = lock.read();
    let data = lock.as_ref().unwrap().deref();
    if filters.is_none() {
        Json(data.guide.items.items.clone())
    } else {
        let filters = filters.into_inner().compiled().unwrap();
        Json(
            data.guide
                .items
                .items
                .iter()
                .filter(|item| filters.filter(item))
                .cloned()
                .collect_vec(),
        )
    }
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

    rocket::custom(&config).mount("/api/v0.1", routes![post_items])
}
