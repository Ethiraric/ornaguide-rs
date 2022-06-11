use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use itertools::Itertools;
use ornaguide_rs::{
    codex::{Codex, CodexBoss, CodexMonster, CodexRaid},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide, Static},
    monsters::admin::AdminMonster,
};

use crate::{
    codex::fetch::{CodexBosses, CodexItems, CodexMonsters, CodexRaids, CodexSkills},
    guide::fetch::{AdminItems, AdminMonsters, AdminSkills},
    misc::bar,
};

/// Add unlisted monsters / bosses / raids to the data.
/// Walks through item drops and lists monsters in those drops we couldn't find.
/// Modifies `data` in-place.
pub fn add_unlisted_monsters(guide: &OrnaAdminGuide, data: &mut OrnaData) -> Result<(), Error> {
    // Monsters that are not necessarily listed (i.e.: belong to an event) and that have no drops.
    // These won't show up when listing through item drops.
    let unlisted_without_drops = &["/codex/monsters/elite-balor-flame/".to_string()];

    let uris = data
        .codex
        .items
        .items
        .iter()
        // List all drops from all items.
        .flat_map(|item| item.dropped_by.iter())
        // Keep only the URI of those those we can't find a codex monster for.
        .filter(|dropped_by| {
            data.codex
                .find_generic_monster_from_uri(&dropped_by.uri)
                .is_none()
        })
        .map(|dropped_by| &dropped_by.uri)
        // Add event monsters we don't have that do not drop any item.
        .chain(
            unlisted_without_drops
                .iter()
                .filter(|uri| data.codex.find_generic_monster_from_uri(uri).is_none()),
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
            match kind {
                "monsters" => {
                    data.codex
                        .monsters
                        .monsters
                        .push(guide.codex_fetch_monster(slug)?);
                }
                "bosses" => {
                    data.codex.bosses.bosses.push(guide.codex_fetch_boss(slug)?);
                }
                "raids" => {
                    data.codex.raids.raids.push(guide.codex_fetch_raid(slug)?);
                }
                _ => {
                    println!("Unknown monster kind for URI {}", uri);
                }
            }
            bar.inc(1);
        } else {
            println!("Failed to parse monster for URI {}", uri);
        }
    }
    bar.finish_with_message("CUnlist fetched");
    Ok(())
}

pub fn refresh(guide: &OrnaAdminGuide) -> Result<(), Error> {
    let mut data = OrnaData {
        codex: CodexData {
            items: crate::codex::fetch::items(guide)?,
            raids: crate::codex::fetch::raids(guide)?,
            monsters: crate::codex::fetch::monsters(guide)?,
            bosses: crate::codex::fetch::bosses(guide)?,
            skills: crate::codex::fetch::skills(guide)?,
        },
        guide: GuideData {
            items: crate::guide::fetch::items(guide)?,
            monsters: crate::guide::fetch::monsters(guide)?,
            skills: crate::guide::fetch::skills(guide)?,
            static_: guide.admin_retrieve_static_resources()?,
        },
    };
    add_unlisted_monsters(guide, &mut data)?;

    // Codex jsons
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_items.json")?),
        &data.codex.items,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_raids.json")?),
        &data.codex.raids,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_monsters.json")?),
        &data.codex.monsters,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_bosses.json")?),
        &data.codex.bosses,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_skills.json")?),
        &data.codex.skills,
    )?;

    // Guide jsons
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_items.json")?),
        &data.guide.items,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_monsters.json")?),
        &data.guide.monsters,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_skills.json")?),
        &data.guide.skills,
    )?;

    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_spawns.json")?),
        &data.guide.static_.spawns,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_elements.json")?),
        &data.guide.static_.elements,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_item_types.json")?),
        &data.guide.static_.item_types,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_equipped_bys.json")?),
        &data.guide.static_.equipped_bys,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_status_effects.json")?),
        &data.guide.static_.status_effects,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_item_categories.json")?),
        &data.guide.static_.item_categories,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_monster_families.json")?),
        &data.guide.static_.monster_families,
    )?;

    Ok(())
}

pub struct CodexData {
    pub items: CodexItems,
    pub raids: CodexRaids,
    pub monsters: CodexMonsters,
    pub bosses: CodexBosses,
    pub skills: CodexSkills,
}

