use ornaguide_rs::{
    codex::{CodexFollower, CodexItem, CodexSkill},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    items::admin::AdminItem,
    monsters::admin::AdminMonster,
    pets::admin::AdminPet,
    skills::admin::AdminSkill,
};
use serde::{Deserialize, Serialize};

use crate::{
    misc::{bar, sanitize_guide_name},
    retry_once,
};

/// Collection of items from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminItems {
    /// Items from the guide's admin view.
    pub items: Vec<AdminItem>,
}

/// Collection of monsters from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminMonsters {
    /// Monsters from the guide's admin view.
    pub monsters: Vec<AdminMonster>,
}

/// Collection of skills from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminSkills {
    /// Skills from the guide's admin view.
    pub skills: Vec<AdminSkill>,
}

/// Collection of pets from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminPets {
    /// Pets from the guide's admin view.
    pub pets: Vec<AdminPet>,
}

impl<'a> AdminItems {
    /// Find the admin item associated with the given codex item.
    pub fn find_match_for_codex_item(&'a self, needle: &CodexItem) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| {
            !item.codex_uri.is_empty()
                && item.codex_uri["/codex/items/".len()..].trim_end_matches('/') == needle.slug
        })
    }

    /// Find the admin item associated with the given codex item.
    /// If there is no match, return an `Err`.
    pub fn get_match_for_codex_item(&'a self, needle: &CodexItem) -> Result<&'a AdminItem, Error> {
        self.find_match_for_codex_item(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex item '{}'", needle.slug)))
    }

    /// Find the admin item associated with the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| item.id == needle)
    }

    /// Find the admin item associated with the given id.
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminItem, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No item with id {}", needle)))
    }

    /// Find the admin item associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| item.codex_uri == needle)
    }

    /// Find the admin item associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminItem, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No item with uri {}", needle)))
    }
}

impl<'a> AdminMonsters {
    /// Find the monster with the given codex uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminMonster> {
        self.monsters
            .iter()
            .find(|monster| monster.codex_uri == needle)
    }

    /// Find the monster with the given codex uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminMonster, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No monster with codex uri '{}'", needle)))
    }

    /// Find the monster with the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminMonster> {
        self.monsters.iter().find(|monster| monster.id == needle)
    }

    /// Find the monster with the given id
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminMonster, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No monster with id {}", needle)))
    }
}

impl<'a> AdminSkills {
    /// Find the admin skill associated with the given codex skill.
    pub fn find_match_for_codex_skill(&'a self, needle: &CodexSkill) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| {
            !skill.codex_uri.is_empty()
                && skill.codex_uri["/codex/spells/".len()..].trim_end_matches('/') == needle.slug
        })
    }

    /// Find the admin skill associated with the given codex skill.
    /// If there is no match, return an `Err`.
    pub fn get_match_for_codex_skill(
        &'a self,
        needle: &CodexSkill,
    ) -> Result<&'a AdminSkill, Error> {
        self.find_match_for_codex_skill(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex skill '{}'", needle.slug)))
    }

    /// Find the admin skill corresponding to the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.id == needle)
    }

    /// Find the admin skill corresponding to the given id.
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminSkill, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No match for admin skill with id #{}", needle)))
    }

    /// Find the admin skill corresponding to the given codex URI.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.codex_uri == needle)
    }

    /// Find the admin skill corresponding to the given codex URI.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin skill with codex_uri #{}",
                needle
            ))
        })
    }

    /// Find the admin offhand skill with the given name.
    pub fn find_offhand_from_name(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills
            .iter()
            .find(|skill| sanitize_guide_name(&skill.name) == needle && skill.offhand)
    }

    /// Find the admin offhand skill with the given name.
    /// If there is no match, return an `Err`.
    pub fn get_offhand_from_name(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_offhand_from_name(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex skill '{}'", needle)))
    }
}

impl<'a> AdminPets {
    /// Find the admin pet associated with the given codex follower.
    pub fn find_match_for_codex_follower(&'a self, needle: &CodexFollower) -> Option<&'a AdminPet> {
        self.pets.iter().find(|pet| {
            !pet.codex_uri.is_empty()
                && pet.codex_uri["/codex/followers/".len()..].trim_end_matches('/') == needle.slug
        })
    }

    /// Find the admin pet associated with the given codex follower.
    /// If there is no match, return an `Err`.
    pub fn get_match_for_codex_follower(
        &'a self,
        needle: &CodexFollower,
    ) -> Result<&'a AdminPet, Error> {
        self.find_match_for_codex_follower(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex follower '{}'", needle.slug)))
    }
}

pub fn items(guide: &OrnaAdminGuide) -> Result<AdminItems, Error> {
    let items = guide.admin_retrieve_items_list()?;
    let mut ret = Vec::new();
    let bar = bar(items.len() as u64);
    for item in items.iter() {
        bar.set_message(item.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_item_by_id(item.id))?);
        bar.inc(1);
    }
    bar.finish_with_message("AItems  fetched");
    Ok(AdminItems { items: ret })
}

pub fn monsters(guide: &OrnaAdminGuide) -> Result<AdminMonsters, Error> {
    let monsters = guide.admin_retrieve_monsters_list()?;
    let mut ret = Vec::new();
    let bar = bar(monsters.len() as u64);
    for monster in monsters.iter() {
        bar.set_message(monster.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_monster_by_id(monster.id))?);
        bar.inc(1);
    }
    bar.finish_with_message("AMnstrs fetched");
    Ok(AdminMonsters { monsters: ret })
}

pub fn skills(guide: &OrnaAdminGuide) -> Result<AdminSkills, Error> {
    let skills = guide.admin_retrieve_skills_list()?;
    let mut ret = Vec::new();
    let bar = bar(skills.len() as u64);
    for skill in skills.iter() {
        bar.set_message(skill.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_skill_by_id(skill.id))?);
        bar.inc(1);
    }
    bar.finish_with_message("ASkills fetched");
    Ok(AdminSkills { skills: ret })
}

pub fn pets(guide: &OrnaAdminGuide) -> Result<AdminPets, Error> {
    let pets = guide.admin_retrieve_pets_list()?;
    let mut ret = Vec::new();
    let bar = bar(pets.len() as u64);
    for pet in pets.iter() {
        bar.set_message(pet.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_pet_by_id(pet.id))?);
        bar.inc(1);
    }
    bar.finish_with_message("APets   fetched");
    Ok(AdminPets { pets: ret })
}
