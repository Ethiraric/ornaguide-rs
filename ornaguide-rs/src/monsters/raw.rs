use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// A drop from the monster.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MonsterDrop {
    pub id: u32,
    pub name: String,
}

/// A skill from the monster.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MonsterSkill {
    pub id: u32,
    pub name: String,
}

/// A quest featuring the monster.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MonsterQuest {
    pub id: u32,
    pub name: String,
}

/// A buff towards which that monster counts.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MonsterBuff {
    pub id: u32,
    pub name: String,
}

/// An object representation of a monster in Orna from the API json. This encompasses regular
/// monsters, bosses and WRBs.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawMonster {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub boss: bool,
    pub image: String,
    pub drops: Option<Vec<MonsterDrop>>,
    pub skills: Option<Vec<MonsterSkill>>,
    pub quests: Option<Vec<MonsterQuest>>,
    pub level: Option<u32>,
    pub spawns: Option<Vec<String>>,
    pub resistant_to: Option<Vec<String>>,
    pub weak_to: Option<Vec<String>>,
    pub immune_to: Option<Vec<String>>,
    pub buffs: Option<Vec<MonsterBuff>>,
    pub immune_to_status: Option<Vec<String>>,
    pub vulnerable_to_status: Option<Vec<String>>,
}

impl RawMonster {
    /// Return true if `self` is a raid monster.
    pub(crate) fn is_raid(&self) -> bool {
        self.spawns.is_some()
            && self
                .spawns
                .as_ref()
                .unwrap()
                .iter()
                .any(|spawn| spawn == "Kingdom Raid" || spawn == "World Raid")
    }
}

/// A set of monsters json objects. This is used to attach methods for parsing.
pub struct RawMonsters {
    pub items: Vec<RawMonster>,
}

impl RawMonsters {
    /// Parse a set of monsters from a json string.
    pub fn parse_from_json(s: &str) -> Result<Self, Error> {
        Ok(Self {
            items: serde_json::from_str(s)?,
        })
    }

    /// Parse a set of monsters from a filename.
    pub fn parse_from_file(filename: &str) -> Result<Self, Error> {
        let file = BufReader::new(File::open(&filename)?);
        Ok(Self {
            items: serde_json::from_reader(file)?,
        })
    }
}
