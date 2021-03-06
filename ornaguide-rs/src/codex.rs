use crate::error::Error;

pub(crate) mod follower;
pub(crate) mod html_follower_parser;
pub(crate) mod html_item_parser;
pub(crate) mod html_list_parser;
pub(crate) mod html_monster_parser;
pub(crate) mod html_skill_parser;
pub(crate) mod item;
pub(crate) mod monster;
pub(crate) mod skill;

pub mod fetch;

pub use follower::{
    Ability as FollowerAbility, Follower as CodexFollower, Followers as CodexFollowers,
};
pub use item::{
    Ability as ItemAbility, DroppedBy as ItemDroppedBy, Element as CodexElement, Item as CodexItem,
    ItemStatusEffects, Items as CodexItems, Stats as ItemStats,
    UpgradeMaterial as ItemUpgradeMaterial,
};
pub use monster::{
    Ability as MonsterAbility, Boss as CodexBoss, Bosses as CodexBosses, Drop as MonsterDrop,
    Monster as CodexMonster, Monsters as CodexMonsters, Raid as CodexRaid, Raids as CodexRaids,
    Tag,
};
pub use skill::{CodexSkill, CodexSkills, SkillStatusEffect, SkillStatusEffects};

#[derive(Debug)]
pub struct SkillEntry {
    pub name: String,
    pub tier: u32,
    pub uri: String,
}

#[derive(Debug)]
pub struct MonsterEntry {
    pub name: String,
    pub family: String,
    pub tier: u32,
    pub uri: String,
}

#[derive(Debug)]
pub struct BossEntry {
    pub name: String,
    pub family: String,
    pub tier: u32,
    pub uri: String,
}

#[derive(Debug)]
pub struct RaidEntry {
    pub name: String,
    pub tier: u32,
    pub uri: String,
}

#[derive(Debug)]
pub struct ItemEntry {
    pub name: String,
    pub tier: u32,
    pub uri: String,
}

#[derive(Debug)]
pub struct FollowerEntry {
    pub name: String,
    pub tier: u32,
    pub uri: String,
}

/// The public codex on `playorna.com`.
pub trait Codex {
    /// Retrieve the list of skills from the orna codex.
    fn codex_fetch_skill_list(&self) -> Result<Vec<SkillEntry>, Error>;
    /// Retrieve the details about a skill from the orna codex.
    fn codex_fetch_skill(&self, skill_name: &str) -> Result<CodexSkill, Error>;

    /// Retrieve the list of monsters from the orna codex.
    fn codex_fetch_monster_list(&self) -> Result<Vec<MonsterEntry>, Error>;
    /// Retrieve the details about a monster from the orna codex.
    fn codex_fetch_monster(&self, monster_name: &str) -> Result<CodexMonster, Error>;

    /// Retrieve the list of bosses from the orna codex.
    fn codex_fetch_boss_list(&self) -> Result<Vec<BossEntry>, Error>;
    /// Retrieve the details about a boss from the orna codex.
    fn codex_fetch_boss(&self, boss_name: &str) -> Result<CodexBoss, Error>;

    /// Retrieve the list of raids from the orna codex.
    fn codex_fetch_raid_list(&self) -> Result<Vec<RaidEntry>, Error>;
    /// Retrieve the details about a raid from the orna codex.
    fn codex_fetch_raid(&self, raid_name: &str) -> Result<CodexRaid, Error>;

    /// Retrieve the list of items from the orna codex.
    fn codex_fetch_item_list(&self) -> Result<Vec<ItemEntry>, Error>;
    /// Retrieve the details about a item from the orna codex.
    fn codex_fetch_item(&self, item_name: &str) -> Result<CodexItem, Error>;

    /// Retrieve the list of followers from the orna codex.
    fn codex_fetch_follower_list(&self) -> Result<Vec<FollowerEntry>, Error>;
    /// Retrieve the details about a follower from the orna codex.
    fn codex_fetch_follower(&self, follower_name: &str) -> Result<CodexFollower, Error>;
}
