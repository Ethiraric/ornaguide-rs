use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    guide::{html_form_parser::ParsedForm, Spawn},
    misc::sanitize_guide_name,
};

/// An item fetched from the admin panel.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AdminMonster {
    /// The CSRF token that was given on the page where the monster was fetched.
    #[serde(skip)]
    pub(crate) csrfmiddlewaretoken: String,
    /// Id of the monster on the guide.
    pub id: u32,
    /// The URI of the monster on the codex.
    /// URI matches `/codex/{entity}/{slug}/` with the trailing slash.
    /// `entity` is either `monsters`, `bosses` or `raids`.
    pub codex_uri: String,
    /// The name of the monster on the guide.
    pub name: String,
    /// The tier of the monster.
    pub tier: u8,
    /// The family to which the monster belongs.
    pub family: Option<u32>,
    /// Path to the image of the monster.
    pub image_name: String,
    /// Whether the monster is a boss (WRB & KRB included).
    pub boss: bool,
    /// The HP of the monster, if specifyable.
    /// This should be set for raids and bosses.
    pub hp: u32,
    /// The level at which the monster is encountered.
    /// This should be set for raids and bosses.
    pub level: u32,
    /// Handwritten notes from the guide team on the monster.
    pub notes: String,
    /// Ids of where the monster spawns.
    pub spawns: Vec<u32>,
    /// Ids of elements to which the monster is weak.
    pub weak_to: Vec<u32>,
    /// Ids of elements to which the monster is resistant.
    pub resistant_to: Vec<u32>,
    /// Ids of elements to which the monster is immune.
    pub immune_to: Vec<u32>,
    /// Ids of statuses to which the monster is immune.
    pub immune_to_status: Vec<u32>,
    /// Ids of statuses to which the monster is vulnerable.
    /// This field is likely to disappear.
    pub vulnerable_to_status: Vec<u32>,
    /// Ids of items the monster drops.
    pub drops: Vec<u32>,
    /// Ids of skills the monster uses.
    pub skills: Vec<u32>,
}

impl TryFrom<ParsedForm> for AdminMonster {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut item = AdminMonster {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "codex" => item.codex_uri = value,
                "name" => item.name = value,
                "tier" => item.tier = value.parse()?,
                "family" => {
                    item.family = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "image_name" => item.image_name = value,
                "boss" => item.boss = value == "on",
                "level" => item.level = value.parse()?,
                "hp" => item.hp = value.parse()?,
                "notes" => item.notes = value,
                "spawns" => item.spawns.push(value.parse()?),
                "weak_to" => item.weak_to.push(value.parse()?),
                "resistant_to" => item.resistant_to.push(value.parse()?),
                "immune_to" => item.immune_to.push(value.parse()?),
                "immune_to_status" => item.immune_to_status.push(value.parse()?),
                "vulnerable_to_status" => item.vulnerable_to_status.push(value.parse()?),
                "drops" => item.drops.push(value.parse()?),
                "skills" => item.skills.push(value.parse()?),
                key => {
                    return Err(Error::ExtraField(key.to_string(), value));
                }
            }
        }

        Ok(item)
    }
}

impl From<AdminMonster> for ParsedForm {
    fn from(item: AdminMonster) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: item.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("name", item.name);
        push("codex", item.codex_uri);
        push("tier", item.tier.to_string());
        push(
            "family",
            item.family
                .map(|family| family.to_string())
                .unwrap_or_else(String::new),
        );
        push("image_name", item.image_name);
        if item.boss {
            push("boss", "on".to_string());
        }
        push("level", item.level.to_string());
        push("hp", item.hp.to_string());
        push("notes", item.notes);

        for x in item.spawns.iter() {
            push("spawns", x.to_string());
        }
        for x in item.weak_to.iter() {
            push("weak_to", x.to_string());
        }
        for x in item.resistant_to.iter() {
            push("resistant_to", x.to_string());
        }
        for x in item.immune_to.iter() {
            push("immune_to", x.to_string());
        }
        for x in item.immune_to_status.iter() {
            push("immune_to_status", x.to_string());
        }
        for x in item.vulnerable_to_status.iter() {
            push("vulnerable_to_status", x.to_string());
        }
        for x in item.drops.iter() {
            push("drops", x.to_string());
        }
        for x in item.skills.iter() {
            push("skills", x.to_string());
        }

        form
    }
}

impl AdminMonster {
    /// Returns true if the monster is a regular one (not a boss, nor a raid).
    pub fn is_regular_monster(&self) -> bool {
        !self.boss
    }

