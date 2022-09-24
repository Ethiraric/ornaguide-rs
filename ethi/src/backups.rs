use std::path::Path;

use itertools::Itertools;
use ornaguide_rs::error::Error;

pub mod data_merger;
pub mod output;

/// Remove duplicate backups in the specified directory.
/// If no fetch is performed during the day, 2 automatic backup files may end up containing the
/// same data. One of them can be removed.
/// The comparison is performed on actual entity data, rather than metadata. The json a/c/mtimes in
/// the tar file or those of the tar file itself do not matter.
pub fn prune<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let backups = {
        // Walk through all backup archives.
        // let (data, locales) =
        std::fs::read_dir(path)?
            // Filter out directory entries we can't read.
            .filter_map(|entry| entry.ok())
            // Filter out directories.
            .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
            // Sort them. The names are chronological, so it orders them oldest first.
            .sorted_by_key(|entry| entry.path())
            // Try to open them. Ignore those we fail to open.
            // Oldest archives have a different format and may not be loadable.
            .filter_map(|entry| match output::load_from(&entry.path()) {
                Ok((data, locales)) => Some((entry.path(), data, locales)),
                Err(x) => {
                    println!("Failed to load {:?}: {}", entry.path(), x);
                    None
                }
            })
            .collect_vec()
    };
    for i in 0..backups.len() - 1 {
        let old = &backups[i];
        let new = &backups[i + 1];
        if old.1.eq(&new.1) && old.2.eq(&new.2) {
            std::fs::remove_file(&new.0)?;
        }
    }
    Ok(())
}
