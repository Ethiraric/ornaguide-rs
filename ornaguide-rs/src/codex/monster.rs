use serde::{Deserialize, Serialize};

pub use crate::guide::html_utils::Tag;
use crate::{data::GuideData, error::Error, monsters::admin::AdminMonster};

/// An ability for a monster.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The uri to the ability.
    pub uri: String,
    /// The icon of the ability.
    pub icon: String,
}

/// A drop for a monster
#[derive(Debug, Serialize, Deserialize)]
pub struct Drop {
    /// The name of the item.
    pub name: String,
    /// The uri to the item.
    pub uri: String,
    /// The icon of the item.
    pub icon: String,
}

/// A monster on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    /// The slug of the monster (`https://playorna.com/codex/monsters/{slug}`).
    pub slug: String,
    /// The name of the monster.
    pub name: String,
    /// The icon of the monster.
    pub icon: String,
    /// The events in which the monster appears.
    pub events: Vec<String>,
    /// The family to which the monster belongs.
    pub family: String,
    /// The rarity of the monster.
    pub rarity: String,
    /// The tier of the monster.
    pub tier: u8,
    /// The abilities of the monster.
    pub abilities: Vec<Ability>,
    /// The items the monster drops.
    pub drops: Vec<Drop>,
}

/// A boss on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct Boss {
    /// The slug of the boss (`https://playorna.com/codex/bosses/{slug}`).
    pub slug: String,
    /// The name of the boss.
    pub name: String,
    /// The icon of the boss.
    pub icon: String,
    /// The event in which the boss appears.
    pub events: Vec<String>,
    /// The family to which the boss belongs.
    pub family: String,
    /// The rarity of the boss.
    pub rarity: String,
    /// The tier of the boss.
    pub tier: u8,
    /// The abilities of the boss.
    pub abilities: Vec<Ability>,
    /// The items the boss drops.
    pub drops: Vec<Drop>,
}

/// A raid on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct Raid {
    /// The slug of the raid (`https://playorna.com/codex/raids/{slug}`).
    pub slug: String,
    /// The name of the raid.
    pub name: String,
    /// The description of the raid.
    pub description: String,
    /// The icon of the raid.
    pub icon: String,
    /// The event in which the raid appears.
    pub events: Vec<String>,
    /// The tier of the raid.
    pub tier: u8,
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
    /// The abilities of the raid.
    pub abilities: Vec<Ability>,
    /// The items the raid drops.
    pub drops: Vec<Drop>,
}

/// Collection of monsters from the codex.
#[derive(Serialize, Deserialize)]
pub struct Monsters {
    /// Monsters from the codex.
    pub monsters: Vec<Monster>,
}

/// Collection of bosses from the codex.
#[derive(Serialize, Deserialize)]
pub struct Bosses {
    /// Bosses from the codex.
    pub bosses: Vec<Boss>,
}

/// Collection of raids from the codex.
#[derive(Serialize, Deserialize)]
pub struct Raids {
    /// Raids from the codex.
    pub raids: Vec<Raid>,
}

