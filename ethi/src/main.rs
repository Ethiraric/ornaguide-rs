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
    match args.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        [_, "json", "refresh"] => output::refresh(&guide).map(|_| ()),
        [_, "json", "refresh", "guide"] => output::refresh_guide(&guide, data()?.codex).map(|_| ()),
        [_, "json", "refresh", "guide", "static"] => {
            output::refresh_guide_static(&guide, data()?).map(|_| ())
        }
        [_, "json", "refresh", "codex"] => output::refresh_codex(&guide, data()?.guide).map(|_| ()),
        [_, "match", "all"] => guide_match::all(&mut data()?, false, &guide),
        [_, "match", "all", "--fix"] => {
            let mut data = data()?;
            guide_match::all(&mut data, true, &guide)?;
            data.save_to("output")
        }
        [_, "match", "items"] => guide_match::items::perform(&mut data()?, false, &guide),
        [_, "match", "items", "--fix"] => guide_match::items::perform(&mut data()?, true, &guide),
        [_, "match", "monsters"] => guide_match::monsters::perform(&mut data()?, false, &guide),
        [_, "match", "monsters", "--fix"] => {
            guide_match::monsters::perform(&mut data()?, true, &guide)
        }
        [_, "match", "pets"] => guide_match::pets::perform(&mut data()?, false, &guide),
        [_, "match", "pets", "--fix"] => guide_match::pets::perform(&mut data()?, true, &guide),
        [_, "match", "skills"] => guide_match::skills::perform(&mut data()?, false, &guide),
        [_, "match", "skills", "--fix"] => guide_match::skills::perform(&mut data()?, true, &guide),
        [_, "sirscor", "rarity", file] => sirscor::push_rarity(file, &data()?, &guide),
        [_, "ratakor", "raid-hp", file] => ratakor::push_raid_hp(file, &data()?, &guide),
        [_, "ethiraric", "summons", file] => ethiraric::summons::summons(file),
        [_, "codex", "bugs"] => codex_bugs::check(&data()?, &guide),
        [_, "codex", "missing"] => codex::fetch::missing(&guide, &data()?).map(|_| ()),
        [_, "translation", "missing"] => {
            let mut locales = localedb()?;
            let missing = codex::fetch::missing_translations(&guide, &data()?, &locales)?;
            locales.merge_with(missing);
            locales.save_to("output/i18n")
        }
        [_, "translation", locale] => codex::fetch::translations(&guide, &data()?, locale)?
            .save_to(&format!("output/i18n/{}.json", locale)),
        [_, "backups", "prune"] => backups::prune("backups_output"),
        [_, "backups", "merge"] => backups::merge("backups_output", "merges"),
        [_, "merge", "match"] => merge::match_(false, &guide),
        [_, "merge", "match", "--fix"] => merge::match_(true, &guide),
        [_] => ethi(&guide, data()?),
        _ => Err(Error::Misc(format!("Invalid CLI arguments: {:?}", &args))),
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
