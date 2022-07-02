use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeDataRef, NodeRef};

use crate::{
    codex::{CodexFollower, FollowerAbility},
    error::Error,
    utils::html::{descend_iter, descend_to, get_attribute_from_node, node_to_text, parse_icon},
};

/// The contents of the `codex-page-description` node.
struct DescriptionNode {
    /// The description of the follower.
    pub description: String,
    /// The events in which the follower appears.
    pub events: Vec<String>,
    /// The rarity of the follower.
    pub rarity: String,
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

/// Parse the text contents from a description node containing family, rarity or events.
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

/// Parse the events, family and rarity of the monster.
fn parse_description_nodes<T>(
    iter: impl Iterator<Item = NodeDataRef<T>>,
) -> Result<DescriptionNode, Error> {
    let mut iter = iter.peekable();
    let mut description = None;
    let mut events = Vec::new();

    if let Some(event_node) = iter.peek() {
        let txt = node_to_text(event_node.as_node());
        if !txt.starts_with("Event:") && !txt.starts_with("Rarity:") {
            description = Some(txt);
            iter.next();
        }
    }

    if let Some(event_node) = iter.peek() {
        // The event string is composed of the different events separated by a single slash (`/`).
        if let Ok(events_str) =
            parse_family_rarity_text(&node_to_text(event_node.as_node()), "Event:")
        {
            events = events_str
                .split('/')
                .map(|ev| ev.trim().to_string())
                .collect();
            events.sort_unstable();
            iter.next();
        }
    }

    let rarity = match iter.next() {
        Some(node) => {
            parse_family_rarity_text(&node_to_text(node.as_node()), "Rarity:")?.to_string()
        }
        None => {
            return Err(Error::HTMLParsingError(
                "Failed to find rarity node".to_string(),
            ));
        }
    };

    if description.is_none() {
        return Err(Error::HTMLParsingError(
            "Failed to find description text".to_string(),
        ));
    }

    Ok(DescriptionNode {
        description: description.unwrap(),
        events,
        rarity,
    })
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
fn parse_abilities(iter_node: &NodeRef) -> Result<Vec<FollowerAbility>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| FollowerAbility { name, uri, icon }))
        .collect()
}

/// Parse a follower page from `playorna.com` for details about a follower.
pub fn parse_html_codex_follower(contents: &str, slug: String) -> Result<CodexFollower, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let descriptions_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let mut abilities = vec![];

    let DescriptionNode {
        description,
        events,
        rarity,
    } = parse_description_nodes(descriptions_it)?;

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Abilities:" => {
                abilities = parse_abilities(h4.as_node())?;
            }
            x => panic!("{}", x),
        }
    }

    Ok(CodexFollower {
        name: node_to_text(name.as_node()),
        slug,
        icon: parse_icon(icon.as_node())?,
        description,
        tier: parse_tier(tier.as_node())?,
        events,
        rarity,
        abilities,
    })
}
