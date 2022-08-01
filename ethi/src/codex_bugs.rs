use ornaguide_rs::{
    codex::{Codex, CodexBoss, CodexMonster, CodexRaid},
    data::OrnaData,
    error::Error,
    guide::OrnaAdminGuide,
};

use crate::guide_match::items::get_iter_element_statuses;

enum Status {
    Fixed,
    PartiallyFixed,
    NotFixed,
}

enum CodexGenericMonsterOwned {
    Monster(CodexMonster),
    Boss(CodexBoss),
    Raid(CodexRaid),
}

/// Get a generic monster from an URI.
fn get_generic_monster(
    guide: &OrnaAdminGuide,
    uri: &str,
) -> Result<CodexGenericMonsterOwned, Error> {
    let uri = uri
        .strip_prefix("/codex/")
        .unwrap()
        .strip_suffix('/')
        .unwrap();
    let pos = uri.find('/').unwrap();
    let kind = &uri[0..pos];
    let slug = &uri[pos + 1..];
    match kind {
        "monsters" => guide
            .codex_fetch_monster(slug)
            .map(CodexGenericMonsterOwned::Monster),
        "bosses" => guide
            .codex_fetch_boss(slug)
            .map(CodexGenericMonsterOwned::Boss),
        "raids" => guide
            .codex_fetch_raid(slug)
            .map(CodexGenericMonsterOwned::Raid),
        _ => panic!("Unknown kind: {} ({})", kind, slug),
    }
}

/// Check for weapons that have an element but are missing some of their elemental status effects.
fn weapons_missing_elemental_status_effects(
    data: &OrnaData,
    guide: &OrnaAdminGuide,
) -> Result<Status, Error> {
    let demeter = match guide.codex_fetch_item("arisen-demeters-staff") {
        Ok(x) => x,
        Err(msg) => {
            return Err(Error::Misc(format!(
                "Failed to retrieve arisen-demeters-staff: {}",
                msg
            )));
        }
    };

    if demeter.causes.iter().any(|effect| effect.name == "Rot") {
        for item in data.codex.items.items.iter().filter(|item| {
            if let Some(element) = item.stats.as_ref().and_then(|stats| stats.element.as_ref()) {
                get_iter_element_statuses(Some(element)).count() > 0
            } else {
                false
            }
        }) {
            // Ignore 404s.
            let item = match guide.codex_fetch_item(&item.slug) {
                Ok(x) => x,
                Err(Error::ResponseError(_, _, 404, _)) => continue,
                Err(x) => return Err(x),
            };

            // Check that all elemental statuses appear.
            if !get_iter_element_statuses(
                item.stats.as_ref().and_then(|stats| stats.element.as_ref()),
            )
            .all(|status| item.causes.iter().any(|cause| cause.name == status))
            {
                return Ok(Status::PartiallyFixed);
            }
        }
        Ok(Status::Fixed)
    } else {
        Ok(Status::NotFixed)
    }
}

/// Check for weapons that are missing the Bind or Bite skills.
fn monsters_missing_bind_bite(data: &OrnaData, guide: &OrnaAdminGuide) -> Result<Status, Error> {
    let gull = match guide.codex_fetch_monster("gull") {
        Ok(x) => x,
        Err(msg) => {
            return Err(Error::Misc(format!("Failed to retrieve gull: {}", msg)));
        }
    };

    if gull.abilities.iter().any(|skill| skill.name == "Bite") {
        let bind = data
            .guide
            .skills
            .skills
            .iter()
            .find(|skill| skill.name == "Bind")
            .ok_or_else(|| Error::Misc("Failed to find Bind".to_string()))?;
        let bite = data
            .guide
            .skills
            .skills
            .iter()
            .find(|skill| skill.name == "Bite")
            .ok_or_else(|| Error::Misc("Failed to find Bite".to_string()))?;
        for monster in data.guide.monsters.monsters.iter().filter(|monster| {
            monster
                .skills
                .iter()
                .any(|id| *id == bind.id || *id == bite.id)
        }) {
            let monster = guide.codex_fetch_monster(
                monster
                    .codex_uri
                    .strip_prefix("/codex/monsters/")
                    .and_then(|s| s.strip_suffix('/'))
                    .unwrap(),
            )?;
            if !monster
                .abilities
                .iter()
                .any(|ability| ability.name == "Bind" || ability.name == "Bite")
            {
                return Ok(Status::PartiallyFixed);
            }
        }
        Ok(Status::Fixed)
    } else {
        Ok(Status::NotFixed)
    }
}

