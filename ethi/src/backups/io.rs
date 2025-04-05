use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use bzip2::{read::BzDecoder, write::BzEncoder, Compression};
use ornaguide_rs::{
    codex::translation::{LocaleDB, LocaleStrings},
    data::OrnaData,
    error::{Error, Kind},
};
use tar::{Archive, Builder, EntryType, Header};

use crate::{backups::Backup, misc::json_read};

/// See [`crate::backups::Backup::save_to`].
#[allow(clippy::similar_names)]
pub(crate) fn save_to<P: AsRef<Path>>(backup: &Backup, path: P, name: &str) -> Result<(), Error> {
    // Create archive path, from path, name and timestamp.
    // Keep the archive basename, as it will be the root directory from inside the archive.
    let now = chrono::Local::now();
    let archive_basename = format!("{}-{}", name, now.format("%FT%H-%M"));
    let mut archive_path = path.as_ref().to_path_buf();
    archive_path.push(format!("{archive_basename}.tar.bz2"));

    // Open the archive.
    let mut archive = Builder::new(BzEncoder::new(
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(archive_path)?,
        Compression::best(),
    ));

    // Metadata to add to every entry in the archive.
    let uid = nix::unistd::getuid();
    let gid = nix::unistd::getgid();
    let username = nix::unistd::User::from_uid(uid).unwrap().unwrap().name;
    let groupname = nix::unistd::Group::from_gid(gid).unwrap().unwrap().name;
    let mtime = now.timestamp() as u64;

    // Create a header for an entry in an archive.
    // Does not set the size of the entry, as we don't know it yet. Can't set the checksum either.
    let new_header = |path: &str, is_folder| {
        let mut header = Header::new_gnu();
        header.set_uid(uid.as_raw() as u64);
        header.set_gid(gid.as_raw() as u64);
        header.set_username(&username).unwrap();
        header.set_groupname(&groupname).unwrap();
        header.set_mode(if is_folder { 0o755 } else { 0o644 });
        header.set_mtime(mtime);
        header.set_entry_type(if is_folder {
            EntryType::Directory
        } else {
            EntryType::Regular
        });
        header.set_path(path).unwrap();
        header
    };

    // Create root folder.
    let mut header = new_header(&archive_basename, true);
    header.set_size(0);
    header.set_cksum();
    archive.append(&header, &*Vec::<u8>::new()).unwrap();

    // Create translation folders.
    let locale_dir = format!("{archive_basename}/i18n");
    let manual_locale_dir = format!("{archive_basename}/i18n/manual");
    let mut header = new_header(&locale_dir, true);
    header.set_size(0);
    header.set_cksum();
    archive.append(&header, &*Vec::<u8>::new()).unwrap();
    let mut header = new_header(&manual_locale_dir, true);
    header.set_size(0);
    header.set_cksum();
    archive.append(&header, &*Vec::<u8>::new()).unwrap();

    // Create a callback to give the data so it writes into the archive.
    let mut writer_callback =
        |path: &str, callback: &dyn Fn(&mut dyn Write) -> Result<(), Error>| -> Result<(), Error> {
            let mut buffer: Vec<u8> = vec![];
            callback(&mut buffer)?;
            let mut header = new_header(path, false);
            header.set_size(buffer.len() as u64);
            header.set_cksum();
            archive.append(&header, Cursor::new(buffer)).unwrap();
            Ok(())
        };

    // Create Orna Data files.
    backup
        .data
        .save_to_generic(&archive_basename, &mut writer_callback)?;

    // Create translation files.
    backup
        .locales
        .save_to_generic(&locale_dir, &mut writer_callback)?;
    backup
        .manual_locales
        .save_to_generic(&manual_locale_dir, &mut writer_callback)?;

    Ok(())
}

/// See [`crate::backups::Backup::load_from`].
#[allow(clippy::too_many_lines)]
pub(crate) fn load_from<P: AsRef<Path>>(archive_path: P) -> Result<Backup, Error> {
    let archive_path: &Path = archive_path.as_ref();
    if !archive_path.to_string_lossy().ends_with(".tar.bz2") {
        return Err(Kind::Misc(format!("Invalid backup output file: {archive_path:?}")).into());
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
                    return Err(Kind::Misc(format!(
                        "Unexpected file in {archive_path:?}: {path:?}"
                    ))
                    .into());
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
            } else if path.starts_with("guide_api") {
                // Ok, skip this file.
            } else {
                return Err(
                    Kind::Misc(format!("Unexpected file in {archive_path:?}: {path:?}")).into(),
                );
            }
        }
    }

    Ok(Backup {
        data,
        locales,
        manual_locales,
    })
}

/// Load a `LocaleStrings` from a reader into the given `LocaleDB`.
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
        .ok_or_else(|| {
            Kind::Misc(format!("{fullpath}: lang file doesn't end in `.json`")).into_err()
        })?
        .to_string();
    db.locales.insert(lang, strings);
    Ok(())
}
