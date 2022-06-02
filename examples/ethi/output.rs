use std::{fs::File, io::BufWriter};

use ornaguide_rs::{
    codex::Codex,
    error::Error,
    guide::{AdminGuide, CachedGuide, OrnaAdminGuide},
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
