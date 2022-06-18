use serde::{Deserialize, Serialize};

/// A spawn for monsters.
#[derive(Debug, Serialize, Deserialize)]
pub struct Spawn {
    /// Id of the spawn.
    pub id: u32,
    /// Name of the spawn.
    pub name: String,
}

/// An item category.
///
/// E.g: Fish, Instrument, Curved Sword, ...
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemCategory {
    /// Id of the category.
    pub id: u32,
    /// Name of the category.
    pub name: String,
}

/// An item type.
///
/// E.g.: Fish, Adornment, Off-hand, ...
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemType {
    /// Id of the type.
    pub id: u32,
    /// Name of the type.
    pub name: String,
}

/// A monster family.
#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterFamily {
    /// Id of the family.
    pub id: u32,
    /// Name of the family.
    pub name: String,
}

/// A status effect.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusEffect {
    /// Id of the status effect.
    pub id: u32,
    /// Name of the status effect.
    pub name: String,
}

/// An element.
#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    /// Id of the element.
    pub id: u32,
    /// Name of the element.
    pub name: String,
}

/// A class who can equip an item.
#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedBy {
    /// Id of the entry.
    pub id: u32,
    /// Name of the entry.
    pub name: String,
}

/// A skill type (passive, magic, AoE, ...).
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillType {
    /// Id of the skill type.
    pub id: u32,
    /// Name of the skill type.
    pub name: String,
}

/// Static resources that are used by the guide.
#[derive(Debug, Serialize, Deserialize)]
pub struct Static {
    /// List of monster spawns.
    pub spawns: Vec<Spawn>,
    /// List of item categories.
    pub item_categories: Vec<ItemCategory>,
    /// List of item types.
    pub item_types: Vec<ItemType>,
    /// List of monster families.
    pub monster_families: Vec<MonsterFamily>,
    /// List of status effects.
    pub status_effects: Vec<StatusEffect>,
    /// List of elements.
    pub elements: Vec<Element>,
    /// List of `equipped_by`s.
    pub equipped_bys: Vec<EquippedBy>,
    /// List of skill types.
    pub skill_types: Vec<SkillType>,
}

impl Spawn {
    /// Return the name of the event (without `Event:` or `Past Event:` prepended).
    /// Returns an empty string if the spawn isn't an event.
    pub fn event_name(&self) -> &str {
        if self.name.starts_with("Event:") {
            &self.name[7..]
        } else if self.name.starts_with("Past Event:") {
            &self.name[12..]
        } else {
            ""
        }
    }
}

impl Static {
    /// Return an iterator over all event spawns in the guide.
    pub fn iter_events(&self) -> impl Iterator<Item = &Spawn> {
        self.spawns.iter().filter(|spawn| {
            spawn.name.starts_with("Event:") || spawn.name.starts_with("Past Event:")
        })
    }
}
