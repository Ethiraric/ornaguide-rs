use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use bzip2::read::BzDecoder;
use ornaguide_rs::{
    codex::translation::{LocaleDB, LocaleStrings},
    data::OrnaData,
    error::Error,
};
use tar::{Archive, EntryType};

use crate::misc::json_read;

fn load_entry_to_translation_db<R>(
    db: &mut LocaleDB,
    fullpath: &str,
    filename: &str,
    reader: R,
) -> Result<(), Error>
where
    R: Read,
{
    let strings: LocaleStrings = json_read(reader, fullpath)?;
    let lang = filename
        .strip_suffix(".json")
        .ok_or_else(|| Error::Misc(format!("{}: lang file doesn't end in `.json`", fullpath)))?
        .to_string();
    db.locales.insert(lang, strings);
    Ok(())
}

pub fn load_from(archive_path: &Path) -> Result<(OrnaData, LocaleDB), Error> {
    if archive_path.ends_with(".tar.bz2") {
        return Err(Error::Misc(format!(
            "Invalid backup output file: {:?}",
            archive_path
        )));
    }

    // Take path, and remove the `.tar.bz2` extension.
    let file_base = archive_path.file_name().unwrap().to_string_lossy();
    let file_base = file_base.strip_suffix(".tar.bz2").unwrap();

    // Open archive.
    let mut archive = Archive::new(BzDecoder::new(File::open(archive_path)?));

    // Set our return value.
    let mut data = OrnaData::default();
    let mut locales = LocaleDB::default();
    let mut manual_locales = LocaleDB::default();

    for entry in archive.entries()? {
        let entry = entry?;
        // Ignore folders.
        if entry.header().entry_type() != EntryType::Regular {
            continue;
        }

        // Strip the root folder of the tar archives.
        // All files are `output-YYYY-MM-DD/file.json`, and we need the first component out.
        let base_path = PathBuf::from(String::from_utf8_lossy(&entry.path_bytes()).as_ref());
        let path = base_path.strip_prefix(file_base).unwrap();
        let base_path = base_path.to_string_lossy();
        let base_pathstr = base_path.as_ref();
        let pathstr = path.to_string_lossy();
        let pathstr = pathstr.as_ref();

        if path.components().count() == 1 {
            // TODO(ethiraric, 07/09/2022): Replace with diagnostics.
            match pathstr {
                "codex_bosses.json" => {
                    data.codex.bosses = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "codex_followers.json" => {
                    data.codex.followers = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "codex_items.json" => {
                    data.codex.items = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "codex_monsters.json" => {
                    data.codex.monsters = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "codex_raids.json" => {
                    data.codex.raids = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "codex_skills.json" => {
                    data.codex.skills = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_elements.json" => {
                    data.guide.static_.elements =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_equipped_bys.json" => {
                    data.guide.static_.equipped_bys =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_item_categories.json" => {
                    data.guide.static_.item_categories =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_items.json" => {
                    data.guide.items = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_item_types.json" => {
                    data.guide.static_.item_types =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_monster_families.json" => {
                    data.guide.static_.monster_families =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_monsters.json" => {
                    data.guide.monsters = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_pets.json" => {
                    data.guide.pets = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_skills.json" => {
                    data.guide.skills = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_skill_types.json" => {
                    data.guide.static_.skill_types =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_spawns.json" => {
                    data.guide.static_.spawns = json_read(entry, base_pathstr).unwrap_or_default();
                }
                "guide_status_effects.json" => {
                    data.guide.static_.status_effects =
                        json_read(entry, base_pathstr).unwrap_or_default();
                }
                _ => {
                    return Err(Error::Misc(format!(
                        "Unexpected file in {:?}: {:?}",
                        archive_path, path
                    )));
                }
            }
        } else {
            // We have a translation file.
            if let Ok(file_name) = path.strip_prefix("i18n") {
                // It's either a manual translation or a regular one.
                if let Ok(file_name) = file_name.strip_prefix("manual") {
                    // Add content to the manual database.
                    load_entry_to_translation_db(
                        &mut manual_locales,
                        path.to_string_lossy().as_ref(),
                        file_name.to_string_lossy().as_ref(),
                        entry,
                    )?;
                } else {
                    // Add content to the regular database.
                    load_entry_to_translation_db(
                        &mut locales,
                        path.to_string_lossy().as_ref(),
                        file_name.to_string_lossy().as_ref(),
                        entry,
                    )?;
                }
            } else {
                return Err(Error::Misc(format!(
                    "Unexpected file in {:?}: {:?}",
                    archive_path, path
                )));
            }
        }
    }

    // Merge both translation databases.
    // The manual one's entries take precedence over the regular ones.
    locales.merge_with(manual_locales);

    Ok((data, locales))
}
