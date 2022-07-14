use itertools::Itertools;
use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};
use serde::Deserialize;

use crate::{output::OrnaData, retry_once};

#[derive(Debug, Deserialize)]
struct Record {
    item: String,
    tier: u8,
    common: String,
    superior: String,
    famed: String,
    legendary: String,
}

impl Record {
    fn rarity(&self) -> String {
        if !self.common.is_empty() {
            "CO".to_string()
        } else if !self.superior.is_empty() {
            "SP".to_string()
        } else if !self.famed.is_empty() {
            "FM".to_string()
        } else if !self.legendary.is_empty() {
            "LG".to_string()
        } else {
            "NO".to_string()
        }
    }
}

/// Read a CSV containing item rarity and push to the guide the item rarities.
/// The CSV must contain the following columns: `item`, `tier`, `common`, `superior`, `famed`,
/// `legendary`. The rarity columns must contain something only if the item is of the given rarity.
pub fn push_rarity(file: &str, data: &OrnaData, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for record in csv::Reader::from_path(file).unwrap().deserialize() {
        let record: Record = record.unwrap();
        if ["Blinders", "Steadfast Charm"].contains(&record.item.as_str()) {
            continue;
        }
        println!("{}", record.item);
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
