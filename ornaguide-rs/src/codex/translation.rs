#![allow(clippy::module_name_repetitions)]
use serde::{Deserialize, Serialize};

use crate::{
    codex::{CodexBoss, CodexFollower, CodexItem, CodexMonster, CodexRaid, CodexSkill},
    data::OrnaData,
    error::{Error, Kind},
    items::admin::AdminItem,
    misc::codex_effect_name_to_guide_name,
    monsters::admin::AdminMonster,
    pets::admin::AdminPet,
    skills::admin::AdminSkill,
};

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

/// Holds strings that can be translated for an item.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ItemTranslation {
    /// The name of the item.
    pub name: String,
    /// The description of the item.
    pub description: String,
}

/// Holds strings that can be translated for a raid.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RaidTranslation {
    /// The name of the raid.
    pub name: String,
    /// The description of the raid.
    pub description: String,
}

/// Holds strings that can be translated for a boss.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BossTranslation {
    /// The name of the boss.
    pub name: String,
}

/// Holds strings that can be translated for a monster.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MonsterTranslation {
    /// The name of the monster.
    pub name: String,
}

/// Holds strings that can be translated for any monster.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum GenericMonsterTranslation {
    /// `self` refers to a monster.
    Monster(MonsterTranslation),
    /// `self` refers to a raid.
    Raid(RaidTranslation),
    /// `self` refers to a boss.
    Boss(BossTranslation),
}

/// Holds strings that can be translated for a skill.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SkillTranslation {
    /// The name of the skill.
    pub name: String,
    /// The description of the skill.
    pub description: String,
}

/// Holds strings that can be translated for a follower.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct FollowerTranslation {
    /// The name of the follower.
    pub name: String,
    /// The description of the follower.
    pub description: String,
}

/// A set of strings for a particular language.
#[derive(Default, Serialize, Deserialize, Debug, Eq, PartialEq)]
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
#[derive(Default, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LocaleDB {
    /// Map of locales. The key is the locale name.
    pub locales: HashMap<String, LocaleStrings>,
}

impl LocaleStrings {
    /// Add some codex items to the locale database.
    /// If an item is already present in `self`, it is updated with the strings from `items`.
    pub fn add_items(&mut self, items: Vec<CodexItem>) {
        for item in items {
            self.items.insert(
                item.slug.to_string(),
                ItemTranslation {
                    name: item.name,
                    description: item.description,
                },
            );
        }
    }

    /// Add some codex raids to the locale database.
    /// If a raid is already present in `self`, it is updated with the strings from `raids`.
    /// Events associated to the raid are added to the events database.
    ///
    /// # Errors
    /// Errors if a raid cannot be found in the regular database.
    pub fn add_raids_and_events(
        &mut self,
        raids: Vec<CodexRaid>,
        data: &OrnaData,
    ) -> Result<(), Error> {
        for raid in raids {
            let raid_data = data
                .codex
                .raids
                .find_by_uri(&format!("/codex/raids/{}/", raid.slug))
                .ok_or_else(|| {
                    Kind::Misc(format!(
                        "Failed to find raid {} (found in locale {})",
                        raid.slug, self.locale
                    ))
                })?;

            // Update strings not directly related to the raid.
            // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
            for (localed, en) in raid.events.iter().zip(raid_data.events.iter()) {
                self.events.insert(en.clone(), localed.clone());
            }

            self.raids.insert(
                raid.slug.to_string(),
                RaidTranslation {
                    name: raid.name,
                    description: raid.description,
                },
            );
        }
        Ok(())
    }

    /// Add some codex monsters to the locale database.
    /// If a monsters is already present in `self`, it is updated with the strings from `monsters`.
    /// Events, families and rarities associated to the monster are added to the database.
    ///
    /// # Errors
    /// Errors if a monster cannot be found in the regular database.
    pub fn add_monsters_events_families_and_rarities(
        &mut self,
        monsters: Vec<CodexMonster>,
        data: &OrnaData,
    ) -> Result<(), Error> {
        for monster in monsters {
            let monster_data = data
                .codex
                .monsters
                .find_by_uri(&format!("/codex/monsters/{}/", monster.slug))
                .ok_or_else(|| {
                    Kind::Misc(format!(
                        "Failed to find monster {} (found in locale {})",
                        monster.slug, self.locale
                    ))
                })?;

            // Update strings not directly related to the monster.
            // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
            for (localed, en) in monster.events.iter().zip(monster_data.events.iter()) {
                self.events.insert(en.clone(), localed.clone());
            }
            self.families
                .insert(monster_data.family.clone(), monster.family.clone());
            self.rarities
                .insert(monster_data.rarity.clone(), monster.rarity.clone());

            self.monsters.insert(
                monster.slug.to_string(),
                MonsterTranslation { name: monster.name },
            );
        }
        Ok(())
    }

