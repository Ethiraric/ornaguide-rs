use std::ops::Deref;

use kuchiki::{
    iter::{Descendants, Elements, Select},
    parse_html,
    traits::TendrilSink,
    ElementData, NodeData, NodeDataRef, NodeRef,
};
use reqwest::Url;

use crate::error::Error;

/// A status effect caused or given from a skill.
#[derive(Debug)]
pub struct StatusEffect {
    /// The name of the effect.
    pub effect: String,
    /// The chance (0-100) of the effect happening.
    pub chance: i8,
}

/// A skill on the codex.
#[derive(Debug)]
pub struct CodexSkill {
    /// The name of the skill.
    pub name: String,
    /// The icon of the skill.
    pub icon: String,
    /// The description of the skill.
    pub description: String,
    /// The tier of the skill.
    pub tier: i8,
    /// The effects the skill causes to the opponent.
    pub causes: Vec<StatusEffect>,
    /// The effects the skill gives to the caster.
    pub gives: Vec<StatusEffect>,
}

/// Select the node that matches the selector and that is a descendant of `node`. `from_name` is a
/// name to be displayed on the error message.
fn descend_iter(
    node: &NodeRef,
    selector: &str,
    from_name: &str,
) -> Result<Select<Elements<Descendants>>, Error> {
    node.select(selector).map_err(|()| {
        Error::HTMLParsingError(format!("Failed to find \"{}\" in {}", selector, from_name))
    })
}

/// Select the node that matches the selector and that is a descendant of `node`. `from_name` is a
/// name to be displayed on the error message.
fn descend_to(
    node: &NodeRef,
    selector: &str,
    from_name: &str,
) -> Result<NodeDataRef<ElementData>, Error> {
    descend_iter(node, selector, from_name)?
        .next()
        .ok_or_else(|| {
            Error::HTMLParsingError(format!("Failed to find \"{}\" in {}", selector, from_name))
        })
}

/// Retrieve an attribute from an HTML node.
fn get_attribute_from_node(node: &NodeRef, attr: &str, node_name: &str) -> Result<String, Error> {
    if let NodeData::Element(ElementData {
        name: _,
        attributes,
        template_contents: _,
    }) = node.data()
    {
        let attributes = attributes.borrow();
        attributes
            .get(attr)
            .ok_or_else(|| {
                Error::HTMLParsingError(format!("Failed to find {} in {}", attr, node_name))
            })
            .map(|s| s.to_string())
    } else {
        Err(Error::HTMLParsingError(format!(
            "Failed to get attributes from {}",
            node_name
        )))
    }
}

/// Parse the icon of the skill.
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

/// Get the text contained in the node.
fn node_to_text(node: &NodeRef) -> String {
    node.text_contents().trim().to_string()
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

/// Parse a status effect section.
fn parse_status_effects(iter_node: &NodeRef) -> Result<Vec<StatusEffect>, Error> {
    let mut ret = vec![];
    let mut iter_node = (*iter_node).clone();
    while let Some(node) = iter_node.next_sibling() {
        if let NodeData::Element(ElementData {
            name,
            attributes: _attributes,
            template_contents: _,
        }) = node.data()
        {
            let tag = name.local.to_string();
            match tag.deref() {
                "h4" => {
                    break;
                }
                "div" => {
                    let span = descend_to(&node, "span", "skill status effect")?;
                    let text = span.text_contents();
                    let text = text.trim();
                    if let Some(pos) = text.find('(') {
                        let (status, chance) = text.split_at(pos);
                        ret.push(StatusEffect {
                            effect: status.trim().to_string(),
                            chance: chance
                                .trim()
                                .trim_start_matches('(')
                                .trim_end_matches(')')
                                .trim_end_matches('%')
                                .parse()?,
                        })
                    } else {
                        return Err(Error::HTMLParsingError(format!(
                            "Failed to find '(' when parsing status effect: \"{}\"",
                            text
                        )));
                    }
                }
                _ => panic!("Unknown node tag for status effect: {}", tag.deref()),
            };
        }
        iter_node = node;
    }
    Ok(ret)
}

/// Parses a page from `playorna.com` and returns the list of entries that were found and their
/// associated tiers.
pub fn parse_html_codex_skill(contents: &str) -> Result<CodexSkill, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let description = descend_to(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let mut causes = vec![];
    let mut gives = vec![];

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Causes:" => {
                causes = parse_status_effects(h4.as_node())?;
            }
            "Gives:" => {
                gives = parse_status_effects(h4.as_node())?;
            }
            x => panic!("{}", x),
        }
    }

    Ok(CodexSkill {
        name: node_to_text(name.as_node()),
        icon: parse_icon(icon.as_node())?,
        description: node_to_text(description.as_node()),
        tier: parse_tier(tier.as_node())?,
        causes,
        gives,
    })
}
