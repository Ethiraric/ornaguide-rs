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

/// Response for an assessat request.
#[derive(Serialize)]
pub struct AssessatResponse {
    /// Base stats for the item.
    pub base_item: &'static AdminItem,
    /// Stats for the item at level 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, MF, DF, GF.
    pub stats: Vec<AssessatStats>,
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

/// Implementation method for `/assessat`.
/// Performs request checks to ensure the assessment can proceed without error.
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

/// Assess an item at the given quality.
/// This function holds the logic behind the `/assessat` call. Logic is extracted here for ease of
/// testing.
pub fn assessat(
    item: &'static AdminItem,
    quality: u8,
    quality_tier: QualityTier,
) -> AssessatResponse {
    AssessCtx::assess(item, quality, quality_tier)
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
    pub ward: i16,
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

/// Item stats, expressed as floats. Used for computations.
#[derive(Clone, Default)]
struct FloatStats {
    pub hp: f32,
    pub mana: f32,
    pub attack: f32,
    pub magic: f32,
    pub defense: f32,
    pub resistance: f32,
    pub ward: f32,
    pub foresight: f32,
}

/// Boni granted by an item at level 1.
#[derive(Default)]
struct Boni {
    pub orn_bonus: f32,
    pub gold_bonus: f32,
    pub drop_bonus: f32,
    pub spawn_bonus: f32,
    pub exp_bonus: f32,
}

/// Context data for an assessment.
struct AssessCtx {
    /// The base stats (100% quality) of the item to assess.
    item: &'static AdminItem,
    /// The quality at which to assess.
    quality: u8,
    /// The quality tier of the item at levels 1-10.
    quality_tier: QualityTier,
    /// Whether the item to assess is an adornment.
    is_adorn: bool,
    /// Stats of the item at level 1.
    lv1_stats: FloatStats,
    /// Boni granted by the item at level 1.
    lv1_boni: Boni,
    /// By how much stats increase per level,
    increment: FloatStats,
    /// Route response.
    response: AssessatResponse,
}

impl AssessCtx {
    /// Create a new assess context for the given item and quality.
    fn new(item: &'static AdminItem, quality: u8, quality_tier: QualityTier) -> Self {
        AssessCtx {
            item,
            quality,
            quality_tier,
            lv1_stats: FloatStats::default(),
            lv1_boni: Boni::default(),
            increment: FloatStats::default(),
            is_adorn: item.type_ == 11, /* Adornment */
            response: AssessatResponse {
                base_item: item,
                stats: vec![],
            },
        }
    }

    /// Compute the base stats of the item at the given quality and assign to self.
    fn compute_lv1_stats(&mut self) {
        let quality_ratio = 1.0 + (self.quality as f32 / 100.0);
        self.lv1_stats = FloatStats {
            hp: base_stat_at(
                self.item.hp_affected_by_quality,
                self.item.hp,
                quality_ratio,
            ),
            mana: base_stat_at(
                self.item.mana_affected_by_quality,
                self.item.mana,
                quality_ratio,
            ),
            attack: base_stat_at(
                self.item.attack_affected_by_quality,
                self.item.attack,
                quality_ratio,
            ),
            magic: base_stat_at(
                self.item.magic_affected_by_quality,
                self.item.magic,
                quality_ratio,
            ),
            defense: base_stat_at(
                self.item.defense_affected_by_quality,
                self.item.defense,
                quality_ratio,
            ),
            resistance: base_stat_at(
                self.item.resistance_affected_by_quality,
                self.item.resistance,
                quality_ratio,
            ),
            ward: base_stat_at(
                self.item.ward_affected_by_quality,
                self.item.ward,
                quality_ratio,
            ),
            foresight: base_stat_at(true, self.item.foresight, quality_ratio),
        };
    }

    /// Compute the boni granted by the item at level 1.
    fn compute_lv1_boni(&mut self) {
        self.lv1_boni = Boni {
            orn_bonus: self.quality_tier.bonus(self.is_adorn, self.item.orn_bonus),
            gold_bonus: self.quality_tier.bonus(self.is_adorn, self.item.gold_bonus),
            drop_bonus: self.quality_tier.bonus(self.is_adorn, self.item.drop_bonus),
            /// TODO(ethiraric, 22/03/2023): How does that even scale?
            spawn_bonus: self.item.spawn_bonus,
            exp_bonus: self.quality_tier.bonus(self.is_adorn, self.item.exp_bonus),
        };
    }

    /// Compute the stats increment per level and assign it to self.
    fn compute_increment(&mut self) {
        let ratio = if self.item.boss { 0.125f32 } else { 0.1 };
        self.increment = FloatStats {
            hp: (self.item.hp as f32 * ratio).ceil(),
            mana: (self.item.mana as f32 * ratio).ceil(),
            attack: (self.item.attack as f32 * ratio).ceil(),
            magic: (self.item.magic as f32 * ratio).ceil(),
            defense: (self.item.defense as f32 * ratio).ceil(),
            resistance: (self.item.resistance as f32 * ratio).ceil(),
            ward: (self.item.ward as f32 * ratio).ceil(),
            foresight: (self.item.foresight as f32 * ratio).ceil(),
        }
    }

    /// Compute stats for each level and add the data in `response`.
    fn populate_response(&mut self) {
        // Start with lv1 stats.
        let mut stats = self.lv1_stats.clone();
        // We can immediately add them to the response.
        self.response
            .stats
            .push(AssessatStats::new_from(&stats, &self.lv1_boni, 0));

        // Level 1 -> 2 adds the increment twice. We need to add once before looping.
        stats.add(&self.increment);

        // From level 2 to 10 (included).
        for level in 2..11 {
            // Add the increment...
            stats.add(&self.increment);
            // ... and push the stats.
            self.response.stats.push(AssessatStats::new_from(
                &stats,
                &self.lv1_boni,
                adorn_slots_at(self.item, level, self.quality_tier),
            ));
        }
    }

    /// Assess an item at the given quality.
    fn assess(
        item: &'static AdminItem,
        quality: u8,
        quality_tier: QualityTier,
    ) -> AssessatResponse {
        let mut ctx = Self::new(item, quality, quality_tier);
        ctx.compute_lv1_stats();
        ctx.compute_lv1_boni();
        ctx.compute_increment();
        ctx.populate_response();
        ctx.response
    }
}

impl AssessatStats {
    /// Create a new `AssessatStats` from fragments of data given as parameters.
    fn new_from(stats: &FloatStats, boni: &Boni, adornment_slots: u8) -> Self {
        Self {
            hp: stats.hp.ceil() as i16,
            mana: stats.mana.ceil() as i16,
            attack: stats.attack.ceil() as i16,
            magic: stats.magic.ceil() as i16,
            defense: stats.defense.ceil() as i16,
            resistance: stats.resistance.ceil() as i16,
            ward: stats.ward.ceil() as i16,
            foresight: stats.hp.ceil() as i16,
            adornment_slots,
            orn_bonus: boni.orn_bonus,
            gold_bonus: boni.gold_bonus,
            drop_bonus: boni.drop_bonus,
            spawn_bonus: boni.spawn_bonus,
            exp_bonus: boni.exp_bonus,
        }
    }
}

impl FloatStats {
    /// Add stats to `self`.
    fn add(&mut self, other: &FloatStats) {
        self.hp += other.hp;
        self.mana += other.mana;
        self.attack += other.attack;
        self.magic += other.magic;
        self.defense += other.defense;
        self.resistance += other.resistance;
        self.ward += other.ward;
        self.foresight += other.hp;
    }
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
        //       ^                ^           ^  ^ rescale to a percentage
        //       |                |           ` remove the 1 we added earlier
        //       |                ` apply quality modifier
        //       ` convert to proportion and add 1 (100%)
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

    /// Return the number of bonus adornment slots to the item given by the quality tier.
    pub fn bonus_adorns(&self) -> u8 {
        match self {
            QualityTier::Broken => 0,
            QualityTier::Poor => 0,
            QualityTier::Common => 0,
            QualityTier::Superior => 1,
            QualityTier::Famed => 1,
            QualityTier::Legendary => 1,
            QualityTier::Ornate => 2,
            QualityTier::Masterforged => 3,
            QualityTier::Demonforged => 3,
            QualityTier::Godforged => 4,
            QualityTier::Impossible => 0,
        }
    }
}

/// Compute the number of adornment slots for an item at the given level and quality.
fn adorn_slots_at(item: &AdminItem, level: i16, quality_tier: QualityTier) -> u8 {
    // Compute the max adorn slots at level 10.
    let mut max_adorn_slots = item.base_adornment_slots;
    // For some reason, items of tiers 1 and 2, and off-hands do not scale their adorn slots with
    // quality.
    if item.type_ !=  10 /* Off-hand */ && item.tier > 2 {
        max_adorn_slots += quality_tier.bonus_adorns()
    };

    // If the item is level 10, it unlocks all slots. This was an issue with some Ornate items who
    // could have 10+ slots at level 10. The default formula only allows for up to 9 slots at level
    // 10.
    if level == 10
        || [
            QualityTier::Masterforged,
            QualityTier::Demonforged,
            QualityTier::Godforged,
        ]
        .contains(&quality_tier)
    {
        max_adorn_slots
    } else {
        ((level - 1) as u8).min(max_adorn_slots)
    }
}

/// Compute a base stat of an item at the given quality ratio.
fn base_stat_at<T: NumCast>(affected: bool, base_stat: T, quality_ratio: f32) -> f32 {
    if affected {
        <f32 as num::NumCast>::from(base_stat).unwrap() * quality_ratio
    } else {
        <f32 as num::NumCast>::from(base_stat).unwrap()
    }
}