    /// Add some codex bosses to the locale database.
    /// If a boss is already present in `self`, it is updated with the strings from `bosses`.
    /// Events, families and rarities associated to the boss are added to the database.
    ///
    /// # Errors
    /// Errors if a boss cannot be found in the regular database.
    pub fn add_bosses_events_families_and_rarities(
        &mut self,
        bosses: Vec<CodexBoss>,
        data: &OrnaData,
    ) -> Result<(), Error> {
        for boss in bosses {
            let boss_data = data
                .codex
                .bosses
                .find_by_uri(&format!("/codex/bosses/{}/", boss.slug))
                .ok_or_else(|| {
                    Kind::Misc(format!(
                        "Failed to find boss {} (found in locale {})",
                        boss.slug, self.locale
                    ))
                })?;

            // Update strings not directly related to the boss.
            // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
            for (localed, en) in boss.events.iter().zip(boss_data.events.iter()) {
                self.events.insert(en.clone(), localed.clone());
            }
            self.families
                .insert(boss_data.family.clone(), boss.family.clone());
            self.rarities
                .insert(boss_data.rarity.clone(), boss.rarity.clone());

            self.bosses
                .insert(boss.slug.to_string(), BossTranslation { name: boss.name });
        }
        Ok(())
    }

    /// Add some codex skills to the locale database.
    /// If a skill is already present in `self`, it is updated with the strings from `skills`.
    /// Statuses associated to the skill are added to the database.
    ///
    /// # Errors
    /// Errors if a skill cannot be found in the regular database.
    pub fn add_skills_and_statuses(
        &mut self,
        skills: Vec<CodexSkill>,
        data: &OrnaData,
    ) -> Result<(), Error> {
        for skill in skills {
            let skill_data = data
                .codex
                .skills
                .find_by_uri(&format!("/codex/spells/{}/", skill.slug))
                .ok_or_else(|| {
                    Kind::Misc(format!(
                        "Failed to find skill {} (found in locale {})",
                        skill.slug, self.locale
                    ))
                })?;

            // Update strings not directly related to the skill.
            // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
            for (localed, en) in skill
                .causes
                .iter()
                .zip(skill_data.causes.iter().chain(skill_data.gives.iter()))
            {
                self.statuses.insert(
                    codex_effect_name_to_guide_name(&en.effect).to_string(),
                    localed.effect.clone(),
                );
            }

            self.skills.insert(
                skill.slug.to_string(),
                SkillTranslation {
                    name: skill.name,
                    description: skill.description,
                },
            );
        }
        Ok(())
    }

    /// Add some codex followers to the locale database.
    /// If a follower is already present in `self`, it is updated with the strings from `followers`.
    /// Events associated to the follower are added to the database.
    ///
    /// # Errors
    /// Errors if a follower cannot be found in the regular database.
    pub fn add_followers_and_events(
        &mut self,
        followers: Vec<CodexFollower>,
        data: &OrnaData,
    ) -> Result<(), Error> {
        for follower in followers {
            let follower_data = data
                .codex
                .followers
                .find_by_uri(&format!("/codex/followers/{}/", follower.slug))
                .ok_or_else(|| {
                    Kind::Misc(format!(
                        "Failed to find follower {} (found in locale {})",
                        follower.slug, self.locale
                    ))
                })?;

            // Update strings not directly related to the follower.
            // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
            for (localed, en) in follower.events.iter().zip(follower_data.events.iter()) {
                self.events.insert(en.clone(), localed.clone());
            }

            self.followers.insert(
                follower.slug.to_string(),
                FollowerTranslation {
                    name: follower.name,
                    description: follower.description,
                },
            );
        }
        Ok(())
    }

