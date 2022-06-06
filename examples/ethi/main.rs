use std::time::Instant;

use dotenv::dotenv;

#[allow(unused_imports)]
use ornaguide_rs::{
    codex::{Codex, CodexItem},
    error::Error,
    guide::{AdminGuide, Guide, OrnaAdminGuide},
};
use output::{refresh, OrnaData};

mod codex;
mod guide;
mod guide_match;
mod misc;
mod output;

#[allow(unused_variables)]
/// Danger zone. Where I test my code.
fn ethi(guide: &OrnaAdminGuide, data: &OrnaData) -> Result<(), Error> {
    guide_match::items::perform(data, false, guide)?;
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
        [_] => ethi(&guide, &data()?),
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
