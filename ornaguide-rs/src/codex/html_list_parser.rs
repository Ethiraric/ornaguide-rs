use kuchiki::{parse_html, traits::TendrilSink, NodeRef};

use crate::{
    error::{Error, ErrorKind},
    utils::html::{descend_iter, descend_to, get_attribute_from_node, node_to_text},
};

/// An entry on the list.
#[derive(Debug, Default)]
pub struct Entry {
    /// Value of the element (the text).
    pub value: String,
    /// Meta information about the entry.
    pub meta: Option<String>,
    /// Tier of the element.
    pub tier: u32,
    /// Uri to the element.
    pub uri: String,
}

/// A parsed list.
#[derive(Debug)]
pub struct ParsedList {
    /// The entries that were parsed.
    pub entries: Vec<Entry>,
    /// Whether there was a next page.
    pub has_next_page: bool,
}

/// Create an entry from an HTML node.
fn node_to_entry(node: &NodeRef) -> Result<Entry, Error> {
    let all_contents = node.text_contents();
    let all_contents = all_contents.trim();
    let mut entry = Entry::default();

    let it = node.children().filter(|n| n.as_element().is_some()).skip(1); // Skip over image.
    let mut it = it.peekable();

    if let Some(name_node) = it.next() {
        entry.value = node_to_text(&name_node);
    } else {
        return Err(ErrorKind::HTMLParsingError(format!(
            "Failed to find name in codex entry: {:#?}",
            all_contents
        ))
        .into());
    }

    if let Some(meta_node) = it.peek() {
        if let Ok(klass) = get_attribute_from_node(meta_node, "class", "") {
            if klass == "codex-entries-entry-meta" {
                entry.meta = Some(node_to_text(meta_node).trim().to_string());
                it.next();
            }
        }
    }

    if let Some(tier_node) = it.next() {
        let tier_str = node_to_text(&tier_node);
        let tier_str = tier_str.trim();
        if let Some(c) = tier_str.chars().next() {
            if c == '★' {
                let mut chars = tier_str.chars();
                chars.next();
                entry.tier = chars.as_str().trim().parse()?;
            } else {
                return Err(ErrorKind::HTMLParsingError(format!(
                    "Failed to find the star in tier in codex entry field: {:#?}",
                    tier_str
                ))
                .into());
            }
        } else {
            return Err(ErrorKind::HTMLParsingError(format!(
                "The tier string is empty in: {:#?}",
                node_to_text(&tier_node)
            ))
            .into());
        }
    } else {
        return Err(ErrorKind::HTMLParsingError(format!(
            "Failed to find tier in codex entry: {:#?}",
            all_contents
        ))
        .into());
    }

    entry.uri = get_attribute_from_node(descend_to(node, "a", "entry")?.as_node(), "href", "a")?;

    Ok(entry)
}

/// Parses a page from `playorna.com` and returns the list of entries that were found and their
/// associated tiers.
pub fn parse_html_codex_list(contents: &str) -> Result<ParsedList, Error> {
    let html = parse_html().one(contents);

    let entries = descend_to(&html, ".codex-entries", "html")?;
    let pagination = descend_to(&html, ".pagination", "html")?;

    Ok(ParsedList {
        entries: descend_iter(entries.as_node(), ".codex-entries-entry", "entries node")?
            .map(|entry| node_to_entry(entry.as_node()))
            .collect::<Result<Vec<_>, _>>()?,
        has_next_page: pagination.text_contents().contains("Next page"),
    })
}