/// A monster from the codex, which may be a regular monster, a boss or a raid.
#[derive(Debug, Clone)]
pub enum CodexGenericMonster<'a> {
    /// A regular monster.
    Monster(&'a CodexMonster),
    /// A boss.
    Boss(&'a CodexBoss),
    /// A raid.
    Raid(&'a CodexRaid),
}

pub struct GuideData {
    pub items: AdminItems,
    pub monsters: AdminMonsters,
    pub skills: AdminSkills,
    pub static_: Static,
}

pub struct OrnaData {
    pub codex: CodexData,
    pub guide: GuideData,
}

impl<'a> CodexData {
    /// Find which monster/boss/raid corresponds to the given URI.
    /// The URI must be of the form `/codex/{kind}/{slug}/`.
    pub fn find_generic_monster_from_uri(&'a self, uri: &str) -> Option<CodexGenericMonster<'a>> {
        // Strip `/codex/` and trailing slash from the uri.
        let uri = uri[7..].trim_end_matches('/');
        if let Some(pos) = uri.find('/') {
            let kind = &uri[0..pos];
            let slug = &uri[pos + 1..];
            match kind {
                "monsters" => self
                    .monsters
                    .monsters
                    .iter()
                    .find(|monster| monster.slug == slug)
                    .map(CodexGenericMonster::Monster),
                "bosses" => self
                    .bosses
                    .bosses
                    .iter()
                    .find(|boss| boss.slug == slug)
                    .map(CodexGenericMonster::Boss),
                "raids" => self
                    .raids
                    .raids
                    .iter()
                    .find(|raid| raid.slug == slug)
                    .map(CodexGenericMonster::Raid),
                _ => {
                    println!("Unknown kind for generic monster {}", uri);
                    None
                }
            }
        } else {
            println!("Failed to find generic monster for {}", uri);
            None
        }
    }
}

impl CodexGenericMonster<'_> {
    // Returns the URI of the monster.
    // URI matches `/codex/{kind}/{slug}/`.
    pub fn uri(&self) -> String {
        match self {
            CodexGenericMonster::Monster(x) => format!("/codex/monsters/{}/", x.slug),
            CodexGenericMonster::Boss(x) => format!("/codex/bosses/{}/", x.slug),
            CodexGenericMonster::Raid(x) => format!("/codex/raids/{}/", x.slug),
        }
    }
}

impl GuideData {
    /// Find the admin item associated with the given codex monster.
    /// If there is no or multiple match, return an `Err`.
    pub fn find_match_for_codex_generic_monster<'a>(
        &'a self,
        needle: CodexGenericMonster,
    ) -> Result<&'a AdminMonster, Error> {
        let matches = match needle {
            CodexGenericMonster::Monster(codex) => self
                .monsters
                .monsters
                .iter()
                .filter(|admin| {
                    admin.is_regular_monster()
                        && admin.tier == codex.tier
                        && admin.image_name == codex.icon
                        && admin.codex_name() == codex.name
                })
                .collect::<Vec<_>>(),
            CodexGenericMonster::Boss(codex) => self
                .monsters
                .monsters
                .iter()
                .filter(|admin| {
                    admin.is_boss(&self.static_.spawns)
                        && admin.tier == codex.tier
                        && admin.image_name == codex.icon
                        && admin.codex_name() == codex.name
                })
                .collect::<Vec<_>>(),
            CodexGenericMonster::Raid(codex) => self
                .monsters
                .monsters
                .iter()
                .filter(|admin| {
                    admin.is_raid(&self.static_.spawns)
                        && admin.tier == codex.tier
                        && admin.image_name == codex.icon
                        && admin.codex_name() == codex.name
                })
                .collect::<Vec<_>>(),
        };

        if matches.is_empty() {
            Err(Error::Misc(format!(
                "No match for codex item '{}'",
                needle.uri()
            )))
        } else if matches.len() > 1 {
            Err(Error::Misc(format!(
                "Multiple matches for codex item '{}'",
                needle.uri()
            )))
        } else {
            Ok(matches[0])
        }
    }
}

