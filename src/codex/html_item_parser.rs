use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeDataRef, NodeRef};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    utils::html::{
        descend_iter, descend_to, get_attribute_from_node, node_to_text, try_descend_to,
    },
};

/// An element (fire, water, arcane, ...).
#[derive(Debug, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Earthen,
    Lightning,
    Holy,
    Dark,
    Arcane,
    Dragon,
    Physical,
}

/// A tag attached to an item.
#[derive(Debug, Serialize, Deserialize)]
pub enum Tag {
    FoundInChests,
    FoundInShops,
}

/// Stats of an item.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    /// The base attack stat of the item.
    pub attack: Option<i16>,
    /// The base magic stat of the item.
    pub magic: Option<i16>,
    /// The base HP stat of the item.
    pub hp: Option<i16>,
    /// The base MP stat of the item.
    pub mana: Option<i16>,
    /// The base defense stat of the item.
    pub defense: Option<i16>,
    /// The base resistance stat of the item.
    pub resistance: Option<i16>,
    /// The base ward stat of the item (%).
    pub ward: Option<i8>,
    /// The base dexterity stat of the item.
    pub dexterity: Option<i16>,
    /// The crit stat of the item.
    pub crit: Option<u8>,
    /// The foresight of the item.
    pub foresight: Option<i8>,
    /// The number of adorn slots at level 10, common quality.
    pub adornment_slots: Option<u8>,
    /// The elment of the item.
    pub element: Option<Element>,
}

/// The ability the item has in off-hand.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The description of the ability.
    pub description: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize)]
pub struct DroppedBy {
    /// The name of the monster.
    pub name: String,
    /// The uri to the monster.
    pub uri: String,
    /// The icon of the monster.
    pub icon: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeMaterial {
    /// The name of the material.
    pub name: String,
    /// The uri to the material.
    pub uri: String,
    /// The icon of the material.
    pub icon: String,
}

/// A debuff the item causes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cause {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// A buff the item gives.
#[derive(Debug, Serialize, Deserialize)]
pub struct Give {
    /// The name of the buff.
    pub name: String,
    /// The chance (0-100) of the effect happening.
    pub chance: i8,
    /// The icon of the buff.
    pub icon: String,
}

/// A debuff the item cures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cure {
    /// The name of the buff.
    pub name: String,
    /// The icon of the buff.
    pub icon: String,
}

/// An debuff the item prevents.
#[derive(Debug, Serialize, Deserialize)]
pub struct Immunity {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// An item on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// The name of the item.
    pub name: String,
    /// The icon of the item.
    pub icon: String,
    /// The description of the item.
    pub description: String,
    /// The tier of the item.
    pub tier: u8,
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
    /// The stats of the item.
    pub stats: Option<Stats>,
    /// The ability of the item.
    pub ability: Option<Ability>,
    /// Debuffs the item can cause.
    pub causes: Vec<Cause>,
    /// Debuffs the item cures.
    pub cures: Vec<Cure>,
    /// Buffs the item can give.
    pub gives: Vec<Give>,
    /// Immunities the item grants.
    pub immunities: Vec<Immunity>,
    /// The monsters that drop the item.
    pub dropped_by: Vec<DroppedBy>,
    /// The materials needed to upgrade the item.
    pub upgrade_materials: Vec<UpgradeMaterial>,
}

/// Parse the icon of the item.
/// Returns an URL path, without the host.
fn parse_icon(node: &NodeRef) -> Result<String, Error> {
    Ok(Url::parse(&get_attribute_from_node(
        descend_to(node, "img", "icon-node")?.as_node(),
        "src",
        "img icon node",
    )?)
    .unwrap()
    .path()
    .to_string())
}

/// Parse the tier of the skill.
fn parse_tier(node: &NodeRef) -> Result<u8, Error> {
    let text = node_to_text(node);
    let text = text.trim();
    if let Some(pos) = text.find(':') {
        let (_, tier_with_star) = text.split_at(pos + 1);
        let mut it = tier_with_star.trim().chars();
        it.next(); // Skip over the star.
        Ok(it.as_str().parse()?)
    } else {
        Err(Error::HTMLParsingError(format!(
            "Failed to find ':' when parsing skill tier: \"{}\"",
            text
        )))
    }
}

/// Parse a `<a>` node to a `name`, `uri`, `icon` tuple.
fn a_to_name_uri_icon(a: &NodeRef) -> Result<(String, String, String), Error> {
    let uri = get_attribute_from_node(a, "href", "monster <a>")?;
    let img = descend_to(a, "img", "monster <a>")?;
    let icon = get_attribute_from_node(img.as_node(), "src", "monster <a> img")?;
    let name = node_to_text(a);
    Ok((name, uri, icon))
}