    /// Get the given item from the locale database.
    #[must_use]
    pub fn item(&self, name: &str) -> Option<&ItemTranslation> {
        self.items.get(name)
    }

    /// Get the name of the given item from the locale database.
    #[must_use]
    pub fn item_name(&self, name: &str) -> Option<&str> {
        self.item(name).map(|item| item.name.as_str())
    }

    /// Get the description of the given item from the locale database.
    #[must_use]
    pub fn item_description(&self, item_name: &str) -> Option<&str> {
        self.item(item_name).map(|item| item.description.as_str())
    }

    /// Get the given raid from the locale database.
    #[must_use]
    pub fn raid(&self, name: &str) -> Option<&RaidTranslation> {
        self.raids.get(name)
    }

    /// Get the name of the given raid from the locale database.
    #[must_use]
    pub fn raid_name(&self, name: &str) -> Option<&str> {
        self.raid(name).map(|raid| raid.name.as_str())
    }

    /// Get the description of the given raid from the locale database.
    #[must_use]
    pub fn raid_description(&self, raid_name: &str) -> Option<&str> {
        self.raid(raid_name).map(|raid| raid.description.as_str())
    }

    /// Get the given monster from the locale database.
    #[must_use]
    pub fn monster(&self, name: &str) -> Option<&MonsterTranslation> {
        self.monsters.get(name)
    }

    /// Get the name of the given monster from the locale database.
    #[must_use]
    pub fn monster_name(&self, name: &str) -> Option<&str> {
        self.monster(name).map(|monster| monster.name.as_str())
    }

    /// Get the given boss from the locale database.
    #[must_use]
    pub fn boss(&self, name: &str) -> Option<&BossTranslation> {
        self.bosses.get(name)
    }

    /// Get the name of the given boss from the locale database.
    #[must_use]
    pub fn boss_name(&self, name: &str) -> Option<&str> {
        self.boss(name).map(|boss| boss.name.as_str())
    }

    /// Get the given skill from the locale database.
    #[must_use]
    pub fn skill(&self, name: &str) -> Option<&SkillTranslation> {
        self.skills.get(name)
    }

    /// Get the name of the given skill from the locale database.
    #[must_use]
    pub fn skill_name(&self, name: &str) -> Option<&str> {
        self.skill(name).map(|skill| skill.name.as_str())
    }

    /// Get the description of the given skill from the locale database.
    #[must_use]
    pub fn skill_description(&self, skill_name: &str) -> Option<&str> {
        self.skill(skill_name)
            .map(|skill| skill.description.as_str())
    }

    /// Get the given follower from the locale database.
    #[must_use]
    pub fn follower(&self, name: &str) -> Option<&FollowerTranslation> {
        self.followers.get(name)
    }

    /// Get the name of the given follower from the locale database.
    #[must_use]
    pub fn follower_name(&self, name: &str) -> Option<&str> {
        self.follower(name).map(|follower| follower.name.as_str())
    }

    /// Get the description of the given follower from the locale database.
    #[must_use]
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

    /// Save translations to a json file.
    ///
    /// # Errors
    /// Errors on I/O error or parsing error.
    pub fn load_from(file: &str) -> Result<Self, Error> {
        serde_json::from_reader(BufReader::new(File::open(file)?)).map_err(|err| {
            Kind::Misc(format!("Failed to parse json from lang db {file}: {err}")).into()
        })
    }

