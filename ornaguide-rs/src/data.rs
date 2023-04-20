use std::{
    fs::File,
    io::{BufReader, Write},
};

use crate::{
    error::{Error, ErrorKind},
    guide::Static,
    monsters::admin::AdminMonster,
};

mod codex_data;
mod codex_generic_monster;
mod guide_data;

pub use codex_data::CodexData;
pub use codex_generic_monster::CodexGenericMonster;
pub use guide_data::GuideData;
use serde::Serialize;

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
            return Err(ErrorKind::Misc(format!(
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
                    ErrorKind::Misc(format!(
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
                    ErrorKind::Misc(format!(
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
                    ErrorKind::Misc(format!(
                        "No codex monster match for admin raid {} (#{})",
                        admin_monster.name, admin_monster.id
                    ))
                    .into()
                })
        }
    }
}

/// Save a structure into a file in a directory.
fn save<Writer, T>(directory: &str, filename: &str, mut writer: Writer, t: T) -> Result<(), Error>
where
    Writer: FnMut(&str, &dyn Fn(&mut dyn Write) -> Result<(), Error>) -> Result<(), Error>,
    T: Serialize,
{
    writer(&format!("{}/{}", directory, filename), &|out| {
        serde_json::to_writer_pretty(out, &t)
            .map_err(ErrorKind::from)
            .map_err(Error::from)
    })
}
