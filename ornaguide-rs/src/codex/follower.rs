use serde::{Deserialize, Serialize};

use crate::{
    data::GuideData,
    error::{Error, Kind},
    pets::admin::{AdminPet, CostType},
};

/// An ability for a follower.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The uri to the ability.
    pub uri: String,
    /// The icon of the ability.
    pub icon: String,
}

/// A follower on the codex.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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
#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct Followers {
    /// Followers from the codex.
    pub followers: Vec<Follower>,
}

impl Follower {
    /// Try to convert `self` to an `AdminPet`.
    ///
    ///  - Unknown skills are ignored, rather than returning an error.
    #[must_use]
    pub fn to_admin_pet(&self, guide_data: &GuideData) -> AdminPet {
        AdminPet {
            codex_uri: format!("/codex/followers/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            image_name: self.icon.clone(),
            description: if self.description.is_empty() {
                ".".to_string()
            } else {
                self.description.clone()
            },
            cost_type: if self.tier >= 8 {
                CostType::Orn
            } else {
                CostType::Gold
            },
            limited: !self.events.is_empty(),
            limited_details: self.events.join(", "),
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
            ..AdminPet::default()
        }
    }
}

impl<'a> Followers {
    /// Find the codex follower associated with the given admin pet.
    #[must_use]
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Follower> {
        static URI_START: &str = "/codex/followers/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.followers.iter().find(|follower| follower.slug == slug)
    }

    /// Find the codex follower associated with the given admin pet.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Follower, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Kind::Misc(format!("No match for follower with uri '{needle}'")).into())
    }
}