    /// Save translations to a json file.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to(&self, file: &str) -> Result<(), Error> {
        Ok(serde_json::to_writer_pretty(
            BufWriter::new(File::create(file)?),
            &self,
        )?)
    }

    /// Save translations as json to a writer.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to_writer<W: Write>(&self, out: W) -> Result<(), Error> {
        Ok(serde_json::to_writer_pretty(out, &self)?)
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
    #[must_use]
    pub fn item(&self, locale: &str, name: &str) -> Option<&ItemTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item(name))
    }

    /// Get the name of the given item from the locale database.
    #[must_use]
    pub fn item_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item_name(name))
    }

    /// Get the description of the given item from the locale database.
    #[must_use]
    pub fn item_description(&self, locale: &str, item_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.item_description(item_name))
    }

    /// Get the given raid from the locale database.
    #[must_use]
    pub fn raid(&self, locale: &str, name: &str) -> Option<&RaidTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid(name))
    }

    /// Get the name of the given raid from the locale database.
    #[must_use]
    pub fn raid_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid_name(name))
    }

    /// Get the description of the given raid from the locale database.
    #[must_use]
    pub fn raid_description(&self, locale: &str, raid_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.raid_description(raid_name))
    }

    /// Get the given monster from the locale database.
    #[must_use]
    pub fn monster(&self, locale: &str, name: &str) -> Option<&MonsterTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.monster(name))
    }

    /// Get the name of the given monster from the locale database.
    #[must_use]
    pub fn monster_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.monster_name(name))
    }

    /// Get the given boss from the locale database.
    #[must_use]
    pub fn boss(&self, locale: &str, name: &str) -> Option<&BossTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.boss(name))
    }

    /// Get the name of the given boss from the locale database.
    #[must_use]
    pub fn boss_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.boss_name(name))
    }

    /// Get the given skill from the locale database.
    #[must_use]
    pub fn skill(&self, locale: &str, name: &str) -> Option<&SkillTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill(name))
    }

    /// Get the name of the given skill from the locale database.
    #[must_use]
    pub fn skill_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill_name(name))
    }

    /// Get the description of the given skill from the locale database.
    #[must_use]
    pub fn skill_description(&self, locale: &str, skill_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.skill_description(skill_name))
    }

    /// Get the given follower from the locale database.
    #[must_use]
    pub fn follower(&self, locale: &str, name: &str) -> Option<&FollowerTranslation> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower(name))
    }

    /// Get the name of the given follower from the locale database.
    #[must_use]
    pub fn follower_name(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower_name(name))
    }

    /// Get the description of the given follower from the locale database.
    #[must_use]
    pub fn follower_description(&self, locale: &str, follower_name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.follower_description(follower_name))
    }

    /// Get the status effect from the locale database.
    #[must_use]
    pub fn status(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.status(name))
    }

    /// Get the event from the locale database.
    #[must_use]
    pub fn event(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.event(name))
    }

    /// Get the spawn from the locale database.
    #[must_use]
    pub fn spawns(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.event(name))
    }

    /// Get the family from the locale database.
    #[must_use]
    pub fn family(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.family(name))
    }

    /// Get the rarity from the locale database.
    #[must_use]
    pub fn rarity(&self, locale: &str, name: &str) -> Option<&str> {
        self.locales
            .get(locale)
            .and_then(|locale| locale.rarity(name))
    }

    /// Write the contents of the locale database with the given helper functions.
    /// `Writer` is called for each file there is to write.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to_generic<Writer>(&self, directory: &str, mut writer: Writer) -> Result<(), Error>
    where
        Writer: FnMut(&str, &dyn Fn(&mut dyn Write) -> Result<(), Error>) -> Result<(), Error>,
    {
        for (lang, db) in &self.locales {
            writer(&format!("{directory}/{lang}.json"), &|out| {
                db.save_to_writer(out)
            })?;
        }

        Ok(())
    }

    /// Save translations to a set of json files in the given directory.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to(&self, directory: &str) -> Result<(), Error> {
        for (lang, db) in &self.locales {
            db.save_to(&format!("{directory}/{lang}.json"))?;
        }

        Ok(())
    }

    /// Load translations from a set of json files in the given directory.
    ///
    /// # Errors
    /// Errors if an I/O error occurs
    pub fn load_from(directory: &str) -> Result<Self, Error> {
        let mut ret = Self::default();
        // List files from folder ending with `.json`.
        for entry in std::fs::read_dir(directory)?
            .filter_map(std::result::Result::ok)
            .filter(|entry| {
                entry.file_name().to_str().map_or(false, |name| {
                    std::path::Path::new(name)
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("json"))
                })
            })
        {
            let filename = entry.file_name();
            let Some(name) = filename.to_str() else { continue; };
            if let Some(lang) = name.strip_suffix(".json") {
                match LocaleStrings::load_from(&format!("{directory}/{name}")) {
                    Ok(db) => {
                        ret.locales.insert(lang.to_string(), db);
                    }
                    Err(err) => {
                        println!("Failed to parse json from lang db {directory}/{name}: {err}");
                    }
                }
            } else {
                println!("Failed to get lang name from lang db {directory}/{name}");
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
