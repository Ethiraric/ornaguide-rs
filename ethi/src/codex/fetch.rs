#![allow(dead_code)]
use itertools::Itertools;
use ornaguide_rs::{
    codex::{
        translation::{
            BossTranslation, FollowerTranslation, ItemTranslation, LocaleStrings,
            MonsterTranslation, RaidTranslation, SkillTranslation,
        },
        Codex, CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills,
        Sluggable,
    },
    data::{CodexData, OrnaData},
    error::Error,
    guide::OrnaAdminGuide,
    misc::codex_effect_name_to_guide_name,
};

use crate::misc::bar;

/// Loop fetching entities and displaying a progress bar.
/// Errors out after the first failed fetch.
fn fetch_loop<Entry, F, Entity>(
    entries: &[Entry],
    fetch: F,
    kind: &str,
) -> Result<Vec<Entity>, Error>
where
    Entry: Sluggable,
    F: Fn(&str) -> Result<Entity, Error>,
{
    let sleep = crate::config::playorna_sleep()? as u64;
    let mut ret = Vec::with_capacity(entries.len());
    let bar = bar(entries.len() as u64);
    for entry in entries.iter() {
        let slug = entry.slug();
        bar.set_message(slug.to_string());
        match fetch(slug) {
            Ok(item) => ret.push(item),
            Err(x) => eprintln!("Failed to fetch {} {}: {}\n", kind, slug, x),
        }
        bar.inc(1);
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_secs(sleep));
        }
    }
    bar.finish_with_message(format!("{:7 } fetched", kind));
    Ok(ret)
}

/// Retrieve all items from the codex.
pub fn items(guide: &OrnaAdminGuide) -> Result<CodexItems, Error> {
    fetch_loop(
        &guide.codex_fetch_item_list()?,
        |slug| guide.codex_fetch_item(slug),
        "CItems",
    )
    .map(|items| CodexItems { items })
}

/// Retrieve all searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn monsters(guide: &OrnaAdminGuide) -> Result<CodexMonsters, Error> {
    fetch_loop(
        &guide.codex_fetch_monster_list()?,
        |slug| guide.codex_fetch_monster(slug),
        "CMnstrs",
    )
    .map(|monsters| CodexMonsters { monsters })
}

/// Retrieve all searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn bosses(guide: &OrnaAdminGuide) -> Result<CodexBosses, Error> {
    fetch_loop(
        &guide.codex_fetch_boss_list()?,
        |slug| guide.codex_fetch_boss(slug),
        "CBosses",
    )
    .map(|bosses| CodexBosses { bosses })
}

/// Retrieve all searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn raids(guide: &OrnaAdminGuide) -> Result<CodexRaids, Error> {
    fetch_loop(
        &guide.codex_fetch_raid_list()?,
        |slug| guide.codex_fetch_raid(slug),
        "CRaids",
    )
    .map(|raids| CodexRaids { raids })
}

/// Retrieve all skills from the codex.
pub fn skills(guide: &OrnaAdminGuide) -> Result<CodexSkills, Error> {
    fetch_loop(
        &guide.codex_fetch_skill_list()?,
        |slug| guide.codex_fetch_skill(slug),
        "CSkills",
    )
    .map(|skills| CodexSkills { skills })
}

/// Retrieve all searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn followers(guide: &OrnaAdminGuide) -> Result<CodexFollowers, Error> {
    fetch_loop(
        &guide.codex_fetch_follower_list()?,
        |slug| guide.codex_fetch_follower(slug),
        "CFollwrs",
    )
    .map(|followers| CodexFollowers { followers })
}

