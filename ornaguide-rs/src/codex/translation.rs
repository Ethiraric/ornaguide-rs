use serde::{Deserialize, Serialize};

use crate::{
    error::Error, items::admin::AdminItem, monsters::admin::AdminMonster, pets::admin::AdminPet,
    skills::admin::AdminSkill,
};

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

/// Holds strings that can be translated for an item.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemTranslation {
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    pub description: String,
}

/// Holds strings that can be translated for a raid.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RaidTranslation {
    /// The name of the raid.
    pub name: String,
    /// The description of the raid.
    pub description: String,
}

/// Holds strings that can be translated for a boss.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BossTranslation {
    /// The name of the boss.
    pub name: String,
}

/// Holds strings that can be translated for a monster.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterTranslation {
    /// The name of the monster.
    pub name: String,
}

/// Holds strings that can be translated for any monster.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GenericMonsterTranslation {
    /// `self` refers to a monster.
    Monster(MonsterTranslation),
    /// `self` refers to a raid.
    Raid(RaidTranslation),
    /// `self` refers to a boss.
    Boss(BossTranslation),
}

/// Holds strings that can be translated for a skill.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillTranslation {
    /// The name of the skill.
    pub name: String,
    /// The description of the skill.
    pub description: String,
}

/// Holds strings that can be translated for a follower.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FollowerTranslation {
    /// The name of the follower.
    pub name: String,
    /// The description of the follower.
    pub description: String,
}

/// A set of strings for a particular language.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LocaleStrings {
    // TODO(ethiraric, 09/08/2022): Make so entries don't have unused fields (e.g.: other than name
    // and description for items).
    /// The locale in which the structure is.
    pub locale: String,
    /// Items from the codex.
    pub items: HashMap<String, ItemTranslation>,
    /// Raids from the codex.
    pub raids: HashMap<String, RaidTranslation>,
    /// Monsters from the codex.
    pub monsters: HashMap<String, MonsterTranslation>,
    /// Bosses from the codex.
    pub bosses: HashMap<String, BossTranslation>,
    /// Skills from the codex.
    pub skills: HashMap<String, SkillTranslation>,
    /// Followers from the codex.
    pub followers: HashMap<String, FollowerTranslation>,
    /// Statuses that can be inflicted.
    /// The key is the English string, the value is that in the target locale.
    pub statuses: HashMap<String, String>,
    /// Event names.
    /// The key is the English string, the value is that in the target locale.
    pub events: HashMap<String, String>,
    /// Spawn names.
    /// The key is the English string, the value is that in the target locale.
    pub spawns: HashMap<String, String>,
    /// Family names.
    /// The key is the English string, the value is that in the target locale.
    pub families: HashMap<String, String>,
    /// Rarity names.
    /// The key is the English string, the value is that in the target locale.
    pub rarities: HashMap<String, String>,
}

/// A set of `LocaleStrings`.
/// Strings organized in their respective locales.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LocaleDB {
    /// Map of locales. The key is the locale name.
    pub locales: HashMap<String, LocaleStrings>,
}

