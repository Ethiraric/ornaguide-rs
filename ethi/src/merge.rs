use std::path::PathBuf;

use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::{Error, Kind},
    guide::OrnaAdminGuide,
};

use crate::{
    backups::Backup,
    cli::{self, merge::Match},
    guide_match,
};

/// Retrieve the latest merge archive (both its path and contents).
fn get_merge_archive() -> Result<(PathBuf, Backup), Error> {
    std::fs::read_dir("data/merges")?
        // Filter out directory entries we can't read.
        .filter_map(std::result::Result::ok)
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
        .find_map(|entry| match Backup::load_from(entry.path()) {
            Ok(backup) => Some((entry.path(), backup)),
            Err(x) => {
                println!("Failed to load {:?}: {}", entry.path(), x);
                None
            }
        })
        .ok_or_else(|| Kind::Misc("Failed to find a merge file".to_string()).into())
}

pub fn match_(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::all(&mut merge.data, fix, guide)
}

pub fn match_status_effects(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::status_effects::perform(&mut merge.data, fix, guide)
}

pub fn match_skills(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::skills::perform(&mut merge.data, fix, guide)
}

pub fn match_items(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::items::perform(&mut merge.data, fix, guide)
}

pub fn match_monsters(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::monsters::perform(&mut merge.data, fix, guide)
}

pub fn match_pets(fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let (path, mut merge) = get_merge_archive()?;
    println!("Matching with merge archive {}", path.to_string_lossy());
    guide_match::pets::perform(&mut merge.data, fix, guide)
}

/// Execute a CLI subcommand on merges.
pub fn cli(command: cli::merge::Command, guide: &OrnaAdminGuide, _: OrnaData) -> Result<(), Error> {
    match command {
        cli::merge::Command::Match(cmd) => {
            let fix = cmd.fix;
            match cmd.c {
                Some(Match::Items) => match_items(fix, guide),
                Some(Match::Monsters) => match_monsters(fix, guide),
                Some(Match::Pets) => match_pets(fix, guide),
                Some(Match::Skills) => match_skills(fix, guide),
                Some(Match::StatusEffects) => match_status_effects(fix, guide),
                None => match_(fix, guide),
            }
        }
    }
}
