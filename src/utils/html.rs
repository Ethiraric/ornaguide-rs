use kuchiki::{
    iter::{Descendants, Elements, Select},
    ElementData, NodeData, NodeDataRef, NodeRef,
};
use reqwest::Url;

use crate::error::Error;

/// Select the node that matches the selector and that is a descendant of `node`. `from_name` is a
/// name to be displayed on the error message.
pub fn descend_iter(
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
pub fn descend_to(
    node: &NodeRef,
    selector: &str,
    from_name: &str,
) -> Result<NodeDataRef<ElementData>, Error> {
    try_descend_to(node, selector, from_name)?.ok_or_else(|| {
        Error::HTMLParsingError(format!("Failed to find \"{}\" in {}", selector, from_name))
    })
}

/// Try to select the node that matches the selector and that is a descendant of `node`.
/// `from_name` is a name to be displayed on the error message.
pub fn try_descend_to(
    node: &NodeRef,
    selector: &str,
    from_name: &str,
) -> Result<Option<NodeDataRef<ElementData>>, Error> {
    Ok(descend_iter(node, selector, from_name)?.next())
}

/// Retrieve an attribute from an HTML node.
pub fn get_attribute_from_node(
    node: &NodeRef,
    attr: &str,
    node_name: &str,
) -> Result<String, Error> {
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

/// Get the text contained in the node.
pub fn node_to_text(node: &NodeRef) -> String {
    node.text_contents().trim().to_string()
}

/// Strip the host and `/static/img` from the given URL.
pub fn icon_url_to_path(url: &str) -> String {
    let url = Url::parse(url).unwrap();
    let mut path = url.path();
    if path.starts_with("/static/img") {
        path = &path[11..];
    }
    if path.starts_with('/') {
        path = &path[1..];
    }
    path.to_string()
}

/// Parse the icon of an item.
/// Returns an URL path, without the host and the "/static".
pub fn parse_icon(node: &NodeRef) -> Result<String, Error> {
    Ok(icon_url_to_path(&get_attribute_from_node(
        descend_to(node, "img", "icon-node")?.as_node(),
        "src",
        "img icon node",
    )?))
}