/// Parse a `<div>` node to a `name`, `icon` tuple.
fn div_to_name_icon(div: &NodeRef) -> Result<(String, String), Error> {
    let img = descend_to(div, "img", "monster <a>")?;
    let icon = get_attribute_from_node(img.as_node(), "src", "item <div> img")?;
    let name = node_to_text(div);
    Ok((name, icon))
}

/// Parse a list of `name`, `uri`, `icon` tuples.
fn parse_name_uri_icon_list(
    iter_node: &NodeRef,
) -> impl Iterator<Item = Result<(String, String, String), Error>> {
    iter_node
        .following_siblings()
        .into_iter()
        .filter(|node| matches!(node.data(), NodeData::Element(_)))
        .map_while(|node| {
            if let NodeData::Element(ElementData {
                name,
                attributes: _attributes,
                template_contents: _,
            }) = node.data()
            {
                let tag = name.local.to_string();
                match tag.deref() {
                    "h4" | "hr" => None,
                    "div" => Some(
                        descend_to(&node, "a", "div drop or ability")
                            .and_then(|node| a_to_name_uri_icon(node.as_node())),
                    ),
                    _ => Some(Err(Error::HTMLParsingError(format!(
                        "Unknown node tag when parsing drop or ability: {}",
                        &tag
                    )))),
                }
            } else {
                panic!("Cannot happen due to previous filter");
            }
        })
}

/// Parse a list of `name`, `icon` tuples.
fn parse_name_icon_list(
    iter_node: &NodeRef,
) -> impl Iterator<Item = Result<(String, String), Error>> {
    iter_node
        .following_siblings()
        .into_iter()
        .filter(|node| matches!(node.data(), NodeData::Element(_)))
        .map_while(|node| {
            if let NodeData::Element(ElementData {
                name,
                attributes: _attributes,
                template_contents: _,
            }) = node.data()
            {
                let tag = name.local.to_string();
                match tag.deref() {
                    "h4" | "hr" => None,
                    "div" => Some(div_to_name_icon(&node)),
                    _ => Some(Err(Error::HTMLParsingError(format!(
                        "Unknown node tag when parsing drop or ability: {}",
                        &tag
                    )))),
                }
            } else {
                panic!("Cannot happen due to previous filter");
            }
        })
}

/// Parse the tags of the item.
fn parse_tags<T>(iter: impl Iterator<Item = NodeDataRef<T>>) -> Result<Vec<Tag>, Error> {
    let mut tags = vec![];

    for node in iter {
        match node_to_text(node.as_node()).as_str() {
            "✓ Found in chests" => tags.push(Tag::FoundInChests),
            "✓ Found in shops" => tags.push(Tag::FoundInShops),
            x => return Err(Error::HTMLParsingError(format!("Unknown tag: {}", x))),
        }
    }

    Ok(tags)
}

/// Parse the stats of the item.
fn parse_stats(node: Option<&NodeRef>) -> Result<Option<Stats>, Error> {
    if let Some(node) = node {
        let mut stats = Stats::default();
        for node in descend_iter(node, ".codex-stat", "codex stats node")? {
            let text = node_to_text(node.as_node());
            let text = text.trim();
            if let Some(pos) = text.find(':') {
                let (stat, value) = text.split_at(pos + 1);
                let stat = stat.trim();
                let value = value.trim().trim_end_matches('%');
                match stat {
                    "Attack:" => stats.attack = Some(value.parse()?),
                    "Magic:" => stats.magic = Some(value.parse()?),
                    "Defense:" => stats.defense = Some(value.parse()?),
                    "Resistance:" => stats.resistance = Some(value.parse()?),
                    "HP:" => stats.hp = Some(value.parse()?),
                    "Mana:" => stats.mana = Some(value.parse()?),
                    "Ward:" => stats.ward = Some(value.parse()?),
                    "Dexterity:" => stats.dexterity = Some(value.parse()?),
                    "Crit:" => stats.crit = Some(value.parse()?),
                    "Foresight:" => stats.foresight = Some(value.parse()?),
                    "Adornment Slots:" => stats.adornment_slots = Some(value.parse()?),
                    _ => panic!("Failed to parse stat: {}", text),
                }
            } else {
                match text {
                    "Fire" => stats.element = Some(Element::Fire),
                    "Water" => stats.element = Some(Element::Water),
                    "Earthen" => stats.element = Some(Element::Earthen),
                    "Lightning" => stats.element = Some(Element::Lightning),
                    "Holy" => stats.element = Some(Element::Holy),
                    "Dark" => stats.element = Some(Element::Dark),
                    "Arcane" => stats.element = Some(Element::Arcane),
                    "Dragon" => stats.element = Some(Element::Dragon),
                    "Physical" => stats.element = Some(Element::Physical),
                    _ => {
                        return Err(Error::HTMLParsingError(format!(
                            "Failed to find ':' when parsing stat: \"{}\"",
                            text
                        )));
                    }
                }
            }
        }
        Ok(Some(stats))
    } else {
        Ok(None)
    }
}

