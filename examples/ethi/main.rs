#![allow(unused_imports, dead_code, unused_variables)]
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::time::{Instant, SystemTime};

use dotenv::dotenv;
use itertools::Itertools;

use ornaguide_rs::guide::Static;
use ornaguide_rs::{
    codex::{Codex, CodexItem},
    error::Error,
    guide::{AdminGuide, CachedGuide, Guide, OrnaAdminGuide},
    items::{admin::AdminItem, RawItem},
    monsters::{MonsterDrop, RawMonster},
    skills::{RawSkill, Skill},
};
use output::{generate_output_jsons, OrnaData};

use crate::codex::fetch::CodexItems;
use crate::guide::fetch::AdminItems;

mod codex;
mod guide;
mod misc;
mod output;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn ethi() -> Result<(), Error> {
    let _ = dotenv();
    let cookie = dotenv::var("ORNAGUIDE_COOKIE").unwrap();
    let guide = OrnaAdminGuide::new(&cookie)?;
    let data = OrnaData::load_from("output")?;

    // codex::fetch::items(&guide)?;
    // codex::fetch::bosses(&guide)?;
    // codex::fetch::raids(&guide)?;
    // guide::fetch::items(&guide)?;
    // adorn_slots(&guide)?;
    // generate_output_jsons(&guide)?;

    Ok(())
}

fn main() {
    let begin = Instant::now();
    match ethi() {
        Ok(_) => println!("OK"),
        Err(err) => eprintln!("Error: {}", err),
    }
    let end = Instant::now();
    let elapsed = end.duration_since(begin);
    println!("Executed in {}ms", elapsed.as_millis());
}
