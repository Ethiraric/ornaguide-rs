use std::{fs::File, io::BufReader, path::Path};

use crate::{error::Error, guide::Guide, items::RawItem};

/// A cache of the API responses, in a directory of jsons.
pub struct CachedGuide {
    items: Vec<RawItem>,
}

impl CachedGuide {
    pub fn from_directory(path: &Path) -> Result<CachedGuide, Error> {
        let file = File::open(path.join("item.json"))?;
        let reader = BufReader::new(file);
        let items: Vec<RawItem> = serde_json::from_reader(reader)?;
        Ok(CachedGuide { items })
    }
}

impl Guide for CachedGuide {
    fn fetch_items(&mut self) -> Result<&[RawItem], crate::error::Error> {
        Ok(&self.items)
    }

    fn get_items(&self) -> Option<&[RawItem]> {
        Some(&self.items)
    }
}