/// Check for Yggdrasils' raid tags.
fn trees_missing_raid_tags(_: &OrnaData, guide: &OrnaAdminGuide) -> Result<Status, Error> {
    let ygg = guide.codex_fetch_raid("yggdrasil")?;
    let aygg = guide.codex_fetch_raid("arisen-yggdrasil")?;

    if ygg.tags.contains(&ornaguide_rs::codex::Tag::WorldRaid)
        && aygg.tags.contains(&ornaguide_rs::codex::Tag::WorldRaid)
    {
        Ok(Status::Fixed)
    } else {
        Ok(Status::NotFixed)
    }
}

/// Check for Swansong's "Blind" cause.
fn swansong_missing_blind(_: &OrnaData, guide: &OrnaAdminGuide) -> Result<Status, Error> {
    let swansong = guide.codex_fetch_item("swansong")?;

    if swansong.causes.iter().any(|cause| cause.name == "Blind") {
        Ok(Status::Fixed)
    } else {
        Ok(Status::NotFixed)
    }
}

/// Check for Kerberos monsters / raids / bosses missing their Rise of Kerberos event.
fn kerberos_missing_event(data: &OrnaData, guide: &OrnaAdminGuide) -> Result<Status, Error> {
    for monster in data
        .guide
        .monsters
        .monsters
        .iter()
        .filter(|monster| monster.spawns.contains(&18))
    // Rise of Kerberos
    {
        let events = match get_generic_monster(guide, &monster.codex_uri)? {
            CodexGenericMonsterOwned::Monster(x) => x.events,
            CodexGenericMonsterOwned::Boss(x) => x.events,
            CodexGenericMonsterOwned::Raid(x) => x.events,
        };
        if !events.contains(&"Rise of Kerberos".to_string()) {
            return Ok(Status::NotFixed);
        }
    }
    Ok(Status::Fixed)
}

/// Check whether a specific bug we found on the codex has been fixed and display the results.
fn do_check<F>(data: &OrnaData, guide: &OrnaAdminGuide, name: &str, checker: F) -> Result<(), Error>
where
    F: FnOnce(&OrnaData, &OrnaAdminGuide) -> Result<Status, Error>,
{
    match checker(data, guide) {
        Ok(Status::NotFixed) => println!("[\x1B[0;31m{:^15}\x1B[0m] {}", "Not fixed", name),
        Ok(Status::PartiallyFixed) => {
            println!("[\x1B[0;33m{:^15}\x1B[0m] {}", "Partially fixed", name)
        }
        Ok(Status::Fixed) => println!("[\x1B[0;32m{:^15}\x1B[0m] {}", "Fixed", name),
        Err(x) => println!("[\x1B[41;30m{:^15}\x1B[0m] {}: {}", "Errored", name, x),
    }
    Ok(())
}

/// Check whether the bugs we found on the codex have been fixed.
pub fn check(data: &OrnaData, guide: &OrnaAdminGuide) -> Result<(), Error> {
    do_check(
        data,
        guide,
        "Weapons missing elemental status effects",
        weapons_missing_elemental_status_effects,
    )?;
    do_check(
        data,
        guide,
        "Monsters missing skills Bind or Bite",
        monsters_missing_bind_bite,
    )?;
    do_check(
        data,
        guide,
        "Yggdrasils missing their raid tags",
        trees_missing_raid_tags,
    )?;
    do_check(
        data,
        guide,
        "Swansong missing its Blind cause",
        swansong_missing_blind,
    )?;
    do_check(
        data,
        guide,
        "Kerberos missing Rise of Kerberos event",
        kerberos_missing_event,
    )?;
    Ok(())
}