/// Retrieve all missing items from the codex.
pub fn missing_items(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexItems, Error> {
    fetch_loop(
        &guide
            .codex_fetch_item_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .items
                    .items
                    .iter()
                    .any(|item| item.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_item(slug),
        "CItems",
    )
    .map(|items| CodexItems { items })
}

/// Retrieve all missing searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn missing_monsters(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexMonsters, Error> {
    fetch_loop(
        &guide
            .codex_fetch_monster_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .monsters
                    .monsters
                    .iter()
                    .any(|monster| monster.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_monster(slug),
        "CMnstrs",
    )
    .map(|monsters| CodexMonsters { monsters })
}

/// Retrieve all missing searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn missing_bosses(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexBosses, Error> {
    fetch_loop(
        &guide
            .codex_fetch_boss_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .bosses
                    .bosses
                    .iter()
                    .any(|boss| boss.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_boss(slug),
        "CBosses",
    )
    .map(|bosses| CodexBosses { bosses })
}

/// Retrieve all missing searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn missing_raids(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexRaids, Error> {
    fetch_loop(
        &guide
            .codex_fetch_raid_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .raids
                    .raids
                    .iter()
                    .any(|raid| raid.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_raid(slug),
        "CRaids",
    )
    .map(|raids| CodexRaids { raids })
}

/// Retrieve all missing skills from the codex.
pub fn missing_skills(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexSkills, Error> {
    fetch_loop(
        &guide
            .codex_fetch_skill_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .skills
                    .skills
                    .iter()
                    .any(|skill| skill.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_skill(slug),
        "CSkills",
    )
    .map(|skills| CodexSkills { skills })
}

/// Retrieve all missing searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn missing_followers(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexFollowers, Error> {
    fetch_loop(
        &guide
            .codex_fetch_follower_list()?
            .into_iter()
            .filter(|entry| {
                !data
                    .codex
                    .followers
                    .followers
                    .iter()
                    .any(|follower| follower.slug == entry.slug())
            })
            .collect_vec(),
        |slug| guide.codex_fetch_follower(slug),
        "CFollwrs",
    )
    .map(|followers| CodexFollowers { followers })
}

/// Retrieve all missing accessible data from the codex.
pub fn missing(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<CodexData, Error> {
    Ok(CodexData {
        items: missing_items(guide, data)?,
        raids: missing_raids(guide, data)?,
        monsters: missing_monsters(guide, data)?,
        bosses: missing_bosses(guide, data)?,
        skills: missing_skills(guide, data)?,
        followers: missing_followers(guide, data)?,
    })
}

/// Retrieve all items from the codex.
pub fn items_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexItems, Error> {
    fetch_loop(
        &guide.codex_fetch_item_list()?,
        |slug| guide.codex_fetch_item_with_locale(slug, locale),
        "CItems",
    )
    .map(|items| CodexItems { items })
}

/// Retrieve all searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn monsters_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexMonsters, Error> {
    fetch_loop(
        &guide.codex_fetch_monster_list()?,
        |slug| guide.codex_fetch_monster_with_locale(slug, locale),
        "CMnstrs",
    )
    .map(|monsters| CodexMonsters { monsters })
}

/// Retrieve all searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn bosses_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexBosses, Error> {
    fetch_loop(
        &guide.codex_fetch_boss_list()?,
        |slug| guide.codex_fetch_boss_with_locale(slug, locale),
        "CBosses",
    )
    .map(|bosses| CodexBosses { bosses })
}

/// Retrieve all searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn raids_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexRaids, Error> {
    fetch_loop(
        &guide.codex_fetch_raid_list()?,
        |slug| guide.codex_fetch_raid_with_locale(slug, locale),
        "CRaids",
    )
    .map(|raids| CodexRaids { raids })
}

/// Retrieve all skills from the codex.
pub fn skills_translations(guide: &OrnaAdminGuide, locale: &str) -> Result<CodexSkills, Error> {
    fetch_loop(
        &guide.codex_fetch_skill_list()?,
        |slug| guide.codex_fetch_skill_with_locale(slug, locale),
        "CSkills",
    )
    .map(|skills| CodexSkills { skills })
}

/// Retrieve all searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn followers_translations(
    guide: &OrnaAdminGuide,
    locale: &str,
) -> Result<CodexFollowers, Error> {
    fetch_loop(
        &guide.codex_fetch_follower_list()?,
        |slug| guide.codex_fetch_follower_with_locale(slug, locale),
        "CFollwrs",
    )
    .map(|followers| CodexFollowers { followers })
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
