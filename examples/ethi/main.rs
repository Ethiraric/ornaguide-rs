use std::time::Instant;

use dotenv::dotenv;

#[allow(unused_imports)]
use ornaguide_rs::{
    codex::{Codex, CodexItem},
    error::Error,
    guide::{AdminGuide, Guide, OrnaAdminGuide},
};
use output::{refresh, OrnaData};

#[allow(unused_imports)]
use crate::misc::diff_sorted_slices;

mod codex;
mod guide;
mod guide_match;
mod misc;
mod output;

#[allow(unused_variables, unused_mut)]
/// Danger zone. Where I test my code.
fn ethi(guide: &OrnaAdminGuide, mut data: OrnaData) -> Result<(), Error> {
    // guide_match::items::perform(&data, true, guide)?;
    guide_match::monsters::perform(&mut data, false, guide)?;

    // let monster_id = 442;
    // let mut monster = guide.admin_retrieve_monster_by_id(monster_id)?;
    // assert_eq!(monster.name, "Medea");
    // monster.name = "Medea, Arisen Queen".to_string();
    // guide.admin_save_monster(monster)?;
    // guide.admin_retrieve_monster_by_id(monster_id)?;
    Ok(())
}

fn main2() -> Result<(), Error> {
    let _ = dotenv();
    let cookie = dotenv::var("ORNAGUIDE_COOKIE").unwrap();
    let guide = OrnaAdminGuide::new(&cookie)?;
    let data = || OrnaData::load_from("output");

    let args = std::env::args().collect::<Vec<_>>();
    match args.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..] {
        [_, "json", "refresh"] => refresh(&guide),
        [_, "match", "items"] => guide_match::items::perform(&data()?, false, &guide),
        [_, "match", "items", "--fix"] => guide_match::items::perform(&data()?, true, &guide),
        [_, "match", "monsters"] => guide_match::monsters::perform(&mut data()?, false, &guide),
        [_, "match", "monsters", "--fix"] => {
            guide_match::monsters::perform(&mut data()?, true, &guide)
        }
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
