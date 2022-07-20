//! Contains the different skill structures that are fetched from the API.
//!
//! The [`raw`] submodule contains the generic structure that we retrieve from the API. All fields
//! that _may_ be `null` in the json for _any_ skill of any kind are `Option`s.
//!
//! The [`skill`] submodule contains utilities to remove some of the `Option`s and categorize the
//! skills into passives and active skills.
//!
//! The [`admin`] submodule contains classes for the administration view of the guide.
//!
//! The Rust items from the [`raw`] and [`skill`] modules are publicly used in this module.

pub mod admin;
pub mod raw;
pub mod skill;

pub use raw::{RawSkill, RawSkills, SkillBuffedBy, SkillLearnedBy, SkillMonsterUse, SkillPetUse};
pub use skill::{
    AoeAttackSkill, AoeBuffSkill, AoeDebuffSkill, AoeMagicSkill, AttackSkill, BuffSkill,
    DebuffSkill, HealingSkill, MagicSkill, MultiRoundAttackSkill, MultiRoundMagicSkill, OtherSkill,
    PassiveSkill, Skill, WardSkill,
};