impl Monster {
    /// Try to convert `self` to an `AdminMonster`.
    ///
    ///  - An unknown family will be ignored, rather than returning an error.
    ///  - Unknown events are ignored, rather than returning an error.
    ///  - Unknown drops are ignored, rather than returning an error.
    ///  - Unknown skills are ignored, rather than returning an error.
    pub fn try_to_admin_monster(&self, guide_data: &GuideData) -> Result<AdminMonster, Error> {
        Ok(AdminMonster {
            codex_uri: format!("/codex/monsters/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            family: guide_data
                .static_
                .monster_families
                .iter()
                .find(|family| family.name == self.family)
                .map(|family| family.id),
            image_name: self.icon.clone(),
            boss: false,
            spawns: self
                .events
                .iter()
                .filter_map(|event_name| {
                    guide_data
                        .static_
                        .iter_events()
                        .find(|event| event.event_name() == *event_name)
                        .map(|event| event.id)
                })
                .collect(),
            drops: self
                .drops
                .iter()
                .filter_map(|drop| guide_data.items.find_by_uri(&drop.uri).map(|item| item.id))
                .collect(),
            skills: self
                .abilities
                .iter()
                .filter_map(|skill| {
                    guide_data
                        .skills
                        .find_by_uri(&skill.uri)
                        .map(|skill| skill.id)
                })
                .collect(),
            ..AdminMonster::default()
        })
    }
}

impl Boss {
    /// Try to convert `self` to an `AdminMonster`.
    ///
    ///  - An unknown family will be ignored, rather than returning an error.
    ///  - Unknown events are ignored, rather than returning an error.
    ///  - Unknown drops are ignored, rather than returning an error.
    ///  - Unknown skills are ignored, rather than returning an error.
    pub fn try_to_admin_monster(&self, guide_data: &GuideData) -> Result<AdminMonster, Error> {
        Ok(AdminMonster {
            codex_uri: format!("/codex/bosses/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            family: guide_data
                .static_
                .monster_families
                .iter()
                .find(|family| family.name == self.family)
                .map(|family| family.id),
            image_name: self.icon.clone(),
            boss: true,
            spawns: self
                .events
                .iter()
                .filter_map(|event_name| {
                    guide_data
                        .static_
                        .iter_events()
                        .find(|event| event.event_name() == *event_name)
                        .map(|event| event.id)
                })
                .collect(),
            drops: self
                .drops
                .iter()
                .filter_map(|drop| guide_data.items.find_by_uri(&drop.uri).map(|item| item.id))
                .collect(),
            skills: self
                .abilities
                .iter()
                .filter_map(|skill| {
                    guide_data
                        .skills
                        .find_by_uri(&skill.uri)
                        .map(|skill| skill.id)
                })
                .collect(),
            ..AdminMonster::default()
        })
    }
}

impl Raid {
    /// Try to convert `self` to an `AdminMonster`.
    ///
    ///  - Unknown events are ignored, rather than returning an error.
    ///  - Unknown spawns are ignored, rather than returning an error.
    ///  - Unknown drops are ignored, rather than returning an error.
    ///  - Unknown skills are ignored, rather than returning an error.
    pub fn try_to_admin_monster(&self, guide_data: &GuideData) -> Result<AdminMonster, Error> {
        Ok(AdminMonster {
            codex_uri: format!("/codex/raids/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            image_name: self.icon.clone(),
            boss: true,
            spawns: self
                // List events to which the raid belongs to.
                .events
                .iter()
                .filter_map(|event_name| {
                    guide_data
                        .static_
                        .iter_events()
                        .find(|event| event.event_name() == *event_name)
                        .map(|event| event.id)
                })
                // Add raid tags.
                .chain(self.tags.iter().filter_map(|tag| {
                    match tag {
                        Tag::WorldRaid => guide_data
                            .static_
                            .spawns
                            .iter()
                            .find(|spawn| spawn.name == "World Raid")
                            .map(|spawn| spawn.id),
                        Tag::KingdomRaid => guide_data
                            .static_
                            .spawns
                            .iter()
                            .find(|spawn| spawn.name == "Kingdom Raid")
                            .map(|spawn| spawn.id),
                        // TODO(ethiraric, 28/07/2022): Include Other Realm Raid as a spawn?
                        Tag::OtherRealmsRaid => None,
                        _ => None,
                    }
                }))
                .collect(),
            drops: self
                .drops
                .iter()
                .filter_map(|drop| guide_data.items.find_by_uri(&drop.uri).map(|item| item.id))
                .collect(),
            skills: self
                .abilities
                .iter()
                .filter_map(|skill| {
                    guide_data
                        .skills
                        .find_by_uri(&skill.uri)
                        .map(|skill| skill.id)
                })
                .collect(),
            ..AdminMonster::default()
        })
    }
}

impl<'a> Monsters {
    /// Find the codex monster associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Monster> {
        static URI_START: &str = "/codex/monsters/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.monsters.iter().find(|monster| monster.slug == slug)
    }

    /// Find the codex monster associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Monster, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex monster with uri '{}'", needle)))
    }
}

impl<'a> Bosses {
    /// Find the codex boss associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Boss> {
        static URI_START: &str = "/codex/bosses/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.bosses.iter().find(|boss| boss.slug == slug)
    }

    /// Find the codex boss associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Boss, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex boses with uri '{}'", needle)))
    }
}

impl<'a> Raids {
    /// Find the codex raid associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Raid> {
        static URI_START: &str = "/codex/raids/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.raids.iter().find(|raid| raid.slug == slug)
    }

    /// Find the codex raid associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Raid, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex raid with uri '{}'", needle)))
    }
}
