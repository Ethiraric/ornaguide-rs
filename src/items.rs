//! Contains the different item structures that are fetched from the API.
//!
//! The [`raw`] submodule contains the generic structure that we retrieve from the API. All fields
//! that _may_ be `null` in the json for _any_ item of any kind is an `Option`.
//!
//! The [`item`] submodule contains utilities to remove some of the `Option`s and categorize the
//! items into the same categories that are in the in-game inventory.
//!
//! The Rust items from the [`raw`] and [`item`] modules are publicly used in this module.

pub mod item;
pub mod raw;

pub use item::{
    AccessoryItem, AdornmentItem, ArmorItem, CurativeItem, FishItem, HeadItem, Item, ItemItem,
    LegsItem, MaterialItem, OffHandItem, OtherItem, WeaponItem,
};
pub use raw::{
    ItemAttackStat, ItemCritStat, ItemDefenseStat, ItemDexterityStat, ItemDroppedBy,
    ItemEquippedBy, ItemHPStat, ItemMagicStat, ItemManaStat, ItemMaterial, ItemQuest,
    ItemResistanceStat, ItemStats, ItemWardStat, RawItem, RawItems,
};
