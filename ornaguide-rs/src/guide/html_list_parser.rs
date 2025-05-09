use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeRef};

use crate::{
    error::{Error, Kind},
    utils::html::descend_to,
};

/// An entry on the list.
pub struct Entry {
    /// Id of the element in the row.
    pub id: u32,
    /// Value of the element (the text).
    pub value: String,
}

/// A parsed table.
pub struct ParsedTable {
    /// The entries that were parsed.
    pub entries: Vec<Entry>,
    /// The number of entries that are total, including all pages, according to the paginator.
    pub number_entries: usize,
}

fn tr_to_entry(tr: &NodeRef) -> Result<Entry, Error> {
    let a = descend_to(tr, "a", "row")?;
    if let NodeData::Element(ElementData {
        name: _,
        attributes,
        template_contents: _,
    }) = a.as_node().data()
    {
        let attributes = attributes.borrow();
        let url = attributes
            .get("href")
            .ok_or_else(|| Kind::HTMLParsingError("Failed to find href in a".to_string()))?;
        let url = if let Some(x) = url.find('?') {
            url.split_at(x).0
        } else {
            url
        };
        if !url.ends_with("/change/") {
            return Err(Kind::HTMLParsingError(format!(
                "a URL doesn't end with \"/change/\": {url}"
            ))
            .into());
        }

        // Trim "/change/" from the end.
        let url = url.split_at(url.len() - "/change/".len()).0;
        if url.ends_with('/') {
            return Err(Kind::HTMLParsingError("a URL has a duplicate '/'".to_string()).into());
        }
        // Get the id.
        let id = if let Some(idx) = url.rfind('/') {
            url.split_at(idx + 1).1
        } else {
            return Err(Kind::HTMLParsingError(
                "a URL doesn't contain an expected '/'".to_string(),
            )
            .into());
        };

        // Return entry.
        Ok(Entry {
            id: id.parse()?,
            value: a.text_contents(),
        })
    } else {
        Err(Kind::HTMLParsingError("Failed to convert a node to data".to_string()).into())
    }
}

pub fn parse_list_html(contents: &str) -> Result<ParsedTable, Error> {
    let html = parse_html().one(contents);

    let table = descend_to(&html, "#result_list", "html")?;
    let tbody = descend_to(table.as_node(), "tbody", "html root node")?;
    let paginator = descend_to(&html, ".paginator", "html")?;
    let paginator_text = paginator.as_node().text_contents();
    let number_entries = paginator_text
        .split_whitespace()
        .map_while(|s| if s == "..." { Some(0) } else { s.parse().ok() })
        .last()
        .ok_or_else(|| {
            Kind::HTMLParsingError(format!("Failed to get parsing from: {paginator_text}"))
        })?;

    Ok(ParsedTable {
        entries: tbody
            .as_node()
            .select("tr")
            .map_err(|()| Kind::HTMLParsingError("Failed to find tr in tbody".to_string()))?
            .map(|tr| tr_to_entry(tr.as_node()))
            .collect::<Result<Vec<_>, _>>()?,
        number_entries,
    })
}
