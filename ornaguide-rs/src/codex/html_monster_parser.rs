use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeDataRef, NodeRef};

use crate::{
    codex::{CodexBoss, CodexMonster, CodexRaid, MonsterAbility, MonsterDrop},
    error::Error,
    guide::html_utils::{parse_tags, Tag},
    misc::truncate_str_until,
    utils::html::{
        descend_iter, descend_to, get_attribute_from_node, list_attributes_form_node, node_to_text,
        parse_icon,
    },
};

/// Information extracted from the monster page.
/// This may be that of a regular monster, boss, or raid.
struct ExtractedInfo {
    /// The name of the monster.
    pub name: String,
    /// The description of the monster.
    pub description: Option<String>,
    /// The icon of the monster.
    pub icon: String,
    /// The events in which the monster appears.
    pub events: Vec<String>,
    /// The family to which the monster belongs.
    pub family: Option<String>,
    /// The rarity of the monster.
    pub rarity: Option<String>,
    /// The tier of the monster.
    pub tier: u8,
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
    /// The abilities of the monster.
    pub abilities: Vec<MonsterAbility>,
    /// The items the monster drops.
    pub drops: Vec<MonsterDrop>,
}

/// The contents of the `codex-page-description` node.
struct DescriptionNode {
    /// The description of the monster (Raids-only).
    pub description: Option<String>,
    /// The events in which the monster appears.
    pub events: Vec<String>,
    /// The family to which the monster belongs (non-Raids-only).
    pub family: Option<String>,
    /// The rarity of the monster (non-Raids-only).
    pub rarity: Option<String>,
}

/// Parse the events, family and rarity of the monster.
fn parse_description_nodes<T>(
    iter: impl Iterator<Item = NodeDataRef<T>>,
    has_description: bool,
) -> Result<DescriptionNode, Error> {
    let mut iter = iter.peekable();
    let mut description = None;
    let mut events = Vec::new();

    if has_description {
        if let Some(event_node) = iter.next() {
            description = Some(node_to_text(event_node.as_node()));
        }
    }

    if let Some(event_node) = iter.peek() {
        // The event string is composed of the different events separated by a single slash (`/`).
        if list_attributes_form_node(event_node.as_node(), "Description event node")?
            .into_iter()
            .any(|name| name == "codex-page-description-highlight")
        {
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

    if let (Some(family_node), Some(rarity_node), None) = (iter.next(), iter.next(), iter.next()) {
        Ok(DescriptionNode {
            description,
            events,
            family: Some(
                truncate_str_until(&node_to_text(family_node.as_node()), ':')
                    .ok_or_else(|| {
                        Error::HTMLParsingError(format!(
                            "Failed to find colon in: monster family {}",
                            &node_to_text(family_node.as_node())
                        ))
                    })?
                    .trim()
                    .to_string(),
            ),
            rarity: Some(
                truncate_str_until(&node_to_text(rarity_node.as_node()), ':')
                    .ok_or_else(|| {
                        Error::HTMLParsingError(format!(
                            "Failed to find colon in: monster rarity {}",
                            &node_to_text(rarity_node.as_node())
                        ))
                    })?
                    .trim()
                    .to_string(),
            ),
        })
    } else {
        Ok(DescriptionNode {
            description,
            events,
            family: None,
            rarity: None,
        })
    }
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
            "Failed to find ':' when parsing monster tier: \"{}\"",
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
fn parse_abilities(iter_node: &NodeRef) -> Result<Vec<MonsterAbility>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| MonsterAbility { name, uri, icon }))
        .collect()
}

/// Parse drops the `h4` drops node.
fn parse_drops(iter_node: &NodeRef) -> Result<Vec<MonsterDrop>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| MonsterDrop { name, uri, icon }))
        .collect()
}

/// Parses a monster page from `playorna.com` and returns the details about the given monster.
fn parse_html_page(
    contents: &str,
    skip_abilities_drops_tags: bool,
    has_description: bool,
) -> Result<ExtractedInfo, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let descriptions_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let mut tags = Vec::new();
    let mut abilities = vec![];
    let mut drops = vec![];

    let DescriptionNode {
        description,
        events,
        family,
        rarity,
    } = parse_description_nodes(descriptions_it, has_description)?;

    if !skip_abilities_drops_tags {
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
        tags = parse_tags(descend_iter(page.as_node(), ".codex-page-tag", "page")?)?;
    }

    Ok(ExtractedInfo {
        name: node_to_text(name.as_node()),
        description,
        icon: parse_icon(icon.as_node())?,
        events,
        family,
        rarity,
        tier: parse_tier(tier.as_node())?,
        tags,
        abilities,
        drops,
    })
}

