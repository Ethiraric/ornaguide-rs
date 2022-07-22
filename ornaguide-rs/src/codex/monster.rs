use serde::{Deserialize, Serialize};

pub use crate::guide::html_utils::Tag;

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
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
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
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
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