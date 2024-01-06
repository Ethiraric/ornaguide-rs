use std::{
    fs::File,
    io::{BufReader, Write},
};

use crate::{
    error::{Error, Kind},
    guide::Static,
    monsters::admin::AdminMonster,
};

mod codex_data;
mod codex_generic_monster;
mod guide_data;

#[allow(clippy::module_name_repetitions)]
pub use codex_data::CodexData;
pub use codex_generic_monster::CodexGenericMonster;
#[allow(clippy::module_name_repetitions)]
pub use guide_data::GuideData;
use serde::Serialize;

/// Aggregate for both the codex and the guide data.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default, PartialEq)]
pub struct OrnaData {
    /// Data from the codex.
    pub codex: CodexData,
    /// Data from the guide.
    pub guide: GuideData,
}

impl OrnaData {
    /// Load data from a set of json files located in the given directory.
    ///
    /// # Errors
    /// Errors on I/O error, parsing error or if a file is missing.
    pub fn load_from(directory: &str) -> Result<Self, Error> {
        Ok(OrnaData {
            codex: CodexData {
                items: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_items.json"
                ))?))?,
                raids: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_raids.json"
                ))?))?,
                monsters: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_monsters.json"
                ))?))?,
                bosses: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_bosses.json"
                ))?))?,
                skills: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_skills.json"
                ))?))?,
                followers: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/codex_followers.json"
                ))?))?,
            },
            guide: GuideData {
                items: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/guide_items.json"
                ))?))?,
                monsters: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/guide_monsters.json"
                ))?))?,
                skills: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/guide_skills.json"
                ))?))?,
                pets: serde_json::from_reader(BufReader::new(File::open(format!(
                    "{directory}/guide_pets.json"
                ))?))?,
                static_: Static {
                    spawns: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_spawns.json"
                    ))?))?,
                    elements: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_elements.json"
                    ))?))?,
                    item_types: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_item_types.json"
                    ))?))?,
                    equipped_bys: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_equipped_bys.json"
                    ))?))?,
                    status_effects: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_status_effects.json"
                    ))?))?,
                    item_categories: serde_json::from_reader(BufReader::new(File::open(
                        format!("{directory}/guide_item_categories.json"),
                    )?))?,
                    monster_families: serde_json::from_reader(BufReader::new(File::open(
                        format!("{directory}/guide_monster_families.json"),
                    )?))?,
                    skill_types: serde_json::from_reader(BufReader::new(File::open(format!(
                        "{directory}/guide_skill_types.json"
                    ))?))?,
                },
            },
        })
    }

    /// Save data to a set of json files in the given writer.
    ///
    /// The generic functio is given the directory in which to save, the name of the file, and a
    /// function to call back with a writer.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to_generic<Writer>(&self, dir: &str, mut writer: Writer) -> Result<(), Error>
    where
        Writer: FnMut(&str, &dyn Fn(&mut dyn Write) -> Result<(), Error>) -> Result<(), Error>,
    {
        // Codex jsons
        save(dir, "codex_items.json", &mut writer, &self.codex.items)?;
        save(dir, "codex_raids.json", &mut writer, &self.codex.raids)?;
        save(
            dir,
            "codex_monsters.json",
            &mut writer,
            &self.codex.monsters,
        )?;
        save(dir, "codex_bosses.json", &mut writer, &self.codex.bosses)?;
        save(dir, "codex_skills.json", &mut writer, &self.codex.skills)?;
        save(
            dir,
            "codex_followers.json",
            &mut writer,
            &self.codex.followers,
        )?;

        // Guide jsons
        save(dir, "guide_items.json", &mut writer, &self.guide.items)?;
        save(
            dir,
            "guide_monsters.json",
            &mut writer,
            &self.guide.monsters,
        )?;
        save(dir, "guide_skills.json", &mut writer, &self.guide.skills)?;
        save(dir, "guide_pets.json", &mut writer, &self.guide.pets)?;

        let static_ = &self.guide.static_;
        save(
            dir,
            "guide_spawns.json",
            &mut writer,
            &self.guide.static_.spawns,
        )?;
        save(dir, "guide_elements.json", &mut writer, &static_.elements)?;
        save(
            dir,
            "guide_item_types.json",
            &mut writer,
            &static_.item_types,
        )?;
        save(
            dir,
            "guide_equipped_bys.json",
            &mut writer,
            &static_.equipped_bys,
        )?;
        save(
            dir,
            "guide_status_effects.json",
            &mut writer,
            &static_.status_effects,
        )?;
        save(
            dir,
            "guide_item_categories.json",
            &mut writer,
            &static_.item_categories,
        )?;
        save(
            dir,
            "guide_monster_families.json",
            &mut writer,
            &static_.monster_families,
        )?;
        save(
            dir,
            "guide_skill_types.json",
            &mut writer,
            &static_.skill_types,
        )?;
        Ok(())
    }

    /// Save data to a set of json files in the given directory.
    ///
    /// # Errors
    /// Errors on I/O error.
    pub fn save_to(&self, directory: &str) -> Result<(), Error> {
        self.save_to_generic(directory, |path, callback| -> Result<(), Error> {
            let mut file = File::create(path)?;
            callback(&mut file)
        })
    }

    /// Find which monster/boss/raid in the codex corresponds to the given admin monster.
    ///
    /// # Errors
    /// Errors if the monster cannot be found.
    pub fn find_generic_codex_monster_from_admin_monster<'a>(
        &'a self,
        admin_monster: &AdminMonster,
    ) -> Result<CodexGenericMonster<'a>, Error> {
        if admin_monster.codex_uri.is_empty() {
            return Err(Kind::Misc(format!(
                "Empty codex_uri for admin monster '{}'",
                admin_monster.name
            ))
            .into());
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
                    Kind::Misc(format!(
                        "No codex monster match for admin monster {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                    .into()
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
                    Kind::Misc(format!(
                        "No codex monster match for admin boss {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                    .into()
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
                    Kind::Misc(format!(
                        "No codex monster match for admin raid {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                    .into()
                })
        }
    }
}

/// Save a structure into a file in a directory.
///
/// # Errors
/// Errors on I/O error.
fn save<Writer, T>(directory: &str, filename: &str, mut writer: Writer, t: T) -> Result<(), Error>
where
    Writer: FnMut(&str, &dyn Fn(&mut dyn Write) -> Result<(), Error>) -> Result<(), Error>,
    T: Serialize,
{
    writer(&format!("{directory}/{filename}"), &|out| {
        serde_json::to_writer_pretty(out, &t)
            .map_err(Kind::from)
            .map_err(Error::from)
    })
}
