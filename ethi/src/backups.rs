use std::path::{Path, PathBuf};

use itertools::Itertools;
use ornaguide_rs::{codex::translation::LocaleDB, data::OrnaData, error::Error};

use crate::backups::data_merger::DataMerger;

pub mod data_merger;
pub(crate) mod io;

/// A structure for backups.
/// Contains all Orna-related data and the translation databases.
#[derive(Default, PartialEq)]
pub struct Backup {
    /// The Orna-related data.
    pub data: OrnaData,
    /// The translation database.
    pub locales: LocaleDB,
    /// The manual translation database.
    pub manual_locales: LocaleDB,
}

impl Backup {
    /// Save the backup to a bzipped archive in `path` and with name `name`. To the name will be
    /// appended a timestamp and the `.tar.bz2` extension.
    pub fn save_to<P: AsRef<Path>>(&self, path: P, name: &str) -> Result<(), Error> {
        io::save_to(self, path, name)
    }

    /// Load the backup from the archive at the given path.
    pub fn load_from(archive_path: &Path) -> Result<Backup, Error> {
        io::load_from(archive_path)
    }
}

/// Iterate through all backup archives we can extract.
fn iter_backups<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = (PathBuf, Backup)>, Error> {
    // Walk through all backup archives.
    Ok(std::fs::read_dir(path)?
        // Filter out directory entries we can't read.
        .filter_map(|entry| entry.ok())
        // Filter out directories.
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        // Sort them. The names are chronological, so it orders them oldest first.
        .sorted_by_key(|entry| entry.path())
        // Try to open them. Ignore those we fail to open.
        // Oldest archives have a different format and may not be loadable.
        .filter_map(|entry| match Backup::load_from(&entry.path()) {
            Ok(backup) => Some((entry.path(), backup)),
            Err(x) => {
                println!("Failed to load {:?}: {}", entry.path(), x);
                None
            }
        }))
}

/// Remove duplicate backups in the specified directory.
/// If no fetch is performed during the day, 2 automatic backup files may end up containing the
/// same data. One of them can be removed.
/// The comparison is performed on actual entity data, rather than metadata. The json a/c/mtimes in
/// the tar file or those of the tar file itself do not matter.
pub fn prune<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let backups = iter_backups(path)?.collect_vec();

    for i in 0..backups.len() - 1 {
        let old = &backups[i];
        let new = &backups[i + 1];
        if old.1.eq(&new.1) {
            std::fs::remove_file(&new.0)?;
        }
    }
    Ok(())
}

/// Merge all backups into one single backup.
/// Codex entries are matched by their slugs. Guide entries are matched by their ids.
/// Archives are sorted by their names. Entries from latter archives take precedence over entries
/// from former archives.
pub fn merge<P: AsRef<Path>>(backups_path: P, output_path: P) -> Result<(), Error> {
    let (data_merger, locales, manual_locales) = iter_backups(backups_path)?
        // Merge everything into one big `OrnaData` and two big `LocaleDB`s.
        .fold(
            (
                DataMerger::default(),
                LocaleDB::default(),
                LocaleDB::default(),
            ),
            |(mut data_merger, mut locale_db, mut manual_locale_db), (_, backup)| {
                data_merger.merge_with(backup.data);
                locale_db.merge_with(backup.locales);
                manual_locale_db.merge_with(backup.manual_locales);
                (data_merger, locale_db, manual_locale_db)
            },
        );
    let data = data_merger.into_orna_data();
    let backup = Backup {
        data,
        locales,
        manual_locales,
    };
    backup.save_to(output_path, "merge")
}
