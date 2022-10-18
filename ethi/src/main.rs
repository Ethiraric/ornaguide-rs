use std::time::Instant;

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
mod codex;
mod codex_bugs;
mod config;
mod ethiraric;
mod guide;
mod guide_html;
mod guide_match;
mod merge;
mod misc;
mod output;
mod ratakor;
mod sirscor;
mod thecraigger;
mod translation;

#[allow(unused_variables, unused_mut)]
/// Danger zone. Where I test my code.
fn ethi(guide: &OrnaAdminGuide, mut data: OrnaData) -> Result<(), Error> {
    let fix = false;

    let mut db = LocaleDB::load_from("output/i18n")?;
    db.merge_with(LocaleDB::load_from("output/i18n/manual")?);

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

    let args = std::env::args().collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    if args.len() == 1 {
        ethi(&guide, data()?)
    } else {
        match args[1] {
            "json" => output::cli(&args[2..], &guide, data),
            "match" => guide_match::cli(&args[2..], &guide, data()?),
            "sirscor" => sirscor::cli(&args[2..], &guide, data()?),
            "ratakor" => ratakor::cli(&args[2..], &guide, data()?),
            "ethiraric" => ethiraric::cli(&args[2..], &guide, data()?),
            "codex" => codex::cli(&args[2..], &guide, data()?),
            "translation" => translation::cli(&args[2..], &guide, data()?, localedb()?),
            "backups" => backups::cli(&args[2..], &guide, data()?),
            "merge" => merge::cli(&args[2..], &guide, data()?),
            subcommand => Err(Error::Misc(format!(
                "Invalid CLI subcommand: {}",
                subcommand
            ))),
        }
    }
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
