#![allow(dead_code)]
use itertools::Itertools;
use ornaguide_rs::{
    codex::{
        translation::{LocaleDB, LocaleStrings},
        Codex, CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills,
        Sluggable,
    },
    data::{CodexData, OrnaData},
    error::Error,
    guide::OrnaAdminGuide,
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

    strings.add_items(codex.items.items);
    strings.add_raids_and_events(codex.raids.raids, &data)?;
    strings.add_monsters_events_families_and_rarities(codex.monsters.monsters, &data)?;
    strings.add_bosses_events_families_and_rarities(codex.bosses.bosses, &data)?;
    strings.add_skills_and_statuses(codex.skills.skills, &data)?;
    strings.add_followers_and_events(codex.followers.followers, &data)?;

    Ok(strings)
}

/// Retrieve all missing searchable items from the codex.
pub fn missing_items_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexItems, Error> {
    fetch_loop(
        &guide
            .codex_fetch_item_list()?
            .into_iter()
            .filter(|entry| !db.items.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_item_with_locale(slug, locale),
        "CItems",
    )
    .map(|items| CodexItems { items })
}

/// Retrieve all missing searchable monsters from the codex.
pub fn missing_monsters_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexMonsters, Error> {
    fetch_loop(
        &guide
            .codex_fetch_monster_list()?
            .into_iter()
            .filter(|entry| !db.monsters.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_monster_with_locale(slug, locale),
        "CMnstrs",
    )
    .map(|monsters| CodexMonsters { monsters })
}

/// Retrieve all missing searchable bosses from the codex.
pub fn missing_bosses_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexBosses, Error> {
    fetch_loop(
        &guide
            .codex_fetch_boss_list()?
            .into_iter()
            .filter(|entry| !db.bosses.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_boss_with_locale(slug, locale),
        "CBosses",
    )
    .map(|bosses| CodexBosses { bosses })
}

/// Retrieve all missing searchable raids from the codex.
pub fn missing_raids_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexRaids, Error> {
    fetch_loop(
        &guide
            .codex_fetch_raid_list()?
            .into_iter()
            .filter(|entry| !db.raids.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_raid_with_locale(slug, locale),
        "CRaids",
    )
    .map(|raids| CodexRaids { raids })
}

/// Retrieve all missing searchable skills from the codex.
pub fn missing_skills_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexSkills, Error> {
    fetch_loop(
        &guide
            .codex_fetch_skill_list()?
            .into_iter()
            .filter(|entry| !db.skills.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_skill_with_locale(slug, locale),
        "CSkills",
    )
    .map(|skills| CodexSkills { skills })
}

/// Retrieve all missing searchable followers from the codex.
pub fn missing_followers_translations(
    guide: &OrnaAdminGuide,
    db: &LocaleStrings,
    locale: &str,
) -> Result<CodexFollowers, Error> {
    fetch_loop(
        &guide
            .codex_fetch_follower_list()?
            .into_iter()
            .filter(|entry| !db.followers.contains_key(entry.slug()))
            .collect_vec(),
        |slug| guide.codex_fetch_follower_with_locale(slug, locale),
        "CFollwrs",
    )
    .map(|followers| CodexFollowers { followers })
}

/// Retrieve all missing translations from the already-known locales in `locale_db`.
/// Returns a new instance of a db, that may be merged with `locale_db` if needed.
pub fn missing_translations(
    guide: &OrnaAdminGuide,
    data: &OrnaData,
    locale_db: &LocaleDB,
) -> Result<LocaleDB, Error> {
    let mut ret = LocaleDB::default();

    for (locale, db) in locale_db.locales.iter() {
        println!("Fetching missing translations for locale {}", locale);
        let mut strings = LocaleStrings::default();

        let items = missing_items_translations(guide, db, locale)?;
        let monsters = missing_monsters_translations(guide, db, locale)?;
        let bosses = missing_bosses_translations(guide, db, locale)?;
        let raids = missing_raids_translations(guide, db, locale)?;
        let skills = missing_skills_translations(guide, db, locale)?;
        let followers = missing_followers_translations(guide, db, locale)?;

        strings.add_items(items.items);
        strings.add_monsters_events_families_and_rarities(monsters.monsters, data)?;
        strings.add_bosses_events_families_and_rarities(bosses.bosses, data)?;
        strings.add_raids_and_events(raids.raids, data)?;
        strings.add_skills_and_statuses(skills.skills, data)?;
        strings.add_followers_and_events(followers.followers, data)?;
        ret.locales.insert(locale.clone(), strings);
    }

    Ok(ret)
}
