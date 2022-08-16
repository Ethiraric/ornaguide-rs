#![allow(dead_code)]

use ornaguide_rs::{
    codex::{
        translation::{
            BossTranslation, FollowerTranslation, ItemTranslation, LocaleStrings,
            MonsterTranslation, RaidTranslation, SkillTranslation,
        },
        Codex, CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills,
    },
    data::{CodexData, OrnaData},
    error::Error,
    guide::OrnaAdminGuide,
    misc::codex_effect_name_to_guide_name,
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

/// Retrieve all items from the codex.
pub fn items_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexItems, Error> {
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
        ret.push(guide.codex_fetch_item_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CItems  fetched");
    Ok(CodexItems { items: ret })
}

/// Retrieve all searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn monsters_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexMonsters, Error> {
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
        ret.push(guide.codex_fetch_monster_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CMnstrs fetched");
    Ok(CodexMonsters { monsters: ret })
}

/// Retrieve all searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn bosses_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexBosses, Error> {
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
        ret.push(guide.codex_fetch_boss_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CBosses fetched");
    Ok(CodexBosses { bosses: ret })
}

/// Retrieve all searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn raids_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexRaids, Error> {
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
        ret.push(guide.codex_fetch_raid_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CRaids  fetched");
    Ok(CodexRaids { raids: ret })
}

/// Retrieve all skills from the codex.
pub fn skills_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexSkills, Error> {
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
        ret.push(guide.codex_fetch_skill_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CSkills fetched");
    Ok(CodexSkills { skills: ret })
}

/// Retrieve all searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn followers_translations(
    guide: &OrnaAdminGuide,
    locale: &str,
) -> Result<CodexFollowers, Error> {
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
        ret.push(guide.codex_fetch_follower_with_locale(slug, locale)?);
        bar.inc(1);
    }
    bar.finish_with_message("CFllwrs fetched");
    Ok(CodexFollowers { followers: ret })
}

/// Fetch the translation strings in the given locale.
pub fn translations(
    guide: &OrnaAdminGuide,
    data: &OrnaData,
    locale: &str,
) -> Result<LocaleStrings, Error> {
    let codex = CodexData {
        items: items_translations(guide, locale)?,
        raids: raids_translations(guide, locale)?,
        monsters: monsters_translations(guide, locale)?,
        bosses: bosses_translations(guide, locale)?,
        skills: skills_translations(guide, locale)?,
        followers: followers_translations(guide, locale)?,
    };
    let mut strings = LocaleStrings {
        locale: locale.to_string(),
        ..Default::default()
    };

    // Insert items.
    for item in codex.items.items.into_iter() {
        strings.items.insert(
            item.slug.to_string(),
            ItemTranslation {
                name: item.name,
                description: item.description,
            },
        );
    }

    // Insert raids and events.
    for raid in codex.raids.raids.into_iter() {
        let raid_data = data
            .codex
            .raids
            .find_by_uri(&format!("/codex/raids/{}/", raid.slug))
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find raid {} (found in locale {})",
                    raid.slug, locale
                ))
            })?;

        // Update strings not directly related to the raid.
        // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
        for (localed, en) in raid.events.iter().zip(raid_data.events.iter()) {
            strings.events.insert(en.clone(), localed.clone());
        }

        strings.raids.insert(
            raid.slug.to_string(),
            RaidTranslation {
                name: raid.name,
                description: raid.description,
            },
        );
    }

    // Insert monsters, events, family and rarity.
    for monster in codex.monsters.monsters.into_iter() {
        let monster_data = data
            .codex
            .monsters
            .find_by_uri(&format!("/codex/monsters/{}/", monster.slug))
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find monster {} (found in locale {})",
                    monster.slug, locale
                ))
            })?;

        // Update strings not directly related to the monster.
        // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
        for (localed, en) in monster.events.iter().zip(monster_data.events.iter()) {
            strings.events.insert(en.clone(), localed.clone());
        }
        strings
            .families
            .insert(monster_data.family.clone(), monster.family.clone());
        strings
            .rarities
            .insert(monster_data.rarity.clone(), monster.rarity.clone());

        strings.monsters.insert(
            monster.slug.to_string(),
            MonsterTranslation { name: monster.name },
        );
    }

    // Insert boss, events, family and rarity.
    for boss in codex.bosses.bosses.into_iter() {
        let boss_data = data
            .codex
            .bosses
            .find_by_uri(&format!("/codex/bosses/{}/", boss.slug))
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find boss {} (found in locale {})",
                    boss.slug, locale
                ))
            })?;

        // Update strings not directly related to the boss.
        // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
        for (localed, en) in boss.events.iter().zip(boss_data.events.iter()) {
            strings.events.insert(en.clone(), localed.clone());
        }
        strings
            .rarities
            .insert(boss_data.rarity.clone(), boss.rarity.clone());

        strings
            .bosses
            .insert(boss.slug.to_string(), BossTranslation { name: boss.name });
    }

    // Insert skill and statuses.
    for skill in codex.skills.skills.into_iter() {
        let skill_data = data
            .codex
            .skills
            .find_by_uri(&format!("/codex/spells/{}/", skill.slug))
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find skill {} (found in locale {})",
                    skill.slug, locale
                ))
            })?;

        // Update strings not directly related to the skill.
        // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
        for (localed, en) in skill
            .causes
            .iter()
            .zip(skill_data.causes.iter().chain(skill_data.gives.iter()))
        {
            strings.statuses.insert(
                codex_effect_name_to_guide_name(&en.effect).to_string(),
                localed.effect.clone(),
            );
        }

        strings.skills.insert(
            skill.slug.to_string(),
            SkillTranslation {
                name: skill.name,
                description: skill.description,
            },
        );
    }

    // Insert follower and events.
    for follower in codex.followers.followers.into_iter() {
        let follower_data = data
            .codex
            .followers
            .find_by_uri(&format!("/codex/followers/{}/", follower.slug))
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find follower {} (found in locale {})",
                    follower.slug, locale
                ))
            })?;

        // Update strings not directly related to the follower.
        // TODO(ethiraric, 09/08/2022): Remove clones. Use try_insert?
        for (localed, en) in follower.events.iter().zip(follower_data.events.iter()) {
            strings.events.insert(en.clone(), localed.clone());
        }

        strings.followers.insert(
            follower.slug.to_string(),
            FollowerTranslation {
                name: follower.name,
                description: follower.description,
            },
        );
    }

    Ok(strings)
}
