use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// A class or specialization learning a spell.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkillLearnedBy {
    /// Id of the class or specialization.
    pub id: u32,
    /// Name of the class or specialization.
    pub name: String,
    /// Level at which the class or specialization learns the spell.
    pub level: u32,
    /// Whether this entry is for a class or specialization.
    pub specialization: bool,
}

/// A monster using the skill.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkillMonsterUse {
    /// Id of the monster.
    pub id: u32,
    /// Name of the monster
    pub name: String,
}

/// A pet using the skill.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkillPetUse {
    /// Id of the pet.
    pub id: u32,
    /// Name of the pet.
    pub name: String,
}

/// A monster buffing the skill (for charged passive.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkillBuffedBy {
    /// Id of the monster.
    pub id: u32,
    /// Name of the monster.
    pub name: String,
}

/// An object representation of a skill in Orna from the API json. This encompasses both passive
/// and active skills.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub is_magic: bool,
    pub bought: bool,
    pub buffed_by: Option<Vec<SkillBuffedBy>>,
    pub cost: Option<u64>,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub cures: Option<Vec<String>>,
    pub causes: Option<Vec<String>>,
    pub gives: Option<Vec<String>>,
    pub learned_by: Option<Vec<SkillLearnedBy>>,
    pub pets_use: Option<Vec<SkillPetUse>>,
    pub monsters_use: Option<Vec<SkillMonsterUse>>,
}

/// A set of skills json objects. This is used to attach methods for parsing.
pub struct RawSkills {
    pub skills: Vec<RawSkill>,
}

impl RawSkills {
    /// Parse a set of skills from a json string.
    pub fn parse_from_json(s: &str) -> Result<Self, Error> {
        Ok(Self {
            skills: serde_json::from_str(s)?,
        })
    }

    /// Parse a set of skills from a filename.
    pub fn parse_from_file(filename: &str) -> Result<Self, Error> {
        let file = BufReader::new(File::open(&filename)?);
        Ok(Self {
            skills: serde_json::from_reader(file)?,
        })
    }
}
