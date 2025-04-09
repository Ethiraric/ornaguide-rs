use crate::error::Error;

pub(crate) mod affix;
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
pub mod translation;

#[allow(clippy::module_name_repetitions)]
pub use affix::Affix as CodexAffix;
#[allow(clippy::module_name_repetitions)]
pub use follower::{
    Ability as FollowerAbility, Follower as CodexFollower, Followers as CodexFollowers,
};
#[allow(clippy::module_name_repetitions)]
pub use item::{
    Ability as ItemAbility, DroppedBy as ItemDroppedBy, Element as CodexElement, Item as CodexItem,
    ItemStatusEffects, Items as CodexItems, Stats as ItemStats,
    UpgradeMaterial as ItemUpgradeMaterial,
};
#[allow(clippy::module_name_repetitions)]
pub use monster::{
    Ability as MonsterAbility, Boss as CodexBoss, Bosses as CodexBosses, Drop as MonsterDrop,
    Monster as CodexMonster, Monsters as CodexMonsters, Raid as CodexRaid, Raids as CodexRaids,
    Tag,
};
#[allow(clippy::module_name_repetitions)]
pub use skill::{CodexSkill, CodexSkills, SkillStatusEffect, SkillStatusEffects, SkillSummon};

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

/// A trait to implement for things we can get a slug from.
pub trait Sluggable {
    /// Return the slug that corresponds to the entity.
    fn slug(&self) -> &str;
}

impl Sluggable for SkillEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/spells/".len()..self.uri.len() - 1]
    }
}

impl Sluggable for MonsterEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/monsters/".len()..self.uri.len() - 1]
    }
}

impl Sluggable for BossEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/bosses/".len()..self.uri.len() - 1]
    }
}

impl Sluggable for RaidEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/raids/".len()..self.uri.len() - 1]
    }
}

impl Sluggable for ItemEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/items/".len()..self.uri.len() - 1]
    }
}

impl Sluggable for FollowerEntry {
    fn slug(&self) -> &str {
        &self.uri["/codex/followers/".len()..self.uri.len() - 1]
    }
}

/// The public codex on `playorna.com`.
pub trait Codex {
    /// Retrieve the list of skills from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_skill_list(&self) -> Result<Vec<SkillEntry>, Error>;
    /// Retrieve the page of a skill from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_skill_page(&self, skill_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a skill from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_skill(&self, skill_name: &str) -> Result<CodexSkill, Error>;

    /// Retrieve the list of monsters from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_monster_list(&self) -> Result<Vec<MonsterEntry>, Error>;
    /// Retrieve the page of a monster from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_monster_page(&self, monster_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a monster from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_monster(&self, monster_name: &str) -> Result<CodexMonster, Error>;

    /// Retrieve the list of bosses from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_boss_list(&self) -> Result<Vec<BossEntry>, Error>;
    /// Retrieve the page of a boss from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_boss_page(&self, boss_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a boss from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_boss(&self, boss_name: &str) -> Result<CodexBoss, Error>;

    /// Retrieve the list of raids from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_raid_list(&self) -> Result<Vec<RaidEntry>, Error>;
    /// Retrieve the page of a raid from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_raid_page(&self, raid_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a raid from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_raid(&self, raid_name: &str) -> Result<CodexRaid, Error>;

    /// Retrieve the list of items from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_item_list(&self) -> Result<Vec<ItemEntry>, Error>;
    /// Retrieve the page of a item from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_item_page(&self, item_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a item from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_item(&self, item_name: &str) -> Result<CodexItem, Error>;

    /// Retrieve the list of followers from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page(s) failed.
    /// This function does not return partial results.
    fn codex_fetch_follower_list(&self) -> Result<Vec<FollowerEntry>, Error>;
    /// Retrieve the page of a follower from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error.
    fn codex_fetch_follower_page(&self, follower_name: &str) -> Result<String, Error>;
    /// Retrieve the details about a follower from the orna codex.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_follower(&self, follower_name: &str) -> Result<CodexFollower, Error>;

    // Locale-aware methods

    /// Retrieve the details about a skill from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - tags
    ///   - "causes"/"gives": Both are put into `causes`.
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_skill_with_locale(
        &self,
        skill_name: &str,
        locale: &str,
    ) -> Result<CodexSkill, Error>;
    /// Retrieve the details about a monster from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - abilities
    ///   - drops
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_monster_with_locale(
        &self,
        monster_name: &str,
        locale: &str,
    ) -> Result<CodexMonster, Error>;
    /// Retrieve the details about a boss from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - abilities
    ///   - drops
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_boss_with_locale(
        &self,
        boss_name: &str,
        locale: &str,
    ) -> Result<CodexBoss, Error>;
    /// Retrieve the details about a raid from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - abilities
    ///   - drops
    ///   - tags
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_raid_with_locale(
        &self,
        raid_name: &str,
        locale: &str,
    ) -> Result<CodexRaid, Error>;
    /// Retrieve the details about a item from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - stats
    ///   - causes
    ///   - cures
    ///   - gives
    ///   - immunities
    ///   - dropped_by
    ///   - upgrade_materials
    ///   - tags
    ///   - ability
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    #[allow(clippy::doc_markdown)]
    fn codex_fetch_item_with_locale(
        &self,
        item_name: &str,
        locale: &str,
    ) -> Result<CodexItem, Error>;
    /// Retrieve the details about a follower from the orna codex in the given locale.
    /// Only some fields are returned. Fields that cannot be accurately parsed are left blank.
    /// Fields ignored:
    ///   - abilities
    ///
    /// # Errors
    /// Errors if there was an I/O error or if parsing the page failed.
    fn codex_fetch_follower_with_locale(
        &self,
        follower_name: &str,
        locale: &str,
    ) -> Result<CodexFollower, Error>;
}
