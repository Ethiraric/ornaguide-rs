use crate::error::Error;

pub(crate) mod html_item_parser;
pub(crate) mod html_list_parser;
pub(crate) mod html_monster_parser;
pub(crate) mod html_skill_parser;

pub use html_item_parser::{
    Ability as ItemAbility, DroppedBy as ItemDroppedBy, Item as CodexItem, Stats as ItemStats,
    UpgradeMaterial as ItemUpgradeMaterial,
};
pub use html_monster_parser::{
    Ability as MonsterAbility, CodexBoss, CodexMonster, Drop as MonsterDrop,
};
pub use html_skill_parser::{CodexSkill, StatusEffect};

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
pub struct ItemEntry {
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

    /// Retrieve the list of items from the orna codex.
    fn codex_fetch_item_list(&self) -> Result<Vec<ItemEntry>, Error>;
    /// Retrieve the details about a item from the orna codex.
    fn codex_fetch_item(&self, item_name: &str) -> Result<CodexItem, Error>;
}
