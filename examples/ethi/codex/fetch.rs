#![allow(dead_code)]
use ornaguide_rs::{
    codex::{Codex, CodexBoss, CodexItem, CodexMonster, CodexRaid, CodexSkill, Tag},
    error::Error,
    guide::OrnaAdminGuide,
    items::admin::AdminItem,
};
use serde::{Deserialize, Serialize};

use crate::misc::{bar, sanitize_guide_name};

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
    pub fn find_match_for_admin_item(&'a self, needle: &AdminItem) -> Result<&'a CodexItem, Error> {
        let mut matches = self.items.iter().filter(|item| {
            item.name == sanitize_guide_name(&needle.name)
                && item.tier == needle.tier
                && item.icon == needle.image_name
        });
        if let Some(item) = matches.next() {
            if matches.next().is_some() {
                Err(Error::Misc(format!(
                    "Multiple matches for raw item '{}'",
                    needle.name
                )))
            } else {
                Ok(item)
            }
        } else {
            Err(Error::Misc(format!(
                "No match for raw item '{}'",
                needle.name
            )))
        }
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
