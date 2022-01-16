use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// The attack stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemAttackStat {
    pub base: i32,
}

/// The crit stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemCritStat {
    pub base: i32,
}

/// The defense stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemDefenseStat {
    pub base: i32,
}

/// The HP stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemHPStat {
    pub base: i32,
}

/// The magic stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemMagicStat {
    pub base: i32,
}

/// The mana stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemManaStat {
    pub base: i32,
}

/// The dexterity stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemDexterityStat {
    pub base: i32,
}

/// The resistance stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemResistanceStat {
    pub base: i32,
}

/// The ward stat from an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemWardStat {
    pub base: i32,
}

/// The stats of an item (equipment, adornment).
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemStats {
    pub attack: Option<ItemAttackStat>,
    pub crit: Option<ItemCritStat>,
    pub defense: Option<ItemDefenseStat>,
    pub hp: Option<ItemHPStat>,
    pub magic: Option<ItemMagicStat>,
    pub mana: Option<ItemManaStat>,
    pub dexterity: Option<ItemDexterityStat>,
    pub resistance: Option<ItemResistanceStat>,
    pub ward: Option<ItemWardStat>,
}

/// A material needed to upgrade an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemMaterial {
    pub id: u32,
    pub name: String,
}

/// A monster that can drop an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemDroppedBy {
    pub id: u32,
    pub name: String,
}

/// A quest which rewards with an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemQuest {
    pub id: u32,
    pub name: String,
}

/// A category of classes that can equip an item.
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ItemEquippedBy {
    pub id: u32,
    pub name: String,
}

/// An object representation of an item in Orna from the API json. This encompasses consumables,
/// equipment, adornments, materials, ...
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RawItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub view_distance: Option<bool>,
    pub stats: Option<ItemStats>,
    pub element: Option<String>,
    pub materials: Option<Vec<ItemMaterial>>,
    pub dropped_by: Option<Vec<ItemDroppedBy>>,
    pub quests: Option<Vec<ItemQuest>>,
    pub equipped_by: Option<Vec<ItemEquippedBy>>,
    pub prevents: Option<Vec<String>>,
    pub causes: Option<Vec<String>>,
    pub cures: Option<Vec<String>>,
    pub gives: Option<Vec<String>>,
    pub category: Option<String>,
}

/// A set of items json objects. This is used to attach methods for parsing.
pub struct RawItems {
    pub items: Vec<RawItem>,
}

impl RawItems {
    /// Parse a set of items from a json string.
    pub fn parse_from_json(s: &str) -> Result<Self, Error> {
        Ok(Self {
            items: serde_json::from_str(s)?,
        })
    }

    /// Parse a set of items from a filename.
    pub fn parse_from_file(filename: &str) -> Result<Self, Error> {
        let file = BufReader::new(File::open(&filename)?);
        Ok(Self {
            items: serde_json::from_reader(file)?,
        })
    }
}
