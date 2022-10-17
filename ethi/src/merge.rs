use itertools::Itertools;
use ornaguide_rs::{error::Error, guide::OrnaAdminGuide};

use crate::{backups::Backup, guide_match};

pub fn match_(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = std::fs::read_dir("merges")?
        // Filter out directory entries we can't read.
        .filter_map(|entry| entry.ok())
        // Filter out directories.
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        // Keep only merge files.
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.starts_with("merge-") && name.ends_with(".tar.bz2")
        })
        // Sort them. The names are chronological, so it orders them oldest first, which is why we
        // compare `b` to `a` and not the other way around.
        .sorted_by(|a, b| b.path().cmp(&a.path()))
        // Try to open them. Ignore those we fail to open.
        // Oldest archives have a different format and may not be loadable.
        .find_map(|entry| match Backup::load_from(&entry.path()) {
            Ok(backup) => Some((entry.path(), backup)),
            Err(x) => {
                println!("Failed to load {:?}: {}", entry.path(), x);
                None
            }
        })
        .ok_or_else(|| Error::Misc("Failed to find a merge file".to_string()))?;

    println!("Matching with merge archive {}", path.to_string_lossy());

    guide_match::all(&mut merge.data, fix, guide)
}
