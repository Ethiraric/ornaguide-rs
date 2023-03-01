use std::{fs::File, path::Path};

use ornaguide_rs::{
    codex::{CodexFollower, CodexItem, CodexMonster, CodexRaid, CodexSkill},
    data::{CodexData, GuideData},
    error::Error,
};
use serde::{Deserialize, Serialize};

use crate::backups::Backup;

/// Removals to be made on Orna Codex data.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CodexRemoval {
    /// Skill slugs to remove.
    pub skills: Vec<String>,
    /// Raid slugs to remove.
    pub raids: Vec<String>,
    /// Monster slugs to remove.
    pub monsters: Vec<String>,
    /// Item slugs to remove.
    pub items: Vec<String>,
}

/// Removals to be made on orna.guide data.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GuideRemoval {
    /// Item IDs to remove.
    pub items: Vec<u32>,
}

/// Removals to be made on a backup.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Removal {
    /// Removals to be made on Orna Codex data.
    pub codex: CodexRemoval,
    /// Removals to be made on orna.guide data.
    pub guide: GuideRemoval,
}

/// Overrides to be made on Orna Codex data.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CodexOverride {
    /// Item slugs to override.
    pub items: Vec<CodexItem>,
    /// Monster slugs to override.
    pub monsters: Vec<CodexMonster>,
    /// Raid slugs to override.
    pub raids: Vec<CodexRaid>,
    /// Skill slugs to override.
    pub skills: Vec<CodexSkill>,
    /// Follower slugs to override.
    pub followers: Vec<CodexFollower>,
}

/// Overrides to be made on a backup.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Override {
    /// Overrides to be made on Orna Codex data.
    pub codex: CodexOverride,
}

/// Hardcoded changes to be made to the backup data.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BackupChanges {
    /// Removals to be made on a backup.
    pub removal: Removal,
    /// Overrides to be made on a backup.
    #[serde(rename = "override")]
    pub override_: Override,
}

impl BackupChanges {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, backup: &mut Backup) {
        self.removal.apply_to(backup);
        self.override_.apply_to(backup);
    }

    /// Load changes from a json file.
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        File::open(path)
            .map_err(Error::from)
            .and_then(|file| serde_json::from_reader(file).map_err(Error::from))
    }
}

impl Removal {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, backup: &mut Backup) {
        self.codex.apply_to(&mut backup.data.codex);
        self.guide.apply_to(&mut backup.data.guide);
    }
}

impl CodexRemoval {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, data: &mut CodexData) {
        data.skills
            .skills
            .retain(|skill| !self.skills.contains(&skill.slug));
        data.raids
            .raids
            .retain(|raid| !self.raids.contains(&raid.slug));
        data.items
            .items
            .retain(|item| !self.items.contains(&item.slug));
        data.monsters
            .monsters
            .retain(|monster| !self.monsters.contains(&monster.slug));
    }
}

impl GuideRemoval {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, data: &mut GuideData) {
        data.items
            .items
            .retain(|item| !self.items.contains(&item.id));
    }
}

impl Override {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, backup: &mut Backup) {
        self.codex.apply_to(&mut backup.data.codex);
    }
}

impl CodexOverride {
    /// Apply the changes from `self` to the data.
    pub fn apply_to(&self, data: &mut CodexData) {
        for override_item in self.items.iter() {
            if let Some(item) = data
                .items
                .items
                .iter_mut()
                .find(|citem| citem.slug == override_item.slug)
            {
                *item = override_item.clone();
            }
        }
        for override_monster in self.monsters.iter() {
            if let Some(monster) = data
                .monsters
                .monsters
                .iter_mut()
                .find(|cmonster| cmonster.slug == override_monster.slug)
            {
                *monster = override_monster.clone();
            }
        }
        for override_raids in self.raids.iter() {
            if let Some(raid) = data
                .raids
                .raids
                .iter_mut()
                .find(|cmonster| cmonster.slug == override_raids.slug)
            {
                *raid = override_raids.clone();
            }
        }
        for override_skill in self.skills.iter() {
            if let Some(skill) = data
                .skills
                .skills
                .iter_mut()
                .find(|cskill| cskill.slug == override_skill.slug)
            {
                *skill = override_skill.clone();
            }
        }
        for override_follower in self.followers.iter() {
            if let Some(follower) = data
                .followers
                .followers
                .iter_mut()
                .find(|cfollower| cfollower.slug == override_follower.slug)
            {
                *follower = override_follower.clone();
            }
        }
    }
}