    /// Returns true if the monster is a boss (not a regular monster, nor a raid).
    pub fn is_boss(&self, guide_spawns: &[Spawn]) -> bool {
        self.boss
            && !self
                .spawns
                .iter()
                .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| spawn.id == *spawn_id))
                .any(|spawn| {
                    spawn.name == "Kingdom Raid"
                        || spawn.name == "World Raid"
                        || spawn.name == "World Raid year-round"
                })
    }

    /// Returns true if the monster is a raid (not a regular monster, nor a boss).
    pub fn is_raid(&self, guide_spawns: &[Spawn]) -> bool {
        self.boss
            && self
                .spawns
                .iter()
                .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| spawn.id == *spawn_id))
                .any(|spawn| {
                    spawn.name == "Kingdom Raid"
                        || spawn.name == "World Raid"
                        || spawn.name == "World Raid year-round"
                })
    }

    /// Returns true if the monster is a world raid.
    pub fn is_world_raid(&self, guide_spawns: &[Spawn]) -> bool {
        self.boss
            && self
                .spawns
                .iter()
                .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| spawn.id == *spawn_id))
                .any(|spawn| spawn.name == "World Raid" || spawn.name == "World Raid year-round")
    }

    /// Returns true if the monster is a kingdom raid.
    pub fn is_kingdom_raid(&self, guide_spawns: &[Spawn]) -> bool {
        self.boss
            && self
                .spawns
                .iter()
                .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| spawn.id == *spawn_id))
                .any(|spawn| spawn.name == "Kingdom Raid")
    }

    /// Try to guess what the codex name for the monster is.
    pub fn codex_name(&self) -> String {
        let monster_name = if self.is_regular_monster() {
            self.name
                .strip_prefix("Arisen ")
                .map(|stripped| format!("{} (Arisen)", stripped))
                .unwrap_or_else(|| self.name.clone())
        } else if self.name == "Arisen Kin of Kerberos" {
            "Kin of Kerberos (Arisen)".to_string()
        } else {
            self.name.clone()
        };
        sanitize_guide_name(&monster_name).to_string()
    }

    /// List the events to which the monster belongs.
    /// The events returned won't have the `Event:` or `Past Event` prefix.
    pub fn get_events<'a>(&self, guide_spawns: &'a [Spawn]) -> Vec<&'a str> {
        self.spawns
            .iter()
            .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| *spawn_id == spawn.id))
            .map(|spawn| &spawn.name)
            .filter_map(|spawn| {
                if spawn.starts_with("Event:") {
                    Some(&spawn[7..])
                } else if spawn.starts_with("Past Event:") {
                    Some(&spawn[12..])
                } else {
                    None
                }
            })
            .collect()
    }

    /// List the events IDs to which the monster belongs.
    pub fn get_event_ids(&self, guide_spawns: &[Spawn]) -> Vec<u32> {
        self.spawns
            .iter()
            .filter_map(|spawn_id| guide_spawns.iter().find(|spawn| *spawn_id == spawn.id))
            .filter_map(|spawn| {
                if spawn.name.starts_with("Event:") || spawn.name.starts_with("Past Event:") {
                    Some(spawn.id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// List the raid spawns associated to the monster.
    /// The spawns are either "Kingdom Raid" or "World Raid" (may be inclusive).
    pub fn get_raid_spawns<'a>(&self, guide_spawns: &'a [Spawn]) -> Vec<&'a str> {
        self.spawns
            .iter()
            .map(|spawn_id| {
                guide_spawns
                    .iter()
                    .find(|spawn| spawn.id == *spawn_id)
                    .unwrap()
            })
            .filter(|spawn| spawn.name == "Kingdom Raid" || spawn.name == "World Raid")
            .map(|spawn| spawn.name.as_str())
            .sorted()
            .collect::<Vec<_>>()
    }
}

/// Collection of monsters from the guide's admin view.
#[derive(Serialize, Deserialize, Clone)]
pub struct AdminMonsters {
    /// Monsters from the guide's admin view.
    pub monsters: Vec<AdminMonster>,
}

impl<'a> AdminMonsters {
    /// Find the monster with the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminMonster> {
        self.monsters.iter().find(|monster| monster.id == needle)
    }

    /// Find the monster with the given id
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminMonster, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No match for admin monster with id {}", needle)))
    }

    /// Find the monster with the given codex uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminMonster> {
        self.monsters
            .iter()
            .find(|monster| monster.codex_uri == needle)
    }

    /// Find the monster with the given codex uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminMonster, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin monster with codex_uri '{}'",
                needle
            ))
        })
    }
}