impl LocaleStrings {
    /// Get the given item from the locale database.
    pub fn item(&self, name: &str) -> Option<&ItemTranslation> {
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
    pub fn raid(&self, name: &str) -> Option<&RaidTranslation> {
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
    pub fn monster(&self, name: &str) -> Option<&MonsterTranslation> {
        self.monsters.get(name)
    }

    /// Get the name of the given monster from the locale database.
    pub fn monster_name(&self, name: &str) -> Option<&str> {
        self.monster(name).map(|monster| monster.name.as_str())
    }

    /// Get the given boss from the locale database.
    pub fn boss(&self, name: &str) -> Option<&BossTranslation> {
        self.bosses.get(name)
    }

    /// Get the name of the given boss from the locale database.
    pub fn boss_name(&self, name: &str) -> Option<&str> {
        self.boss(name).map(|boss| boss.name.as_str())
    }

    /// Get the given skill from the locale database.
    pub fn skill(&self, name: &str) -> Option<&SkillTranslation> {
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
    pub fn follower(&self, name: &str) -> Option<&FollowerTranslation> {
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

    /// Get the spawn from the locale database.
    pub fn spawn(&self, name: &str) -> Option<&str> {
        self.spawns.get(name).map(String::as_str)
    }

    /// Get the family from the locale database.
    pub fn family(&self, name: &str) -> Option<&str> {
        self.families.get(name).map(String::as_str)
    }

    /// Get the rarity from the locale database.
    pub fn rarity(&self, name: &str) -> Option<&str> {
        self.rarities.get(name).map(String::as_str)
    }

    /// Merge the contents of `self` with that of `other`.
    /// For each key in each hash map, the contents of `other` will take precedence over `self` and
    /// overwrite values in case of duplicate keys.
    /// `other.locale` is assumed to match `self.locale`. No check is performed.
    pub fn merge_with(&mut self, mut other: Self) {
        self.items.extend(other.items.drain());
        self.raids.extend(other.raids.drain());
        self.monsters.extend(other.monsters.drain());
        self.bosses.extend(other.bosses.drain());
        self.skills.extend(other.skills.drain());
        self.followers.extend(other.followers.drain());
        self.statuses.extend(other.statuses.drain());
        self.events.extend(other.events.drain());
        self.spawns.extend(other.spawns.drain());
        self.families.extend(other.families.drain());
        self.rarities.extend(other.rarities.drain());
    }
}

impl LocaleDB {
    /// Get the given item from the locale database.
    pub fn item(&self, locale: &str, name: &str) -> Option<&ItemTranslation> {
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
    pub fn raid(&self, locale: &str, name: &str) -> Option<&RaidTranslation> {
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
    pub fn monster(&self, locale: &str, name: &str) -> Option<&MonsterTranslation> {
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
    pub fn boss(&self, locale: &str, name: &str) -> Option<&BossTranslation> {
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
    pub fn skill(&self, locale: &str, name: &str) -> Option<&SkillTranslation> {
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
    pub fn follower(&self, locale: &str, name: &str) -> Option<&FollowerTranslation> {
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

    /// Get the status effect from the locale database.
    pub fn status(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.status(name))
    }

    /// Get the event from the locale database.
    pub fn event(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.event(name))
    }

    /// Get the spawn from the locale database.
    pub fn spawns(&self, locale: &str, name: &str) -> Option<&str> {
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

    /// Save translations to a set of json files in the given directory.
    pub fn save_to(&self, directory: &str) -> Result<(), Error> {
        for (lang, db) in self.locales.iter() {
            serde_json::to_writer_pretty(
                BufWriter::new(File::create(format!("{}/{}.json", directory, lang))?),
                db,
            )?;
        }

        Ok(())
    }

    /// Load translations from a set of json files in the given directory.
    pub fn load_from(directory: &str) -> Result<Self, Error> {
        let mut ret = Self::default();
        // List files from folder ending with `.json`.
        for entry in std::fs::read_dir(directory)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|name| name.ends_with(".json"))
                    .unwrap_or(false)
            })
        {
            let filename = entry.file_name();
            let name = filename.to_str().unwrap();
            if let Some(lang) = name.strip_suffix(".json") {
                match serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/{}",
                    directory, name
                ))?)) {
                    Ok(db) => {
                        ret.locales.insert(lang.to_string(), db);
                    }
                    Err(err) => println!(
                        "Failed to parse json from lang db {}/{}: {}",
                        directory, name, err
                    ),
                }
            } else {
                println!(
                    "Failed to get lang name from lang db {}/{}",
                    directory, name
                );
            }
        }

        Ok(ret)
    }

    /// Merge the contents of `self` with that of `other`.
    /// For each locale, the contents of `other` will take precedence over `self` and overwrite
    /// values in case of duplicate keys.
    /// If `other` contains a locale not contained in `self`, it will be added to `self`.
    pub fn merge_with(&mut self, other: Self) {
        for (lang, db) in other.locales {
            if let Some(self_db) = self.locales.get_mut(&lang) {
                self_db.merge_with(db);
            } else {
                self.locales.insert(lang, db);
            }
        }
    }
}

/// A trait for types that contain translation information and that are able to translate entities
/// of type `T`.
pub trait TranslationFor<T> {
    /// Translate the strings of `entity`.
    fn apply_to(&self, entity: &mut T);
}

impl TranslationFor<AdminItem> for ItemTranslation {
    fn apply_to(&self, item: &mut AdminItem) {
        item.name = self.name.clone();
        item.description = self.description.clone();
    }
}

impl TranslationFor<AdminMonster> for RaidTranslation {
    fn apply_to(&self, raid: &mut AdminMonster) {
        raid.name = self.name.clone();
    }
}

impl TranslationFor<AdminMonster> for BossTranslation {
    fn apply_to(&self, boss: &mut AdminMonster) {
        boss.name = self.name.clone();
    }
}

impl TranslationFor<AdminMonster> for MonsterTranslation {
    fn apply_to(&self, monster: &mut AdminMonster) {
        monster.name = self.name.clone();
    }
}

impl TranslationFor<AdminMonster> for GenericMonsterTranslation {
    fn apply_to(&self, monster: &mut AdminMonster) {
        match self {
            GenericMonsterTranslation::Monster(x) => x.apply_to(monster),
            GenericMonsterTranslation::Raid(x) => x.apply_to(monster),
            GenericMonsterTranslation::Boss(x) => x.apply_to(monster),
        }
    }
}

impl TranslationFor<AdminSkill> for SkillTranslation {
    fn apply_to(&self, skill: &mut AdminSkill) {
        skill.name = self.name.clone();
        skill.description = self.description.clone();
    }
}

impl TranslationFor<AdminPet> for FollowerTranslation {
    fn apply_to(&self, follower: &mut AdminPet) {
        follower.name = self.name.clone();
        follower.description = self.description.clone();
    }
}
