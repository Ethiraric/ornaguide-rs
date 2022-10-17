use itertools::Itertools;
use ornaguide_rs::{
    codex::Codex,
    data::{CodexData, GuideData, OrnaData},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};

use crate::misc::bar;

/// Add unlisted monsters / bosses / raids to the data.
/// Walks through item drops and lists monsters in those drops we couldn't find.
/// Also adds event monsters that have no drops.
/// Modifies `data` in-place.
fn add_unlisted_monsters(guide: &OrnaAdminGuide, data: &mut CodexData) -> Result<(), Error> {
    // Monsters that are not necessarily listed (i.e.: belong to an event) and that have no drops.
    // These won't show up when listing through item drops.
    let unlisted_without_drops = &["/codex/monsters/elite-balor-flame/".to_string()];

    let uris = data
        .items
        .items
        .iter()
        // List all drops from all items.
        .flat_map(|item| item.dropped_by.iter())
        // Keep only the URI of those those we can't find a codex monster for.
        .filter(|dropped_by| {
            data.find_generic_monster_from_uri(&dropped_by.uri)
                .is_none()
        })
        .map(|dropped_by| &dropped_by.uri)
        // Add event monsters we don't have that do not drop any item.
        .chain(
            unlisted_without_drops
                .iter()
                .filter(|uri| data.find_generic_monster_from_uri(uri).is_none()),
        )
        // Remove duplicates.
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let bar = bar(uris.len() as u64);
    for uri in uris {
        // Strip `/codex/` and trailing slash from the uri.
        let uri = uri[7..].trim_end_matches('/');
        bar.set_message(uri.to_string());
        if let Some(pos) = uri.find('/') {
            let kind = &uri[0..pos];
            let slug = &uri[pos + 1..];
            let result = || -> Result<(), Error> {
                match kind {
                    "monsters" => {
                        data.monsters
                            .monsters
                            .push(guide.codex_fetch_monster(slug)?);
                    }
                    "bosses" => {
                        data.bosses.bosses.push(guide.codex_fetch_boss(slug)?);
                    }
                    "raids" => {
                        data.raids.raids.push(guide.codex_fetch_raid(slug)?);
                    }
                    _ => {
                        println!("Unknown monster kind for URI {}", uri);
                    }
                }
                Ok(())
            }();
            // Ignore 404s.
            match result {
                Err(Error::ResponseError(_, _, 404, _)) => {}
                Err(x) => return Err(x),
                _ => {}
            }
            bar.inc(1);
        } else {
            println!("Failed to parse monster for URI {}", uri);
        }
    }
    bar.finish_with_message("CUnlstM fetched");
    Ok(())
}

/// Add unlisted followers to the data.
/// Modifies `data` in-place.
fn add_event_followers(guide: &OrnaAdminGuide, data: &mut CodexData) -> Result<(), Error> {
    // List of event pet slugs. Those may or may not appear in the follower list, depending on the
    // time of the year.
    let event_pets = &[
        "age-old-mimic",
        "alfar",
        "alfar-mage",
        "amadan",
        "apollyons-apprentice",
        "apollyons-apprentice-91d096c2",
        "apollyons-apprentice-d370c676",
        "apollyons-apprentice-e9f91df6",
        "apollyons-pupil",
        "apollyons-pupil-9d70a08e",
        "apollyons-pupil-dde6d90b",
        "apollyons-pupil-e453d6d9",
        "archimedes",
        "arisen-naggeneen",
        "ashen-phoenix",
        "balor-flame",
        "balor-worm",
        "carman",
        "castor",
        "cerus",
        "cruel-banshee",
        "ebon-scruug",
        "fey-chimera",
        "fey-dragon",
        "fey-gazer",
        "fey-yeti",
        "glatisant",
        "great-pegasus",
        "gullinkambi",
        "gwyllgi",
        "hengreon",
        "kerberos",
        "kin-of-kerberos",
        "lindworm",
        "llamrai",
        "naggeneen",
        "phoenix",
        "pollux",
        "pumpkinhead",
        "raging-cerus",
        "scary-skeleton",
        "scruug",
        "spooky-ghost",
        "steward-cactus",
        "steward-dragon",
        "steward-gazer",
        "steward-golem",
        "steward-wolf",
        "surtrs-flame",
        "surtrs-mighty-flame",
        "the-mightiest-mimic",
        "untamed-cerus",
        "very-scary-skeleton",
    ];

    let bar = bar(event_pets.len() as u64);
    for slug in event_pets {
        bar.set_message(slug.to_string());
        // Don't include a follower twice.
        if !data
            .followers
            .followers
            .iter()
            .any(|follower| &&*follower.slug == slug)
        {
            match guide.codex_fetch_follower(slug) {
                Ok(follower) => data.followers.followers.push(follower),
                Err(Error::ResponseError(_, _, 404, _)) => {}
                Err(x) => return Err(x),
            }
        }
        bar.inc(1);
    }
    bar.finish_with_message("CEvtFlw fetched");
    Ok(())
}

