use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeDataRef, NodeRef};

use crate::{
    codex::{CodexFollower, FollowerAbility},
    error::Error,
    misc::truncate_str_until,
    utils::html::{
        descend_iter, descend_to, get_attribute_from_node, list_attributes_form_node, node_to_text,
        parse_icon,
    },
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

/// Parse the events, family and rarity of the monster.
fn parse_description_nodes<T>(
    iter: impl Iterator<Item = NodeDataRef<T>>,
) -> Result<DescriptionNode, Error> {
    let mut iter = iter.peekable();
    let mut events = Vec::new();
    let rarity;

    // First node is the description.
    let description = if let Some(description_node) = iter.next() {
        node_to_text(description_node.as_node())
    } else {
        return Err(Error::HTMLParsingError(
            "No description node when parsing follower".to_string(),
        ));
    };

    // Look for the event node.
    if let Some(event_node) = iter.peek() {
        // Event nodes have a `highlight` attribute. Otherwise, the node isn't an event node.
        if list_attributes_form_node(event_node.as_node(), "Description event node")?
            .into_iter()
            .any(|name| name == "codex-page-description-highlight")
        {
            // The event string is composed of the different events separated by a single slash (`/`).
            if let Some(events_str) = truncate_str_until(&node_to_text(event_node.as_node()), ':') {
                events = events_str
                    .trim()
                    .split('/')
                    .map(|ev| ev.trim().to_string())
                    .collect();
                events.sort_unstable();
                iter.next();
            }
        }
    }

    // Look for the rarity node.
    if let Some(rarity_node) = iter.next() {
        if let Some(rarity_str) = truncate_str_until(&node_to_text(rarity_node.as_node()), ':') {
            rarity = rarity_str.trim().to_string();
        } else {
            return Err(Error::HTMLParsingError(
                "Failed to find ':' in rarity node".to_string(),
            ));
        }
    } else {
        return Err(Error::HTMLParsingError(
            "Failed to find rarity node".to_string(),
        ));
    }

    Ok(DescriptionNode {
        description,
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

/// Parse a follower page from `playorna.com` for details about a follower.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - abilities
pub fn parse_html_codex_follower_translation(
    contents: &str,
    slug: String,
) -> Result<CodexFollower, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let descriptions_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;

    let DescriptionNode {
        description,
        events,
        rarity,
    } = parse_description_nodes(descriptions_it)?;

    Ok(CodexFollower {
        name: node_to_text(name.as_node()),
        slug,
        icon: parse_icon(icon.as_node())?,
        description,
        tier: parse_tier(tier.as_node())?,
        events,
        rarity,
        abilities: vec![],
    })
}
