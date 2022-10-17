use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};
use serde::Deserialize;

use crate::retry_once;

#[derive(Deserialize)]
struct Record {
    raid: String,
    tier: u8,
    hp: u32,
}

/// Read a CSV containing raid data and push to the guide the raid hp.
/// The CSV must contain the following columns: `raid`, `tier`, `hp`.
pub fn push_raid_hp(file: &str, data: &OrnaData, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for record in csv::Reader::from_path(file).unwrap().deserialize() {
        let record: Record = record.unwrap();
        println!("{}", record.raid);
        let admin_raid = data
            .guide
            .monsters
            .monsters
            .iter()
            .filter(|monster| monster.is_raid(&data.guide.static_.spawns))
            .filter(|monster| monster.name == record.raid && monster.tier == record.tier)
            .collect_vec();
        if admin_raid.len() != 1 {
            panic!("{}: {:?}", record.raid, admin_raid);
        }
        let raid = admin_raid[0];
        let mut admin_raid = retry_once!(guide.admin_retrieve_monster_by_id(raid.id))?;
        if admin_raid.hp != record.hp {
            println!("{}-{}", admin_raid.hp, record.hp);
            admin_raid.hp = record.hp;
            guide.admin_save_monster(admin_raid)?;
            retry_once!(guide.admin_retrieve_monster_by_id(raid.id))?;
        }
    }
    Ok(())
}

/// Execute a CLI subcommand for ratakor.
pub fn cli(args: &[&str], guide: &OrnaAdminGuide, data: OrnaData) -> Result<(), Error> {
    match args {
        ["raid-hp", file] => push_raid_hp(file, &data, guide),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `ratakor` arguments: {:?}",
            &args
        ))),
    }
}
