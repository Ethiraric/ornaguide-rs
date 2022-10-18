use std::ops::Deref;

use kuchiki::{Attributes, ElementData, NodeData, NodeDataRef, NodeRef};
use serde::{Deserialize, Serialize};

use crate::{error::Error, utils::html::node_to_text};

/// A tag attached to an item, a monster or a skill.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Tag {
    FoundInChests,
    FoundInShops,
    WorldRaid,
    KingdomRaid,
    OffHandAbility,
    FoundInArcanists,
    OtherRealmsRaid,
    FoundInArena,
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
            "✓ Off-hand ability" => tags.push(Tag::OffHandAbility),
            "✓ Found in Arcanists" => tags.push(Tag::FoundInArcanists),
            "✓ Other Realms Raid" => tags.push(Tag::OtherRealmsRaid),
            "✓ Found in the arena" => tags.push(Tag::FoundInArena),
            x => return Err(Error::HTMLParsingError(format!("Unknown tag: {}", x))),
        }
    }

    Ok(tags)
}

/// Parse a string of the form `<name> (<chance>%)`.
pub fn parse_name_and_chance<'a>(text: &'a str, kind: &str) -> Result<(&'a str, i8), Error> {
    let text = text.trim();
    if let Some(pos) = text.find('(') {
        let (name, chance) = text.split_at(pos);
        Ok((
            name.trim(),
            chance
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_end_matches('%')
                .parse()?,
        ))
    } else {
        return Err(Error::HTMLParsingError(format!(
            "Failed to find '(' when parsing {} chance: \"{}\"",
            kind, text
        )));
    }
}

/// Return true if the node is an HTML node.
/// Return false otherwise (text node, comment node, doctype, ...)
pub fn is_html_tag_node(node: &NodeRef) -> bool {
    matches!(node.data(), NodeData::Element(_))
}

/// Check if a node has the expected tag. If it does, call `f` and return `Some(f(...))`.
/// If the tag is unexpected or we couldn't the node data, return `None`.
pub fn descend_if_tag<F, Ret>(node: &NodeRef, expected_tag: &str, f: F) -> Option<Ret>
where
    F: FnOnce(&NodeRef, &Attributes) -> Ret,
{
    if let NodeData::Element(ElementData {
        name,
        attributes,
        template_contents: _,
    }) = node.data()
    {
        let tag = name.local.to_string();
        if tag == expected_tag {
            Some(f(node, attributes.borrow().deref()))
        } else {
            None
        }
    } else {
        None
    }
}