/// Parses a monster page from `playorna.com` and returns the details about the given monster.
pub fn parse_html_codex_monster(contents: &str, slug: String) -> Result<CodexMonster, Error> {
    parse_html_page(contents, false, false)
        .and_then(|info| {
            Ok(CodexMonster {
                slug: slug.clone(),
                name: info.name,
                icon: info.icon,
                events: info.events,
                family: info.family.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve family from monster".to_string())
                })?,
                rarity: info.rarity.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve rarity from monster".to_string())
                })?,
                tier: info.tier,
                abilities: info.abilities,
                drops: info.drops,
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Monster {}: {}", slug, msg))
            }
            x => x,
        })
}

/// Parses a boss page from `playorna.com` and returns the details about the given boss.
pub fn parse_html_codex_boss(contents: &str, slug: String) -> Result<CodexBoss, Error> {
    parse_html_page(contents, false, false)
        .and_then(|info| {
            Ok(CodexBoss {
                slug: slug.clone(),
                name: info.name,
                icon: info.icon,
                events: info.events,
                family: info.family.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve family from monster".to_string())
                })?,
                rarity: info.rarity.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve rarity from monster".to_string())
                })?,
                tier: info.tier,
                abilities: info.abilities,
                drops: info.drops,
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Boss {}: {}", slug, msg))
            }
            x => x,
        })
}

/// Parses a raid page from `playorna.com` and returns the details about the given raid.
pub fn parse_html_codex_raid(contents: &str, slug: String) -> Result<CodexRaid, Error> {
    parse_html_page(contents, false, true)
        .and_then(|info| {
            Ok(CodexRaid {
                slug: slug.clone(),
                name: info.name,
                description: info.description.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve description from raid".to_string())
                })?,
                icon: info.icon,
                events: info.events,
                tier: info.tier,
                tags: info.tags,
                abilities: info.abilities,
                drops: info.drops,
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Raid {}: {}", slug, msg))
            }
            x => x,
        })
}

/// Parses a monster page from `playorna.com` and returns the details about the given monster.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - abilities
///   - drops
pub fn parse_html_codex_monster_translation(
    contents: &str,
    slug: String,
) -> Result<CodexMonster, Error> {
    parse_html_page(contents, true, false)
        .and_then(|info| {
            Ok(CodexMonster {
                slug: slug.clone(),
                name: info.name,
                icon: info.icon,
                events: info.events,
                family: info.family.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve family from monster".to_string())
                })?,
                rarity: info.rarity.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve rarity from monster".to_string())
                })?,
                tier: info.tier,
                abilities: vec![],
                drops: vec![],
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Monster {}: {}", slug, msg))
            }
            x => x,
        })
}

/// Parses a boss page from `playorna.com` and returns the details about the given boss.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - abilities
///   - drops
pub fn parse_html_codex_boss_translation(contents: &str, slug: String) -> Result<CodexBoss, Error> {
    parse_html_page(contents, true, false)
        .and_then(|info| {
            Ok(CodexBoss {
                slug: slug.clone(),
                name: info.name,
                icon: info.icon,
                events: info.events,
                family: info.family.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve family from boss".to_string())
                })?,
                rarity: info.rarity.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve rarity from boss".to_string())
                })?,
                tier: info.tier,
                abilities: vec![],
                drops: vec![],
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Monster {}: {}", slug, msg))
            }
            x => x,
        })
}

/// Parses a raid page from `playorna.com` and returns the details about the given raid.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - abilities
///   - drops
///   - tags
pub fn parse_html_codex_raid_translation(contents: &str, slug: String) -> Result<CodexRaid, Error> {
    parse_html_page(contents, true, true)
        .and_then(|info| {
            Ok(CodexRaid {
                slug: slug.clone(),
                name: info.name,
                description: info.description.ok_or_else(|| {
                    Error::HTMLParsingError("Failed to retrieve description from raid".to_string())
                })?,
                icon: info.icon,
                events: info.events,
                tier: info.tier,
                tags: vec![],
                abilities: vec![],
                drops: vec![],
            })
        })
        .map_err(|err| match err {
            Error::HTMLParsingError(msg) => {
                Error::HTMLParsingError(format!("Monster {}: {}", slug, msg))
            }
            x => x,
        })
}
