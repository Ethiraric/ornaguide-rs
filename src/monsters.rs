//! Contains the different monster structures that are fetched from the API.
//!
//! The [`raw`] submodule contains the generic structure that we retrieve from the API. All fields
//! that _may_ be `null` in the json for _any_ monster of any kind is an `Option`.
//!
//! The [`monster`] submodule contains utilities to remove some of the `Option`s and categorize the
//! monsters into the same categories that are in the in-game inventory.
//!
//! The [`admin`] submodule contains classes for the administration view of the guide.
//!
//! The Rust monsters from the [`raw`] and [`monster`] modules are publicly used in this module.

// pub mod admin;
pub mod monster;
pub mod raw;

// pub use monster::{
//     AccessoryMonster, AdornmentMonster, ArmorMonster, CurativeMonster, FishMonster, HeadMonster, Monster, MonsterMonster,
//     LegsMonster, MaterialMonster, OffHandMonster, OtherMonster, WeaponMonster,
// };
pub use raw::{MonsterBuff, MonsterDrop, MonsterQuest, MonsterSkill, RawMonster, RawMonsters};
