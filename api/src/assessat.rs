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

/// Quality tier of an item.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QualityTier {
    Broken,
    Poor,
    Common,
    Superior,
    Famed,
    Legendary,
    Ornate,
    Masterforged,
    Demonforged,
    Godforged,
    Impossible,
}

impl QualityTier {
    /// Return the quality tier of an item according to its quality percent.
    /// As per Dangy in the FAQ:
    ///   - Broken: 70-90%
    ///   - Poor: 90-99%
    ///   - Common: 100%
    ///   - Superior: 110-120%
    ///   - Famed: 120-130%
    ///   - Legendary: 140-170%
    ///   - Ornate: 170-200%
    /// In the event a quality belongs to two tiers, it is assigned the lowest tier (e.g.: 170%
    /// will return Legendary).
    pub fn from_percent(percent: u8) -> Self {
        match percent {
            percent if percent < 70 => QualityTier::Impossible,
            percent if percent <= 90 => QualityTier::Broken,
            percent if percent < 100 => QualityTier::Poor,
            percent if percent == 100 => QualityTier::Common,
            percent if percent < 110 => QualityTier::Impossible,
            percent if percent <= 120 => QualityTier::Superior,
            percent if percent <= 130 => QualityTier::Famed,
            percent if percent < 140 => QualityTier::Impossible,
            percent if percent <= 170 => QualityTier::Legendary,
            percent if percent <= 200 => QualityTier::Ornate,
            _ => QualityTier::Impossible,
        }
    }

    /// Return the bonus multiplier associated to the given quality tier.
    pub fn bonus_multiplier(&self) -> f32 {
        match self {
            QualityTier::Broken => 0.1,
            QualityTier::Poor => 1.0,
            QualityTier::Common => 1.0,
            QualityTier::Superior => 1.10,
            QualityTier::Famed => 1.15,
            QualityTier::Legendary => 1.20,
            QualityTier::Ornate => 1.25,
            QualityTier::Masterforged => 1.30,
            QualityTier::Demonforged => 1.40,
            QualityTier::Godforged => 1.50,
            QualityTier::Impossible => 0.0,
        }
    }

    /// Return the bonus% of an item of `self` quality tier with the given base bonus percent.
    /// For adornments, use `adorn_bonus`. They follow a different formula.
    pub fn item_bonus(&self, base_bonus: f32) -> f32 {
        // The formula, with the base B expressed as a percent (ranging from 1 to 100) is:
        //      ((base / 100 + 1) * quality - 1) * 100
        //       |                |           |  ^ rescale to a percentage
        //       |                |           ^ remove the 1 we added earlier
        //       |                ^ apply quality modifier
        //       ^ convert to proportion and add 1 (100%)
        //         i.e.: get (gain with bonus) / (gain without bonus)
        //               with base = 25%, we get 1.25
        // Courtesy of Rubenir.
        (base_bonus / 100.0 + 1.0 * self.bonus_multiplier() - 1.0) * 100.0
    }

    /// Return the bonus% of an adornment of `self` quality tier with the given base bonus percent.
    /// For items, use `item_bonus`. They follow a different formula.
    pub fn adorn_bonus(&self, base_bonus: f32) -> f32 {
        // The formula is simply B * quality.
        // Courtesy of Rubenir.
        base_bonus * self.bonus_multiplier()
    }

    /// Call either `item_bonus` or `adorn_bonus`.
    pub fn bonus(&self, is_adorn: bool, base_bonus: f32) -> f32 {
        if !is_adorn {
            self.item_bonus(base_bonus)
        } else {
            self.adorn_bonus(base_bonus)
        }
    }
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
fn base_stats_at(
    item: &AdminItem,
    quality: u8,
    quality_tier: QualityTier,
    is_adorn: bool,
) -> AssessatStats {
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
        orn_bonus: quality_tier.bonus(is_adorn, item.orn_bonus),
        gold_bonus: quality_tier.bonus(is_adorn, item.gold_bonus),
        drop_bonus: quality_tier.bonus(is_adorn, item.drop_bonus),
        spawn_bonus: item.spawn_bonus,
        exp_bonus: quality_tier.bonus(is_adorn, item.exp_bonus),
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

/// Computes the stats of an item at a given level.
/// The level must be between 1 and 10 (included). This function will return erroneous results for
/// Masterforged, Demonforged and Godforged items.
fn stats_at_level_x(
    item: &AdminItem,
    base_stats: &AssessatStats,
    increment: &AssessatStats,
    level: i16,
    quality_tier: QualityTier,
    is_adorn: bool,
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
        orn_bonus: quality_tier.bonus(is_adorn, item.orn_bonus),
        gold_bonus: quality_tier.bonus(is_adorn, item.gold_bonus),
        drop_bonus: quality_tier.bonus(is_adorn, item.drop_bonus),
        /// TODO(ethiraric): How does that even scale?
        spawn_bonus: item.spawn_bonus,
        exp_bonus: quality_tier.bonus(is_adorn, item.exp_bonus),
    }
}

/// Assess an item at the given quality.
/// This function holds the logic behind the `/assessat` call. Logic is extracted here for ease of
/// testing.
pub fn assessat(item: &AdminItem, quality: u8, quality_tier: QualityTier) -> AssessatResponse {
    let mut response = AssessatResponse {
        base_item: item.clone(),
        ..Default::default()
    };

    let is_adorn = item.type_ == 11 /* Adornment */;
    let base_stats = base_stats_at(item, quality, quality_tier, is_adorn);
    let increment = increments_from_base_stats(&base_stats, item.boss);
    response.stats.push(base_stats);

    for i in 2..11 {
        response.stats.push(stats_at_level_x(
            item,
            &response.stats[0],
            &increment,
            i,
            quality_tier,
            is_adorn,
        ));
    }

    response
}

/// Implementation method for `/assessat`.
fn post_impl(request: AssessatRequest) -> Result<serde_json::Value, Error> {
    let quality_tier = QualityTier::from_percent(request.quality);
    if quality_tier == QualityTier::Impossible {
        return Err(og_error(
            Status::BadRequest,
            format!("{}: Invalid quality", request.quality),
        ));
    }

    let response = match with_data(|data| Ok(data.guide.items.find_by_id(request.item)))? {
        Some(x) => assessat(x, request.quality, quality_tier),
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
