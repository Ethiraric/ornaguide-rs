use serde::{Deserialize, Serialize};

use crate::codex::{CodexBoss, CodexFollower, CodexItem, CodexMonster, CodexRaid, CodexSkill};

use std::collections::HashMap;

/// A set of strings for a particular language.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LocaleStrings {
    // TODO(ethiraric, 09/08/2022): Make so entries don't have unused fields (e.g.: other than name
    // and description for items).
    /// The locale in which the structure is.
    pub locale: String,
    /// Items from the codex.
    pub items: HashMap<String, CodexItem>,
    /// Raids from the codex.
    pub raids: HashMap<String, CodexRaid>,
    /// Monsters from the codex.
    pub monsters: HashMap<String, CodexMonster>,
    /// Bosses from the codex.
    pub bosses: HashMap<String, CodexBoss>,
    /// Skills from the codex.
    pub skills: HashMap<String, CodexSkill>,
    /// Followers from the codex.
    pub followers: HashMap<String, CodexFollower>,
    /// Statuses that can be inflicted.
    /// The key is the English string, the value is that in the target locale.
    pub statuses: HashMap<String, String>,
    /// Event names.
    /// The key is the English string, the value is that in the target locale.
    pub events: HashMap<String, String>,
    /// Family names.
    /// The key is the English string, the value is that in the target locale.
    pub families: HashMap<String, String>,
    /// Rarity names.
    /// The key is the English string, the value is that in the target locale.
    pub rarities: HashMap<String, String>,
}

/// A set of `LocaleStrings`.
/// Strings organized in their respective locales.
pub struct LocaleDB {
    /// Map of locales. The key is the locale name.
    pub locales: HashMap<String, LocaleStrings>,
}

impl LocaleStrings {
    /// Get the given item from the locale database.
    pub fn item(&self, name: &str) -> Option<&CodexItem> {
        self.items.get(name)
    }
    /// Get the name of the given item from the locale database.
    pub fn item_name(&self, name: &str) -> Option<&str> {
        self.item(name).map(|item| item.name.as_str())
    }

    /// Get the description of the given item from the locale database.
    pub fn item_description(&self, item_name: &str) -> Option<&str> {
        self.item(item_name).map(|item| item.description.as_str())
    }

    /// Get the given raid from the locale database.
    pub fn raid(&self, name: &str) -> Option<&CodexRaid> {
        self.raids.get(name)
    }

    /// Get the name of the given raid from the locale database.
    pub fn raid_name(&self, name: &str) -> Option<&str> {
        self.raid(name).map(|raid| raid.name.as_str())
    }

    /// Get the description of the given raid from the locale database.
    pub fn raid_description(&self, raid_name: &str) -> Option<&str> {
        self.raid(raid_name).map(|raid| raid.description.as_str())
    }

    /// Get the given monster from the locale database.
    pub fn monster(&self, name: &str) -> Option<&CodexMonster> {
        self.monsters.get(name)
    }

    /// Get the name of the given monster from the locale database.
    pub fn monster_name(&self, name: &str) -> Option<&str> {
        self.monster(name).map(|monster| monster.name.as_str())
    }

    /// Get the given boss from the locale database.
    pub fn boss(&self, name: &str) -> Option<&CodexBoss> {
        self.bosses.get(name)
    }

    /// Get the name of the given boss from the locale database.
    pub fn boss_name(&self, name: &str) -> Option<&str> {
        self.boss(name).map(|boss| boss.name.as_str())
    }

    /// Get the given skill from the locale database.
    pub fn skill(&self, name: &str) -> Option<&CodexSkill> {
        self.skills.get(name)
    }

    /// Get the name of the given skill from the locale database.
    pub fn skill_name(&self, name: &str) -> Option<&str> {
        self.skill(name).map(|skill| skill.name.as_str())
    }

    /// Get the description of the given skill from the locale database.
    pub fn skill_description(&self, skill_name: &str) -> Option<&str> {
        self.skill(skill_name)
            .map(|skill| skill.description.as_str())
    }

    /// Get the given follower from the locale database.
    pub fn follower(&self, name: &str) -> Option<&CodexFollower> {
        self.followers.get(name)
    }

    /// Get the name of the given follower from the locale database.
    pub fn follower_name(&self, name: &str) -> Option<&str> {
        self.follower(name).map(|follower| follower.name.as_str())
    }

    /// Get the description of the given follower from the locale database.
    pub fn follower_description(&self, follower_name: &str) -> Option<&str> {
        self.follower(follower_name)
            .map(|follower| follower.description.as_str())
    }

    /// Get the status effect from the locale database.
    pub fn status(&self, name: &str) -> Option<&str> {
        self.statuses.get(name).map(String::as_str)
    }

    /// Get the event from the locale database.
    pub fn event(&self, name: &str) -> Option<&str> {
        self.events.get(name).map(String::as_str)
    }

    /// Get the family from the locale database.
    pub fn family(&self, name: &str) -> Option<&str> {
        self.families.get(name).map(String::as_str)
    }

    /// Get the rarity from the locale database.
    pub fn rarity(&self, name: &str) -> Option<&str> {
        self.rarities.get(name).map(String::as_str)
    }
}

impl LocaleDB {
    /// Get the given item from the locale database.
    pub fn item(&self, locale: &str, name: &str) -> Option<&CodexItem> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item(name))
    }
    /// Get the name of the given item from the locale database.
    pub fn item_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item_name(name))
    }

    /// Get the description of the given item from the locale database.
    pub fn item_description(&self, locale: &str, item_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item_description(item_name))
    }

    /// Get the given raid from the locale database.
    pub fn raid(&self, locale: &str, name: &str) -> Option<&CodexRaid> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid(name))
    }

    /// Get the name of the given raid from the locale database.
    pub fn raid_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid_name(name))
    }

    /// Get the description of the given raid from the locale database.
    pub fn raid_description(&self, locale: &str, raid_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid_description(raid_name))
    }

    /// Get the given monster from the locale database.
    pub fn monster(&self, locale: &str, name: &str) -> Option<&CodexMonster> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.monster(name))
    }

    /// Get the name of the given monster from the locale database.
    pub fn monster_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.monster_name(name))
    }

    /// Get the given boss from the locale database.
    pub fn boss(&self, locale: &str, name: &str) -> Option<&CodexBoss> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.boss(name))
    }

    /// Get the name of the given boss from the locale database.
    pub fn boss_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.boss_name(name))
    }

    /// Get the given skill from the locale database.
    pub fn skill(&self, locale: &str, name: &str) -> Option<&CodexSkill> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill(name))
    }

    /// Get the name of the given skill from the locale database.
    pub fn skill_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill_name(name))
    }

    /// Get the description of the given skill from the locale database.
    pub fn skill_description(&self, locale: &str, skill_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill_description(skill_name))
    }

    /// Get the given follower from the locale database.
    pub fn follower(&self, locale: &str, name: &str) -> Option<&CodexFollower> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower(name))
    }

    /// Get the name of the given follower from the locale database.
    pub fn follower_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower_name(name))
    }

    /// Get the description of the given follower from the locale database.
    pub fn follower_description(&self, locale: &str, follower_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower_description(follower_name))
    }

    /// Get the event from the locale database.
    pub fn event(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.event(name))
    }

    /// Get the family from the locale database.
    pub fn family(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.family(name))
    }

    /// Get the rarity from the locale database.
    pub fn rarity(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.rarity(name))
    }
}
