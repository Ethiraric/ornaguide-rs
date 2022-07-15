#![allow(dead_code)]
use ornaguide_rs::{
    codex::{
        Codex, CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills,
    },
    error::Error,
    guide::OrnaAdminGuide,
};

use crate::misc::bar;

/// Retrieve all items from the codex.
pub fn items(guide: &OrnaAdminGuide) -> Result<CodexItems, Error> {
    let items = guide.codex_fetch_item_list()?;
    let mut ret = Vec::with_capacity(items.len());
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
    bar.finish_with_message("CItems  fetched");
    Ok(CodexItems { items: ret })
}

/// Retrieve all searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn monsters(guide: &OrnaAdminGuide) -> Result<CodexMonsters, Error> {
    let monsters = guide.codex_fetch_monster_list()?;
    let mut ret = Vec::with_capacity(monsters.len());
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

/// Retrieve all searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn bosses(guide: &OrnaAdminGuide) -> Result<CodexBosses, Error> {
    let bosses = guide.codex_fetch_boss_list()?;
    let mut ret = Vec::with_capacity(bosses.len());
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

/// Retrieve all searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn raids(guide: &OrnaAdminGuide) -> Result<CodexRaids, Error> {
    let raids = guide.codex_fetch_raid_list()?;
    let mut ret = Vec::with_capacity(raids.len());
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
    bar.finish_with_message("CRaids  fetched");
    Ok(CodexRaids { raids: ret })
}

/// Retrieve all skills from the codex.
pub fn skills(guide: &OrnaAdminGuide) -> Result<CodexSkills, Error> {
    let skills = guide.codex_fetch_skill_list()?;
    let mut ret = Vec::with_capacity(skills.len());
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

/// Retrieve all searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn followers(guide: &OrnaAdminGuide) -> Result<CodexFollowers, Error> {
    let followers = guide.codex_fetch_follower_list()?;
    let mut ret = Vec::with_capacity(followers.len());
    let bar = bar(followers.len() as u64);
    for follower in followers.iter() {
        let slug = follower
            .uri
            .strip_suffix('/')
            .unwrap()
            .strip_prefix("/codex/followers/")
            .unwrap();
        bar.set_message(slug.to_string());
        ret.push(guide.codex_fetch_follower(slug)?);
        bar.inc(1);
    }
    bar.finish_with_message("CFllwrs fetched");
    Ok(CodexFollowers { followers: ret })
}
