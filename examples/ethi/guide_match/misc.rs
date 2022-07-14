use ornaguide_rs::{
    codex::{FollowerAbility, ItemDroppedBy, ItemUpgradeMaterial, MonsterAbility},
    error::Error,
    guide::Static,
};

use crate::guide::fetch::{AdminItems, AdminMonsters, AdminSkills};

/// A trait to extend `Vec<ItemDroppedBy>` specifically.
pub trait ItemDroppedBys {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide monster ids.
    fn try_to_guide_ids(&self, monsters: &AdminMonsters) -> Result<Vec<u32>, Error>;
}

impl ItemDroppedBys for Vec<ItemDroppedBy> {
    fn try_to_guide_ids(&self, monsters: &AdminMonsters) -> Result<Vec<u32>, Error> {
        self.iter()
            .map(|dropped_by| {
                monsters
                    .get_by_uri(&dropped_by.uri)
                    .map(|monster| monster.id)
            })
            .collect()
    }
}

/// A trait to extend `Vec<ItemUpgradeMaterial>` specifically.
pub trait ItemUpgradeMaterials {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide item ids.
    fn try_to_guide_ids(&self, items: &AdminItems) -> Result<Vec<u32>, Error>;
}

impl ItemUpgradeMaterials for Vec<ItemUpgradeMaterial> {
    fn try_to_guide_ids(&self, items: &AdminItems) -> Result<Vec<u32>, Error> {
        self.iter()
            .map(|dropped_by| items.get_by_uri(&dropped_by.uri).map(|item| item.id))
            .collect()
    }
}

/// A trait to extend `Vec`s of codex abilities.
pub trait CodexAbilities {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide skill ids.
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error>;
}

impl CodexAbilities for Vec<FollowerAbility> {
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error> {
        self.iter()
            .map(|ability| skills.get_by_uri(&ability.uri).map(|skill| skill.id))
            .collect()
    }
}

impl CodexAbilities for Vec<MonsterAbility> {
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error> {
        self.iter()
            .map(|ability| skills.get_by_uri(&ability.uri).map(|skill| skill.id))
            .collect()
    }
}

/// A trait to extend `Vec`s of event names.
pub trait EventsNames {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide event ids.
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error>;
}

impl EventsNames for Vec<&str> {
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error> {
        self.iter()
            .map(|event| {
                static_
                    .iter_events()
                    .find(|spawn| spawn.event_name() == *event)
                    .map(|spawn| spawn.id)
                    .ok_or_else(|| Error::Misc(format!("Failed to find event {}", event)))
            })
            .collect()
    }
}
