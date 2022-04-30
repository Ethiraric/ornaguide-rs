use kuchiki::{parse_html, traits::TendrilSink, NodeRef};

use crate::{
    error::Error,
    utils::html::{descend_iter, descend_to, get_attribute_from_node},
};

/// An entry on the list.
#[derive(Debug)]
pub struct Entry {
    /// Value of the element (the text).
    pub value: String,
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
    let contents = node.text_contents();
    let contents = contents.trim();
    let uri = get_attribute_from_node(descend_to(node, "a", "entry")?.as_node(), "href", "a")?;
    if let Some(pos) = contents.find('\n') {
        let (name, tier_str) = contents.split_at(pos);
        let mut it = tier_str.trim().chars();
        it.next();
        let tier_str = it.as_str();
        Ok(Entry {
            value: name.to_string(),
            tier: tier_str.parse()?,
            uri,
        })
    } else {
        return Err(Error::HTMLParsingError(format!(
            "Failed to find '\\n' in codex skill: {:#?}",
            contents
        )));
    }
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
