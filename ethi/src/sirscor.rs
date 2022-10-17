use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};
use serde::Deserialize;

use crate::retry_once;

#[derive(Debug, Deserialize)]
struct Record {
    item: String,
    tier: u8,
    rarity: String,
}

impl Record {
    fn rarity(&self) -> String {
        match self.rarity.as_str() {
            "Common" => "CO".to_string(),
            "Superior" => "SP".to_string(),
            "Famed" => "FM".to_string(),
            "Legendary" => "LG".to_string(),
            _ => "NO".to_string(),
        }
    }
}

/// Read a CSV containing item rarity and push to the guide the item rarities.
pub fn push_rarity(file: &str, data: &OrnaData, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for record in csv::Reader::from_path(file).unwrap().deserialize() {
        let record: Record = record.unwrap();
        if ["Blinders", "Steadfast Charm"].contains(&record.item.as_str()) {
            continue;
        }
        println!("{:30} -> {}", record.item, record.rarity());
        let admin_item = data
            .guide
            .items
            .items
            .iter()
            .filter(|item| item.name == record.item && item.tier == record.tier)
            .collect_vec();
        if admin_item.len() != 1 {
            panic!("{}: {:?}", record.item, admin_item);
        }
        let item = admin_item[0];
        let mut admin_item = retry_once!(guide.admin_retrieve_item_by_id(item.id))?;
        let rarity = record.rarity();
        if admin_item.rarity != rarity {
            admin_item.rarity = rarity;
            guide.admin_save_item(admin_item)?;
            retry_once!(guide.admin_retrieve_item_by_id(item.id))?;
        }
    }
    Ok(())
}

/// Execute a CLI subcommand for sirscor.
pub fn cli(args: &[&str], guide: &OrnaAdminGuide, data: OrnaData) -> Result<(), Error> {
    match args {
        ["rarity", file] => push_rarity(file, &data, guide),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `sirscor` arguments: {:?}",
            &args
        ))),
    }
}
