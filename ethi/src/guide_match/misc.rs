use itertools::Itertools;
use ornaguide_rs::{
    codex::{FollowerAbility, ItemDroppedBy, ItemUpgradeMaterial, MonsterAbility},
    error::{Error, ErrorKind},
    guide::Static,
    items::admin::AdminItems,
    monsters::admin::AdminMonsters,
    skills::admin::AdminSkills,
};

/// A trait to extend `Vec<ItemDroppedBy>` specifically.
pub trait ItemDroppedBys {
    /// Convert `self` to a `Vec<u32>`, with `u32`s being the guide monster ids.
    /// Returns `ErrorKind::PartialCodexItemDroppedBysConversion` if all fields have not been
    /// successfully converted.
    fn try_to_guide_ids(&self, monsters: &AdminMonsters) -> Result<Vec<u32>, Error>;
}

impl ItemDroppedBys for Vec<ItemDroppedBy> {
    fn try_to_guide_ids(&self, monsters: &AdminMonsters) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|dropped_by| {
                monsters
                    .get_by_uri(&dropped_by.uri)
                    .map(|monster| monster.id)
                    .map_err(|_| dropped_by.uri.clone())
            })
            .partition_result();

        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(ErrorKind::PartialCodexItemDroppedBysConversion(successes, failures).into())
        }
    }
}

/// A trait to extend `Vec<ItemUpgradeMaterial>` specifically.
pub trait ItemUpgradeMaterials {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide item ids.
    /// Returns `ErrorKind::PartialCodexItemDroppedBysConversion` if all fields have not been
    /// successfully converted.
    fn try_to_guide_ids(&self, items: &AdminItems) -> Result<Vec<u32>, Error>;
}

impl ItemUpgradeMaterials for Vec<ItemUpgradeMaterial> {
    fn try_to_guide_ids(&self, items: &AdminItems) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|dropped_by| {
                items
                    .get_by_uri(&dropped_by.uri)
                    .map(|item| item.id)
                    .map_err(|_| dropped_by.uri.clone())
            })
            .partition_result();

        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(ErrorKind::PartialCodexItemUpgradeMaterialsConversion(successes, failures).into())
        }
    }
}

/// A trait to extend `Vec`s of codex abilities.
pub trait CodexAbilities {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide skill ids.
    /// Returns `ErrorKind::PartialCodexFollowerAbilitiesConversion` or
    /// `ErrorKind::PartialCodexMonsterAbilitiesConversion` if all fields have not been successfully
    /// converted.
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error>;
}

impl CodexAbilities for Vec<FollowerAbility> {
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|ability| {
                skills
                    .get_by_uri(&ability.uri)
                    .map(|skill| skill.id)
                    .map_err(|_| ability.uri.clone())
            })
            .partition_result();

        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(ErrorKind::PartialCodexFollowerAbilitiesConversion(successes, failures).into())
        }
    }
}

impl CodexAbilities for Vec<MonsterAbility> {
    fn try_to_guide_ids(&self, skills: &AdminSkills) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|ability| {
                skills
                    .get_by_uri(&ability.uri)
                    .map(|skill| skill.id)
                    .map_err(|_| ability.uri.clone())
            })
            .partition_result();

        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(ErrorKind::PartialCodexMonsterAbilitiesConversion(successes, failures).into())
        }
    }
}

/// A trait to extend `Vec`s of event names.
pub trait EventsNames {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide event ids.
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error>;
}

impl EventsNames for Vec<&str> {
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|event| {
                static_
                    .iter_events()
                    .find(|spawn| spawn.event_name() == *event)
                    .map(|spawn| spawn.id)
                    .ok_or_else(|| event.to_string())
            })
            .partition_result();

        if failures.is_empty() {
            Ok(successes)
        } else {
            Err(ErrorKind::PartialCodexEventsConversion(successes, failures).into())
        }
    }
}
