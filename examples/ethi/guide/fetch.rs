use ornaguide_rs::{
    codex::CodexItem,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    items::admin::AdminItem,
    monsters::admin::AdminMonster,
    skills::admin::AdminSkill,
};
use serde::{Deserialize, Serialize};

use crate::misc::{bar, sanitize_guide_name};

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

impl<'a> AdminItems {
    /// Find the codex item associated with the given admin monster.
    /// If there is no or multiple match, return an `Err`.
    pub fn find_match_for_codex_item(&'a self, needle: &CodexItem) -> Result<&'a AdminItem, Error> {
        let mut matches = self.items.iter().filter(|item| {
            sanitize_guide_name(&item.name) == needle.name
                && item.tier == needle.tier
                && needle.icon == item.image_name
        });
        if let Some(item) = matches.next() {
            if matches.next().is_some() {
                Err(Error::Misc(format!(
                    "Multiple matches for admin item '{}'",
                    needle.name
                )))
            } else {
                Ok(item)
            }
        } else {
            Err(Error::Misc(format!(
                "No match for admin item '{}'",
                needle.name
            )))
        }
    }
}

pub fn items(guide: &OrnaAdminGuide) -> Result<AdminItems, Error> {
    let items = guide.admin_retrieve_items_list()?;
    let mut ret = Vec::new();
    let bar = bar(items.len() as u64);
    for item in items.iter() {
        bar.set_message(item.name.clone());
        ret.push(guide.admin_retrieve_item_by_id(item.id)?);
        bar.inc(1);
    }
    bar.finish_with_message("AItems fetched");
    Ok(AdminItems { items: ret })
}

pub fn monsters(guide: &OrnaAdminGuide) -> Result<AdminMonsters, Error> {
    let monsters = guide.admin_retrieve_monsters_list()?;
    let mut ret = Vec::new();
    let bar = bar(monsters.len() as u64);
    for monster in monsters.iter() {
        bar.set_message(monster.name.clone());
        ret.push(guide.admin_retrieve_monster_by_id(monster.id)?);
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
        ret.push(guide.admin_retrieve_skill_by_id(skill.id)?);
        bar.inc(1);
    }
    bar.finish_with_message("ASkills fetched");
    Ok(AdminSkills { skills: ret })
}
