use std::{
    fs::File,
    io::{BufReader, Write},
};

use crate::{error::Error, guide::Static, monsters::admin::AdminMonster};

mod codex_data;
mod codex_generic_monster;
mod guide_data;

pub use codex_data::CodexData;
pub use codex_generic_monster::CodexGenericMonster;
pub use guide_data::GuideData;

/// Aggregate for both the codex and the guide data.
#[derive(Clone, Default, PartialEq)]
pub struct OrnaData {
    /// Data from the codex.
    pub codex: CodexData,
    /// Data from the guide.
    pub guide: GuideData,
}

impl OrnaData {
    /// Load data from a set of json files located in the given directory.
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
                followers: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/codex_followers.json",
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
                pets: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{}/guide_pets.json",
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
                    skill_types: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{}/guide_skill_types.json",
                        directory
                    ))?))?,
                },
            },
        })
    }

    pub fn save_to_generic<Writer>(&self, directory: &str, mut writer: Writer) -> Result<(), Error>
    where
        Writer: FnMut(&str, &dyn Fn(&mut dyn Write) -> Result<(), Error>) -> Result<(), Error>,
    {
        // Codex jsons
        writer(&format!("{}/codex_items.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.items).map_err(Error::from)
        })?;
        writer(&format!("{}/codex_raids.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.raids).map_err(Error::from)
        })?;
        writer(&format!("{}/codex_monsters.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.monsters).map_err(Error::from)
        })?;
        writer(&format!("{}/codex_bosses.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.bosses).map_err(Error::from)
        })?;
        writer(&format!("{}/codex_skills.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.skills).map_err(Error::from)
        })?;
        writer(&format!("{}/codex_followers.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.codex.followers).map_err(Error::from)
        })?;

        // Guide jsons
        writer(&format!("{}/guide_items.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.items).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_monsters.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.monsters).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_skills.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.skills).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_pets.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.pets).map_err(Error::from)
        })?;

        writer(&format!("{}/guide_spawns.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.static_.spawns).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_elements.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.static_.elements).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_item_types.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.static_.item_types).map_err(Error::from)
        })?;
        writer(&format!("{}/guide_equipped_bys.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.static_.equipped_bys).map_err(Error::from)
        })?;
        writer(
            &format!("{}/guide_status_effects.json", directory),
            &|out| {
                serde_json::to_writer_pretty(out, &self.guide.static_.status_effects)
                    .map_err(Error::from)
            },
        )?;
        writer(
            &format!("{}/guide_item_categories.json", directory),
            &|out| {
                serde_json::to_writer_pretty(out, &self.guide.static_.item_categories)
                    .map_err(Error::from)
            },
        )?;
        writer(
            &format!("{}/guide_monster_families.json", directory),
            &|out| {
                serde_json::to_writer_pretty(out, &self.guide.static_.monster_families)
                    .map_err(Error::from)
            },
        )?;
        writer(&format!("{}/guide_skill_types.json", directory), &|out| {
            serde_json::to_writer_pretty(out, &self.guide.static_.skill_types).map_err(Error::from)
        })?;
        Ok(())
    }

    /// Save data to a set of json files in the given directory.
    pub fn save_to(&self, directory: &str) -> Result<(), Error> {
        self.save_to_generic(directory, |path, callback| -> Result<(), Error> {
            let mut file = File::create(path)?;
            callback(&mut file)
        })
    }

    /// Find which monster/boss/raid in the codex corresponds to the given admin monster.
    pub fn find_generic_codex_monster_from_admin_monster<'a>(
        &'a self,
        admin_monster: &AdminMonster,
    ) -> Result<CodexGenericMonster<'a>, Error> {
        if admin_monster.codex_uri.is_empty() {
            return Err(Error::Misc(format!(
                "Empty codex_uri for admin monster '{}'",
                admin_monster.name
            )));
        }

        if admin_monster.is_regular_monster() {
            // Monster
            let slug = admin_monster.codex_uri["/codex/monsters/".len()..].trim_end_matches('/');
            self.codex
                .monsters
                .monsters
                .iter()
                .find(|codex_monster| codex_monster.slug == slug)
                .map(CodexGenericMonster::Monster)
                .ok_or_else(|| {
                    Error::Misc(format!(
                        "No codex monster match for admin monster {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                })
        } else if admin_monster.is_boss(&self.guide.static_.spawns) {
            // Boss
            let slug = admin_monster.codex_uri["/codex/bosses/".len()..].trim_end_matches('/');
            self.codex
                .bosses
                .bosses
                .iter()
                .find(|codex_boss| codex_boss.slug == slug)
                .map(CodexGenericMonster::Boss)
                .ok_or_else(|| {
                    Error::Misc(format!(
                        "No codex monster match for admin boss {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                })
        } else {
            // Raid
            let slug = admin_monster.codex_uri["/codex/raids/".len()..].trim_end_matches('/');
            self.codex
                .raids
                .raids
                .iter()
                .find(|codex_raid| codex_raid.slug == slug)
                .map(CodexGenericMonster::Raid)
                .ok_or_else(|| {
                    Error::Misc(format!(
                        "No codex monster match for admin raid {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                })
        }
    }
}
