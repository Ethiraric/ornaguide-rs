use serde::{Deserialize, Serialize};

/// An ability for a follower.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The uri to the ability.
    pub uri: String,
    /// The icon of the ability.
    pub icon: String,
}

/// A follower on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct Follower {
    /// The name of the follower.
    pub name: String,
    /// The slug of the follower (`https://playorna.com/codex/followers/{slug}`).
    pub slug: String,
    /// The icon of the follower.
    pub icon: String,
    /// The description of the follower.
    pub description: String,
    /// The events in which the follower appears.
    pub events: Vec<String>,
    /// The rarity of the follower.
    pub rarity: String,
    /// The tier of the follower.
    pub tier: u8,
    /// The abilities of the follower.
    pub abilities: Vec<Ability>,
}