/// From a string `Status (x%)`, return a tuple `("Status", x)`.
fn split_status_chance(text: &str) -> Result<(String, i8), Error> {
    if let Some(pos) = text.find('(') {
        let (status, chance) = text.split_at(pos);
        Ok((
            status.trim().to_string(),
            chance
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_end_matches('%')
                .parse()?,
        ))
    } else {
        Err(Error::HTMLParsingError(format!(
            "Failed to find '(' when parsing status effect: \"{}\"",
            text
        )))
    }
}

/// Parse causes from the `h4` abilities node.
fn parse_causes(iter_node: &NodeRef) -> Result<Vec<Cause>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Cause { name, icon }))
        .collect()
}

/// Parse gives from the `h4` abilities node.
fn parse_gives(iter_node: &NodeRef) -> Result<Vec<Give>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| {
            tupleresult.and_then(|(text, icon)| {
                split_status_chance(&text).map(|(name, chance)| Give { name, chance, icon })
            })
        })
        .collect()
}

/// Parse cures from the `h4` abilities node.
fn parse_cures(iter_node: &NodeRef) -> Result<Vec<Cure>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Cure { name, icon }))
        .collect()
}

/// Parse immunitiees from the `h4` abilities node.
fn parse_immunities(iter_node: &NodeRef) -> Result<Vec<Immunity>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Immunity { name, icon }))
        .collect()
}

/// Parse dropped by from the `h4` abilities node.
fn parse_dropped_by(iter_node: &NodeRef) -> Result<Vec<DroppedBy>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| DroppedBy { name, uri, icon }))
        .collect()
}

/// Parse drops the `h4` drops node.
fn parse_upgrade_materials(iter_node: &NodeRef) -> Result<Vec<UpgradeMaterial>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| UpgradeMaterial { name, uri, icon }))
        .collect()
}

fn parse_ability(node: Option<&NodeRef>) -> Result<Option<Ability>, Error> {
    if let Some(node) = node {
        if let Some(previous) = node.previous_sibling().and_then(|n| n.previous_sibling()) {
            let text = node_to_text(&previous);
            let text = text.trim();
            if let Some(pos) = text.find(':') {
                let (left, right) = text.split_at(pos + 1);
                let left = left.trim();
                let right = right.trim();
                if left == "Ability:" {
                    Ok(Some(Ability {
                        name: right.to_string(),
                        description: node_to_text(node),
                    }))
                } else {
                    Err(Error::HTMLParsingError(format!(
                        "Failed to find 'Ability:' when parsing: \"{}\"",
                        text
                    )))
                }
            } else {
                Err(Error::HTMLParsingError(format!(
                    "Failed to find ':' when parsing ability name: \"{}\"",
                    text
                )))
            }
        } else {
            Err(Error::HTMLParsingError(
                "Failed to find previous node when parsing ability".to_string(),
            ))
        }
    } else {
        Ok(None)
    }
}

/// Parses an item page from `playorna.com` and returns the details about the given item.
pub fn parse_html_codex_item(contents: &str) -> Result<Item, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let mut description_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let stats_parent = try_descend_to(page.as_node(), ".codex-stats", "page")?;

    let mut causes = vec![];
    let mut cures = vec![];
    let mut gives = vec![];
    let mut immunities = vec![];
    let mut dropped_by = vec![];
    let mut upgrade_materials = vec![];

    let tags = parse_tags(descend_iter(page.as_node(), ".codex-page-tag", "page")?)?;

    let description = if let Some(description) = description_it.next() {
        node_to_text(description.as_node())
    } else {
        return Err(Error::HTMLParsingError(
            "Failed to find description".to_string(),
        ));
    };

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Causes:" => {
                causes = parse_causes(h4.as_node())?;
            }
            "Gives:" => {
                gives = parse_gives(h4.as_node())?;
            }
            "Cures:" => {
                cures = parse_cures(h4.as_node())?;
            }
            "Immunities:" => {
                immunities = parse_immunities(h4.as_node())?;
            }
            "Dropped by:" => {
                dropped_by = parse_dropped_by(h4.as_node())?;
            }
            "Upgrade materials:" => {
                upgrade_materials = parse_upgrade_materials(h4.as_node())?;
            }
            x => panic!("{}", x),
        }
    }

    Ok(Item {
        name: node_to_text(name.as_node()),
        icon: parse_icon(icon.as_node())?,
        description,
        tier: parse_tier(tier.as_node())?,
        stats: parse_stats(stats_parent.as_ref().map(|n| n.as_node()))?,
        ability: parse_ability(description_it.next().as_ref().map(|n| n.as_node()))?,
        causes,
        cures,
        gives,
        immunities,
        dropped_by,
        upgrade_materials,
        tags,
    })
}