/// Refresh all output jsons. Fetches all codex and guide entities.
/// Adds unlisted event monsters, bosses, raids and followers.
pub fn refresh(guide: &OrnaAdminGuide) -> Result<OrnaData, Error> {
    let mut data = OrnaData {
        codex: CodexData {
            items: crate::codex::fetch::items(guide)?,
            raids: crate::codex::fetch::raids(guide)?,
            monsters: crate::codex::fetch::monsters(guide)?,
            bosses: crate::codex::fetch::bosses(guide)?,
            skills: crate::codex::fetch::skills(guide)?,
            followers: crate::codex::fetch::followers(guide)?,
        },
        guide: GuideData {
            items: crate::guide::fetch::items(guide)?,
            monsters: crate::guide::fetch::monsters(guide)?,
            skills: crate::guide::fetch::skills(guide)?,
            pets: crate::guide::fetch::pets(guide)?,
            static_: guide.admin_retrieve_static_resources()?,
        },
    };
    add_unlisted_monsters(guide, &mut data.codex)?;
    add_event_followers(guide, &mut data.codex)?;

    data.save_to("output")?;

    Ok(data)
}

/// Refresh all guide output jsons. Fetches all guide entities.
pub fn refresh_guide(guide: &OrnaAdminGuide, codex_data: CodexData) -> Result<OrnaData, Error> {
    let data = OrnaData {
        codex: codex_data,
        guide: GuideData {
            items: crate::guide::fetch::items(guide)?,
            monsters: crate::guide::fetch::monsters(guide)?,
            skills: crate::guide::fetch::skills(guide)?,
            pets: crate::guide::fetch::pets(guide)?,
            static_: guide.admin_retrieve_static_resources()?,
        },
    };

    data.save_to("output")?;

    Ok(data)
}

/// Refresh the guide's static resources.
pub fn refresh_guide_static(guide: &OrnaAdminGuide, data: OrnaData) -> Result<OrnaData, Error> {
    let data = OrnaData {
        codex: data.codex,
        guide: GuideData {
            items: data.guide.items,
            monsters: data.guide.monsters,
            skills: data.guide.skills,
            pets: data.guide.pets,
            static_: guide.admin_retrieve_static_resources()?,
        },
    };

    data.save_to("output")?;

    Ok(data)
}

/// Refresh all codex output jsons. Fetches all codex entities.
pub fn refresh_codex(guide: &OrnaAdminGuide, guide_data: GuideData) -> Result<OrnaData, Error> {
    let mut data = OrnaData {
        codex: CodexData {
            items: crate::codex::fetch::items(guide)?,
            raids: crate::codex::fetch::raids(guide)?,
            monsters: crate::codex::fetch::monsters(guide)?,
            bosses: crate::codex::fetch::bosses(guide)?,
            skills: crate::codex::fetch::skills(guide)?,
            followers: crate::codex::fetch::followers(guide)?,
        },
        guide: guide_data,
    };
    add_unlisted_monsters(guide, &mut data.codex)?;
    add_event_followers(guide, &mut data.codex)?;

    data.save_to("output")?;

    Ok(data)
}
