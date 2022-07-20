use serde::{Deserialize, Serialize};

use crate::error::Error;

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

/// Collection of followers from the codex.
#[derive(Serialize, Deserialize)]
pub struct Followers {
    /// Followers from the codex.
    pub followers: Vec<Follower>,
}

impl<'a> Followers {
    /// Find the codex follower associated with the given admin pet.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Follower> {
        static URI_START: &str = "/codex/followers/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.followers.iter().find(|follower| follower.slug == slug)
    }

    /// Find the codex follower associated with the given admin pet.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Follower, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for follower with uri '{}'", needle)))
    }
}
