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
    #[serde(skip)]
    pub(crate) csrfmiddlewaretoken: String,
    pub id: u32,
    pub codex_uri: String,
    pub name: String,
    pub tier: u8,
    pub family: Option<u32>,
    pub image_name: String,
    pub boss: bool,
    pub hp: u32,
    pub level: u32,
    pub notes: String,
    pub spawns: Vec<u32>,
    pub weak_to: Vec<u32>,
    pub resistant_to: Vec<u32>,
    pub immune_to: Vec<u32>,
    pub immune_to_status: Vec<u32>,
    pub vulnerable_to_status: Vec<u32>,
    pub drops: Vec<u32>,
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

fn slugify_name(name: &str) -> String {
    sanitize_guide_name(name).to_lowercase().replace(' ', "-")
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

    /// Try to guess what the codex URI for the monster is.
    /// Returns something like `/codex/monsters/ghost/`.
    pub fn codex_uri(&self, guide_spawns: &[Spawn]) -> String {
        let slug = slugify_name(&self.name);
        if self.is_regular_monster() {
            format!("/codex/monsters/{}/", slug)
        } else if self.is_boss(guide_spawns) {
            format!("/codex/bosses/{}/", slug)
        } else {
            format!("/codex/raids/{}/", slug)
        }
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

    // List the events to which the monster belongs.
    // The events returned won't have the `Event:` or `Past Event` prefix.
    pub fn get_events(&self, guide_spawns: &[Spawn]) -> Vec<String> {
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
            .map(|spawn| spawn.to_string())
            .collect()
    }

    // List the raid spawns associated to the monster.
    // The spawns are either "Kingdom Raid" or "World Raid" (may be inclusive).
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
