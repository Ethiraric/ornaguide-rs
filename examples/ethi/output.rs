use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use ornaguide_rs::{
    codex::Codex,
    error::Error,
    guide::{AdminGuide, CachedGuide, OrnaAdminGuide, Static},
};

use crate::{
    codex::fetch::{CodexBosses, CodexItems, CodexMonsters, CodexRaids},
    guide::fetch::{AdminItems, AdminMonsters, AdminSkills},
};

pub fn generate_output_jsons(guide: &OrnaAdminGuide) -> Result<(), Error> {
    // Codex jsons
    let items = crate::codex::fetch::items(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_items.json")?),
        &items,
    )?;

    let raids = crate::codex::fetch::raids(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_raids.json")?),
        &raids,
    )?;

    let monsters = crate::codex::fetch::monsters(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_monsters.json")?),
        &monsters,
    )?;

    let bosses = crate::codex::fetch::bosses(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/codex_bosses.json")?),
        &bosses,
    )?;

    // Guide jsons
    let items = crate::guide::fetch::items(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_items.json")?),
        &items,
    )?;

    let monsters = crate::guide::fetch::monsters(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_monsters.json")?),
        &monsters,
    )?;

    let skills = crate::guide::fetch::skills(guide)?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_skills.json")?),
        &skills,
    )?;

    let rsc = guide.admin_retrieve_static_resources()?;

    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_spawns.json")?),
        &rsc.spawns,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_elements.json")?),
        &rsc.elements,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_item_types.json")?),
        &rsc.item_types,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_equipped_bys.json")?),
        &rsc.equipped_bys,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_status_effects.json")?),
        &rsc.status_effects,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_item_categories.json")?),
        &rsc.item_categories,
    )?;
    serde_json::to_writer_pretty(
        BufWriter::new(File::create("output/guide_monster_families.json")?),
        &rsc.monster_families,
    )?;

    Ok(())
}

pub struct CodexData {
    pub items: CodexItems,
    pub raids: CodexRaids,
    pub monsters: CodexMonsters,
    pub bosses: CodexBosses,
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
}