impl OrnaData {
    pub fn load_from(directory: &str) -> Result<Self, Error> {
        Ok(OrnaData {
            codex: CodexData {
                items: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_items.json",
                    directory
                ))?))?,
                raids: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_raids.json",
                    directory
                ))?))?,
                monsters: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_monsters.json",
                    directory
                ))?))?,
                bosses: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_bosses.json",
                    directory
                ))?))?,
                skills: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_skills.json",
                    directory
                ))?))?,
            },
            guide: GuideData {
                items: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/guide_items.json",
                    directory
                ))?))?,
                monsters: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/guide_monsters.json",
                    directory
                ))?))?,
                skills: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/guide_skills.json",
                    directory
                ))?))?,
                static_: Static {
                    spawns: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_spawns.json",
                        directory
                    ))?))?,
                    elements: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_elements.json",
                        directory
                    ))?))?,
                    item_types: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_item_types.json",
                        directory
                    ))?))?,
                    equipped_bys: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_equipped_bys.json",
                        directory
                    ))?))?,
                    status_effects: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_status_effects.json",
                        directory
                    ))?))?,
                    item_categories: serde_json::from_reader(BufReader::new(File::open(
                        format!("{}/guide_item_categories.json", directory),
                    )?))?,
                    monster_families: serde_json::from_reader(BufReader::new(File::open(
                        format!("{}/guide_monster_families.json", directory),
                    )?))?,
                },
            },
        })
    }

    /// Find which monster/boss/raid in the codex corresponds to the given admin monster.
    pub fn find_generic_codex_monster_from_admin_monster<'a>(
        &'a self,
        admin_monster: &AdminMonster,
    ) -> Result<CodexGenericMonster<'a>, Error> {
        let monster_name = admin_monster.codex_name();
        // TODO(fsabourin, 06/06/2022): Factorize.
        if admin_monster.is_regular_monster() {
            // Monster
            let mut matches = self.codex.monsters.monsters.iter().filter(|codex_monster| {
                admin_monster.tier == codex_monster.tier
                    && admin_monster.image_name == codex_monster.icon
                    && monster_name == codex_monster.name
            });
            if let Some(matched) = matches.next() {
                if matches.next().is_some() {
                    Err(Error::Misc(format!(
                        "Multiple codex monster matches for admin monster {} (#{}, {})",
                        admin_monster.name, admin_monster.id, monster_name
                    )))
                } else {
                    Ok(CodexGenericMonster::Monster(matched))
                }
            } else {
                Err(Error::Misc(format!(
                    "No codex monster match for admin monster {} (#{}, {})",
                    admin_monster.name, admin_monster.id, monster_name
                )))
            }
        } else if admin_monster.is_boss(&self.guide.static_.spawns) {
            // Boss
            let mut matches = self.codex.bosses.bosses.iter().filter(|codex_boss| {
                admin_monster.tier == codex_boss.tier
                    && admin_monster.image_name == codex_boss.icon
                    && monster_name == codex_boss.name
            });
            if let Some(matched) = matches.next() {
                if matches.next().is_some() {
                    Err(Error::Misc(format!(
                        "Multiple codex monster matches for admin boss {} (#{}, {})",
                        admin_monster.name, admin_monster.id, monster_name
                    )))
                } else {
                    Ok(CodexGenericMonster::Boss(matched))
                }
            } else {
                Err(Error::Misc(format!(
                    "No codex monster match for admin boss {} (#{}, {})",
                    admin_monster.name, admin_monster.id, monster_name
                )))
            }
        } else {
            // Raid
            let mut matches = self.codex.raids.raids.iter().filter(|codex_raid| {
                admin_monster.tier == codex_raid.tier
                    && admin_monster.image_name == codex_raid.icon
                    && monster_name == codex_raid.name
            });
            if let Some(matched) = matches.next() {
                if matches.next().is_some() {
                    Err(Error::Misc(format!(
                        "Multiple codex monster matches for admin raid {} (#{}, {})",
                        admin_monster.name, admin_monster.id, monster_name
                    )))
                } else {
                    Ok(CodexGenericMonster::Raid(matched))
                }
            } else {
                Err(Error::Misc(format!(
                    "No codex monster match for admin raid {} (#{}, {})",
                    admin_monster.name, admin_monster.id, monster_name
                )))
            }
        }
    }
}
