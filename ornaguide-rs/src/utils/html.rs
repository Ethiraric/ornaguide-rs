use kuchiki::{
    iter::{Descendants, Elements, Select},
    ElementData, NodeData, NodeDataRef, NodeRef,
};
use reqwest::Url;

use crate::error::{Error, Kind};

/// Select the node that matches the selector and that is a descendant of `node`. `from_name` is a
/// name to be displayed on the error message.
pub fn descend_iter(
    node: &NodeRef,
    selector: &str,
    from_name: &str,
) -> Result<Select<Elements<Descendants>>, Error> {
    node.select(selector).map_err(|()| {
        Kind::HTMLParsingError(format!("Failed to find \"{selector}\" in {from_name}")).into()
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
        Kind::HTMLParsingError(format!("Failed to find \"{selector}\" in {from_name}")).into()
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
                Kind::HTMLParsingError(format!("Failed to find {attr} in {node_name}")).into()
            })
            .map(std::string::ToString::to_string)
    } else {
        Err(Kind::HTMLParsingError(format!("Failed to get attributes from {node_name}")).into())
    }
}

/// Return the list of attributes form an HTML node.
/// I am very unsure of what I'm doing here, but it works for what I use it for.
pub fn list_attributes_form_node(node: &NodeRef, node_name: &str) -> Result<Vec<String>, Error> {
    if let NodeData::Element(ElementData {
        name: _,
        attributes,
        template_contents: _,
    }) = node.data()
    {
        let attributes = attributes.borrow();
        Ok(attributes
            .map
            .iter()
            .flat_map(|(_, value)| value.value.split(' ').map(str::to_string))
            .collect())
    } else {
        Err(Kind::HTMLParsingError(format!("Failed to get attributes from {node_name}")).into())
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
