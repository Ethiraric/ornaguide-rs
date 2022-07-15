use std::time::Instant;

use dotenv::dotenv;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use ornaguide_rs::{
    codex::{Codex, CodexItem},
    error::Error,
    guide::{AdminGuide, Guide, OrnaAdminGuide},
};
use output::OrnaData;

mod codex;
mod guide;
mod guide_match;
mod misc;
mod output;
mod ratakor;
mod sirscor;

#[allow(unused_variables, unused_mut)]
/// Danger zone. Where I test my code.
fn ethi(guide: &OrnaAdminGuide, mut data: OrnaData) -> Result<(), Error> {
    guide_match::all(&mut data, false, guide)?;

    Ok(())
}

fn main2() -> Result<(), Error> {
    let _ = dotenv();
    let cookie = dotenv::var("ORNAGUIDE_COOKIE").unwrap();
    let guide = OrnaAdminGuide::new(&cookie)?;
    let data = || OrnaData::load_from("output");

    let args = std::env::args().collect::<Vec<_>>();
    match args.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        [_, "json", "refresh"] => output::refresh(&guide).map(|_| ()),
        [_, "match", "all"] => guide_match::all(&mut data()?, false, &guide),
        [_, "match", "all", "--fix"] => guide_match::all(&mut data()?, true, &guide),
        [_, "match", "items"] => guide_match::items::perform(&data()?, false, &guide),
        [_, "match", "items", "--fix"] => guide_match::items::perform(&data()?, true, &guide),
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
        [_] => ethi(&guide, data()?),
        _ => Err(Error::Misc("Invalid CLI arguments".to_string())),
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
