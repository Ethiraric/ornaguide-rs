use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeDataRef, NodeRef};

use crate::{
    error::Error,
    utils::html::{descend_iter, descend_to, get_attribute_from_node, node_to_text, parse_icon},
};

/// An ability for a monster
#[derive(Debug)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The uri to the ability.
    pub uri: String,
    /// The icon of the ability.
    pub icon: String,
}

/// A drop for a monster
#[derive(Debug)]
pub struct Drop {
    /// The name of the item.
    pub name: String,
    /// The uri to the item.
    pub uri: String,
    /// The icon of the item.
    pub icon: String,
}

/// A monster on the codex.
#[derive(Debug)]
pub struct CodexMonster {
    /// The name of the monster.
    pub name: String,
    /// The icon of the monster.
    pub icon: String,
    /// The event in which the monster appears.
    pub event: Option<String>,
    /// The family to which the monster belongs.
    pub family: String,
    /// The rarity of the monster.
    pub rarity: String,
    /// The tier of the monster.
    pub tier: i8,
    /// The abilities of the monster.
    pub abilities: Vec<Ability>,
    /// The items the monster drops.
    pub drops: Vec<Drop>,
}

/// Parse the text contents from a description node containing family or rarity.
fn parse_family_rarity_text<'a>(txt: &'a str, expected_left: &str) -> Result<&'a str, Error> {
    if let Some(pos) = txt.find(':') {
        let (left, right) = txt.split_at(pos + 1);
        if left != expected_left {
            Err(Error::HTMLParsingError(format!(
                "Failed to parse family or rarity. Expected {}, got {}",
                expected_left, left,
            )))
        } else {
            Ok(right.trim())
        }
    } else {
        Err(Error::HTMLParsingError(format!(
            "Failed to find family or rarity in: {}",
            txt,
        )))
    }
}

/// Parse the event, family and rarity of the monster.
fn parse_event_family_rarity<T>(
    iter: impl Iterator<Item = NodeDataRef<T>>,
) -> Result<(Option<String>, String, String), Error> {
    let mut iter = iter.peekable();
    let mut event = None;
    if let Some(event_node) = iter.peek() {
        if let Ok(ev) = parse_family_rarity_text(&node_to_text(event_node.as_node()), "Event:") {
            event = Some(ev.to_string());
            iter.next();
        }
    }
    if let (Some(family_node), Some(rarity_node), None) = (iter.next(), iter.next(), iter.next()) {
        Ok((
            event,
            parse_family_rarity_text(&node_to_text(family_node.as_node()), "Family:")?.to_string(),
            parse_family_rarity_text(&node_to_text(rarity_node.as_node()), "Rarity:")?.to_string(),
        ))
    } else {
        Err(Error::HTMLParsingError(
            "Failed to retrieve family and rarity from iterator".to_string(),
        ))
    }
}

/// Parse the tier of the skill.
fn parse_tier(node: &NodeRef) -> Result<i8, Error> {
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

/// Parse abilities from the `h4` abilities node.
fn parse_abilities(iter_node: &NodeRef) -> Result<Vec<Ability>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| Ability { name, uri, icon }))
        .collect()
}

/// Parse drops the `h4` drops node.
fn parse_drops(iter_node: &NodeRef) -> Result<Vec<Drop>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| Drop { name, uri, icon }))
        .collect()
}

/// Parses a monster page from `playorna.com` and returns the details about the given monster.
pub fn parse_html_codex_monster(contents: &str) -> Result<CodexMonster, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let family_rarity_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let mut abilities = vec![];
    let mut drops = vec![];

    let (event, family, rarity) = parse_event_family_rarity(family_rarity_it)?;

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Abilities:" => {
                abilities = parse_abilities(h4.as_node())?;
            }
            "Drops:" => {
                drops = parse_drops(h4.as_node())?;
            }
            x => panic!("{}", x),
        }
    }

    Ok(CodexMonster {
        name: node_to_text(name.as_node()),
        icon: parse_icon(icon.as_node())?,
        event,
        family,
        rarity,
        tier: parse_tier(tier.as_node())?,
        abilities,
        drops,
    })
}
