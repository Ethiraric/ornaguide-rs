use std::{fs::File, io::BufReader, path::Path};

use crate::{error::Error, guide::Guide, items::RawItem, monsters::RawMonster};

/// A cache of the API responses, in a directory of jsons.
pub struct CachedGuide {
    items: Vec<RawItem>,
    monsters: Vec<RawMonster>,
}

impl CachedGuide {
    pub fn from_directory(path: &Path) -> Result<CachedGuide, Error> {
        let items: Vec<RawItem> =
            serde_json::from_reader(BufReader::new(File::open(path.join("item.json"))?))?;
        let monsters: Vec<RawMonster> =
            serde_json::from_reader(BufReader::new(File::open(path.join("monster.json"))?))?;
        Ok(CachedGuide { items, monsters })
    }
}

impl Guide for CachedGuide {
    fn fetch_items(&mut self) -> Result<&[RawItem], crate::error::Error> {
        Ok(&self.items)
    }

    fn get_items(&self) -> Option<&[RawItem]> {
        Some(&self.items)
    }

    fn fetch_monsters(&mut self) -> Result<&[RawMonster], crate::error::Error> {
        Ok(&self.monsters)
    }

    fn get_monsters(&self) -> Option<&[RawMonster]> {
        Some(&self.monsters)
    }
}
