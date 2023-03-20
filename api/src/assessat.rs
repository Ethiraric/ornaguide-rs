use num::NumCast;
use ornaguide_rs::items::admin::AdminItem;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::{
    data::with_data,
    error::{og_error, Error, MaybeResponse},
};

/// The body of an assessat request.
#[derive(Serialize, Deserialize)]
pub struct AssessatRequest {
    /// The ID of the item to assess.
    item: u32,
    /// The quality of the item to assess.
    quality: u8,
}

/// Stats at a specific level.
#[derive(Serialize)]
pub struct AssessatStats {
    /// How much HP the item gives, if equippable.
    pub hp: i16,
    /// How much mana the item gives, if equippable.
    pub mana: i16,
    /// How much attack the item gives, if equippable.
    pub attack: i16,
    /// How much magic the item gives, if equippable.
    pub magic: i16,
    /// How much defense the item gives, if equippable.
    pub defense: i16,
    /// How much resistance the item gives, if equippable.
    pub resistance: i16,
    /// How much ward the item gives, if equippable (%).
    pub ward: i8,
    /// How much foresight the item gives, if equippable.
    pub foresight: i16,
    /// The number of adornment slots of the item has.
    pub adornment_slots: u8,
    /// How much more Orns you gain with this item (%).
    pub orn_bonus: f32,
    /// How much more Gold you gain with this item (%).
    pub gold_bonus: f32,
    /// How much more luck you have with this item (%).
    pub drop_bonus: f32,
    /// How much more spawns there are with this item (%).
    pub spawn_bonus: f32,
    /// How much more experience you gain with this item (%).
    pub exp_bonus: f32,
}

/// Response for an assessat request.
#[derive(Default, Serialize)]
pub struct AssessatResponse {
    /// Base stats for the item.
    pub base_item: AdminItem,
    /// Stats for the item at level 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, MF, DF, GF.
    pub stats: Vec<AssessatStats>,
}

/// Computes a base stat of an item at the given quality.
fn base_stat_at<T: NumCast>(affected: bool, base_stat: T, quality: u8) -> T {
    if affected {
        T::from(<i32 as num::NumCast>::from(base_stat).unwrap() * quality as i32 / 100).unwrap()
    } else {
        base_stat
    }
}

/// Computes the base stats of an item at the given quality.
fn base_stats_at(item: &AdminItem, quality: u8) -> AssessatStats {
    AssessatStats {
        hp: base_stat_at(item.hp_affected_by_quality, item.hp, quality),
        mana: base_stat_at(item.mana_affected_by_quality, item.mana, quality),
        attack: base_stat_at(item.attack_affected_by_quality, item.attack, quality),
        magic: base_stat_at(item.magic_affected_by_quality, item.magic, quality),
        defense: base_stat_at(item.defense_affected_by_quality, item.defense, quality),
        resistance: base_stat_at(
            item.resistance_affected_by_quality,
            item.resistance,
            quality,
        ),
        ward: base_stat_at(item.ward_affected_by_quality, item.ward, quality),
        foresight: base_stat_at(true, item.foresight, quality),
        adornment_slots: 0,
        orn_bonus: item.orn_bonus,
        gold_bonus: item.gold_bonus,
        drop_bonus: item.drop_bonus,
        spawn_bonus: item.spawn_bonus,
        exp_bonus: item.exp_bonus,
    }
}

/// Computes increments to the base stats at each level.
fn increments_from_base_stats(base_stats: &AssessatStats, boss: bool) -> AssessatStats {
    AssessatStats {
        hp: if boss {
            base_stats.hp / 8
        } else {
            base_stats.hp / 10
        },
        mana: if boss {
            base_stats.mana / 8
        } else {
            base_stats.mana / 10
        },
        attack: if boss {
            base_stats.attack / 8
        } else {
            base_stats.attack / 10
        },
        magic: if boss {
            base_stats.magic / 8
        } else {
            base_stats.magic / 10
        },
        defense: if boss {
            base_stats.defense / 8
        } else {
            base_stats.defense / 10
        },
        resistance: if boss {
            base_stats.resistance / 8
        } else {
            base_stats.resistance / 10
        },
        ward: if boss {
            base_stats.ward / 8
        } else {
            base_stats.ward / 10
        },
        foresight: if boss {
            base_stats.foresight / 8
        } else {
            base_stats.foresight / 10
        },
        adornment_slots: 0,
        orn_bonus: 0.0,
        gold_bonus: 0.0,
        drop_bonus: 0.0,
        spawn_bonus: 0.0,
        exp_bonus: 0.0,
    }
}

fn stats_at_level_x(
    base_stats: &AssessatStats,
    increment: &AssessatStats,
    level: i16,
) -> AssessatStats {
    AssessatStats {
        hp: base_stats.hp + increment.hp * level,
        mana: base_stats.mana + increment.mana * level,
        attack: base_stats.attack + increment.attack * level,
        magic: base_stats.magic + increment.magic * level,
        defense: base_stats.defense + increment.defense * level,
        resistance: base_stats.resistance + increment.resistance * level,
        ward: base_stats.ward + increment.ward * level as i8,
        foresight: base_stats.foresight + increment.foresight * level,
        adornment_slots: 0,
        orn_bonus: 0.0,
        gold_bonus: 0.0,
        drop_bonus: 0.0,
        spawn_bonus: 0.0,
        exp_bonus: 0.0,
    }
}

/// Assess an item at the given quality.
/// This function holds the logic behind the `/assessat` call. Logic is extracted here for ease of
/// testing.
pub fn assessat(item: &AdminItem, quality: u8) -> AssessatResponse {
    let mut response = AssessatResponse {
        base_item: item.clone(),
        ..Default::default()
    };

    let base_stats = base_stats_at(item, quality);
    let increment = increments_from_base_stats(&base_stats, item.boss);
    response.stats.push(base_stats);

    for i in 2..11 {
        response
            .stats
            .push(stats_at_level_x(&response.stats[0], &increment, i));
    }

    response
}

/// Implementation method for `/assessat`.
fn post_impl(request: AssessatRequest) -> Result<serde_json::Value, Error> {
    if request.quality > 200 {
        return Err(og_error(
            Status::BadRequest,
            format!("{}: Invalid quality", request.quality),
        ));
    }

    let response = match with_data(|data| Ok(data.guide.items.find_by_id(request.item)))? {
        Some(x) => assessat(x, request.quality),
        None => {
            return Err(og_error(
                Status::NotFound,
                format!("{}: Unknown item id", request.item),
            ))
        }
    };

    serde_json::to_value(response).map_err(|e| Error {
        status: Status::InternalServerError,
        error: e.into(),
    })
}

/// Query for items.
/// The `Content-Type` header must be set to `application/json` when calling this route.
/// Even when using no filter, the body should be an empty JSON object (`{}`).
#[post("/assessat", format = "json", data = "<request>")]
pub fn post(request: Json<AssessatRequest>) -> MaybeResponse {
    MaybeResponse {
        contents: post_impl(request.into_inner()),
    }
}
