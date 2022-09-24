use std::{
    fs::File,
    io::{BufReader, BufWriter},
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

    /// Save data to a set of json files in the given directory.
    pub fn save_to(&self, directory: &str) -> Result<(), Error> {
        // Codex jsons
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_items.json", directory))?),
            &self.codex.items,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_raids.json", directory))?),
            &self.codex.raids,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_monsters.json", directory))?),
            &self.codex.monsters,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_bosses.json", directory))?),
            &self.codex.bosses,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_skills.json", directory))?),
            &self.codex.skills,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/codex_followers.json", directory))?),
            &self.codex.followers,
        )?;

        // Guide jsons
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_items.json", directory))?),
            &self.guide.items,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_monsters.json", directory))?),
            &self.guide.monsters,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_skills.json", directory))?),
            &self.guide.skills,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_pets.json", directory))?),
            &self.guide.pets,
        )?;

        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_spawns.json", directory))?),
            &self.guide.static_.spawns,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!("{}/guide_elements.json", directory))?),
            &self.guide.static_.elements,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_item_types.json",
                directory
            ))?),
            &self.guide.static_.item_types,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_equipped_bys.json",
                directory
            ))?),
            &self.guide.static_.equipped_bys,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_status_effects.json",
                directory
            ))?),
            &self.guide.static_.status_effects,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_item_categories.json",
                directory
            ))?),
            &self.guide.static_.item_categories,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_monster_families.json",
                directory
            ))?),
            &self.guide.static_.monster_families,
        )?;
        serde_json::to_writer_pretty(
            BufWriter::new(File::create(format!(
                "{}/guide_skill_types.json",
                directory
            ))?),
            &self.guide.static_.skill_types,
        )?;
        Ok(())
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
