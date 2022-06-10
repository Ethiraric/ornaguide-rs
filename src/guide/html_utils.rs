use kuchiki::NodeDataRef;
use serde::{Deserialize, Serialize};

use crate::{error::Error, utils::html::node_to_text};

/// A tag attached to an item or a monster.
#[derive(Debug, Serialize, Deserialize)]
pub enum Tag {
    FoundInChests,
    FoundInShops,
    WorldRaid,
    KingdomRaid,
}

/// Parse the tags of the item.
pub fn parse_tags<T>(iter: impl Iterator<Item = NodeDataRef<T>>) -> Result<Vec<Tag>, Error> {
    let mut tags = vec![];

    for node in iter {
        match node_to_text(node.as_node()).as_str() {
            "✓ Found in chests" => tags.push(Tag::FoundInChests),
            "✓ Found in shops" => tags.push(Tag::FoundInShops),
            "✓ World Raid" => tags.push(Tag::WorldRaid),
            "✓ Kingdom Raid" => tags.push(Tag::KingdomRaid),
            x => return Err(Error::HTMLParsingError(format!("Unknown tag: {}", x))),
        }
    }

    Ok(tags)
}
