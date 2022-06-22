#![allow(dead_code)]
use ornaguide_rs::{
    codex::{Codex, CodexBoss, CodexItem, CodexMonster, CodexRaid, CodexSkill},
    error::Error,
    guide::OrnaAdminGuide,
    items::admin::AdminItem,
    skills::admin::AdminSkill,
};
use serde::{Deserialize, Serialize};

use crate::misc::bar;

/// Collection of items from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexItems {
    /// Items from the codex.
    pub items: Vec<CodexItem>,
}

/// Collection of monsters from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexMonsters {
    /// Monsters from the codex.
    pub monsters: Vec<CodexMonster>,
}

/// Collection of bosses from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexBosses {
    /// Bosses from the codex.
    pub bosses: Vec<CodexBoss>,
}

/// Collection of raids from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexRaids {
    /// Raids from the codex.
    pub raids: Vec<CodexRaid>,
}

/// Collection of skills from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexSkills {
    /// Skills from the codex.
    pub skills: Vec<CodexSkill>,
}

impl<'a> CodexItems {
    /// Find the codex item associated with the given admin item.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_admin_item(&'a self, needle: &AdminItem) -> Result<&'a CodexItem, Error> {
        if needle.codex_uri.is_empty() {
            return Err(Error::Misc(format!(
                "Empty codex uri for admin item '{}'",
                needle.name
            )));
        }

        let slug = needle.codex_uri["/codex/items/".len()..].trim_end_matches('/');
        self.items
            .iter()
            .find(|item| item.slug == slug)
            .ok_or_else(|| Error::Misc(format!("No match for admin item '{}'", needle.name)))
    }
}

impl<'a> CodexSkills {
    /// Find the codex skill associated with the given admin skill.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_admin_skill(
        &'a self,
        needle: &AdminSkill,
    ) -> Result<&'a CodexSkill, Error> {
        if needle.codex_uri.is_empty() {
            return Err(Error::Misc(format!(
                "Empty codex uri for admin skill '{}'",
                needle.name
            )));
        }

        let slug = needle.codex_uri["/codex/spells/".len()..].trim_end_matches('/');
        self.skills
            .iter()
            .find(|skill| skill.slug == slug)
            .ok_or_else(|| Error::Misc(format!("No match for admin skill '{}'", needle.name)))
    }
}

pub fn items(guide: &OrnaAdminGuide) -> Result<CodexItems, Error> {
    let items = guide.codex_fetch_item_list()?;
    let mut ret = Vec::new();
    let bar = bar(items.len() as u64);
    for item in items.iter() {
        let slug = item
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/items/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_item(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CItems fetched");
    Ok(CodexItems { items: ret })
}

pub fn monsters(guide: &OrnaAdminGuide) -> Result<CodexMonsters, Error> {
    let monsters = guide.codex_fetch_monster_list()?;
    let mut ret = Vec::new();
    let bar = bar(monsters.len() as u64);
    for monster in monsters.iter() {
        let slug = monster
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/monsters/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_monster(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CMnstrs fetched");
    Ok(CodexMonsters { monsters: ret })
}

pub fn bosses(guide: &OrnaAdminGuide) -> Result<CodexBosses, Error> {
    let bosses = guide.codex_fetch_boss_list()?;
    let mut ret = Vec::new();
    let bar = bar(bosses.len() as u64);
    for boss in bosses.iter() {
        let slug = boss
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/bosses/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_boss(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CBosses fetched");
    Ok(CodexBosses { bosses: ret })
}

pub fn raids(guide: &OrnaAdminGuide) -> Result<CodexRaids, Error> {
    let raids = guide.codex_fetch_raid_list()?;
    let mut ret = Vec::new();
    let bar = bar(raids.len() as u64);
    for raid in raids.iter() {
        let slug = raid
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/raids/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_raid(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CRaids fetched");
    Ok(CodexRaids { raids: ret })
}

pub fn skills(guide: &OrnaAdminGuide) -> Result<CodexSkills, Error> {
    let skills = guide.codex_fetch_skill_list()?;
    let mut ret = Vec::new();
    let bar = bar(skills.len() as u64);
    for skill in skills.iter() {
        let slug = skill
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/spells/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_skill(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CSkills fetched");
    Ok(CodexSkills { skills: ret })
}
