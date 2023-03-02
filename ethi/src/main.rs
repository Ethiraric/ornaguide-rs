use std::{path::PathBuf, time::Instant};

use crate::backups::Backup;
use clap::Parser;
#[allow(unused_imports)]
use itertools::Itertools;
use ornaguide_rs::{codex::translation::LocaleDB, data::OrnaData};
#[allow(unused_imports)]
use ornaguide_rs::{
    codex::{Codex, CodexItem},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};

mod backups;
mod cli;
mod codex;
mod codex_bugs;
mod config;
mod guide;
mod guide_html;
mod guide_match;
mod merge;
mod misc;
mod output;
mod translation;

/// Retrieve the latest merge archive (both its path and contents).
fn get_merge_archive() -> Result<(PathBuf, Backup), Error> {
    std::fs::read_dir("merges")?
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
        .find_map(|entry| match Backup::load_from(entry.path()) {
            Ok(backup) => Some((entry.path(), backup)),
            Err(x) => {
                println!("Failed to load {:?}: {}", entry.path(), x);
                None
            }
        })
        .ok_or_else(|| Error::Misc("Failed to find a merge file".to_string()))
}

#[allow(unused_variables, unused_mut)]
/// Danger zone. Where I test my code.
fn ethi(guide: &OrnaAdminGuide, mut data: OrnaData) -> Result<(), Error> {
    let fix = false;

    // let mut db = LocaleDB::load_from("output/i18n")?;
    // db.merge_with(LocaleDB::load_from("output/i18n/manual")?);

    let (path, archive) = get_merge_archive()?;
    println!("Found archive {}", path.display());

    // guide_match::all(&mut data, fix, guide)?;
    // guide_match::status_effects::perform(&mut data, fix, guide)?;
    // guide_match::skills::perform(&mut data, fix, guide)?;
    // guide_match::items::perform(&mut data, fix, guide)?;
    // guide_match::monsters::perform(&mut data, fix, guide)?;
    // guide_match::pets::perform(&mut data, fix, guide)?;

    Ok(())
}

fn main2() -> Result<(), Error> {
    let guide = config::with_config(|config| {
        OrnaAdminGuide::new_with_hosts(
            &config.ornaguide_cookie,
            config.ornaguide_host.clone(),
            config.playorna_host.clone(),
        )
    })?;
    let data = || OrnaData::load_from("output");
    let localedb = || LocaleDB::load_from("output/i18n");

    match cli::Cli::parse().command {
        Some(command) => match command {
            cli::Command::Backups(cmd) => backups::cli(cmd, &guide, data()?),
            cli::Command::Codex(cmd) => codex::cli(cmd, &guide, data()?),
            cli::Command::Json(cmd) => output::cli(cmd, &guide, data),
            cli::Command::Match(cmd) => guide_match::cli(cmd, &guide, data()?),
            cli::Command::Merge(cmd) => merge::cli(cmd, &guide, data()?),
            cli::Command::Translation(cmd) => translation::cli(cmd, &guide, data()?, localedb()?),
        },
        None => ethi(&guide, data()?),
    }

    // let args = std::env::args().collect::<Vec<_>>();
    // let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    // if args.len() == 1 {
    //     ethi(&guide, data()?)
    // } else {
    //     match args[1] {
    //         "translation" => translation::cli(&args[2..], &guide, data()?, localedb()?),
    //         subcommand => Err(Error::Misc(format!(
    //             "Invalid CLI subcommand: {}",
    //             subcommand
    //         ))),
    //     }
    // }
}

fn main() {
    let begin = Instant::now();
    match main2() {
        Ok(_) => println!("OK"),
        Err(err) => eprintln!("Error: {}", err),
    }
    let end = Instant::now();
    let elapsed = end.duration_since(begin);
    println!("Executed in {}ms", elapsed.as_millis());
}
