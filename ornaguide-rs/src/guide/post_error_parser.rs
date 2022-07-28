use itertools::Itertools;
use kuchiki::{parse_html, traits::TendrilSink};

use crate::{
    error::Error,
    utils::html::{descend_iter, descend_to, list_attributes_form_node, node_to_text},
};

/// When receiving the response to a form POST request, parse the response and look for errors.
pub fn parse_post_error_html(url: &str, contents: &str, form_root_name: &str) -> Result<(), Error> {
    let html = parse_html().one(contents);
    let form = match descend_to(&html, form_root_name, "html") {
        Ok(x) => x,
        // If we fail to find the root name, it means that the request succeeded.
        Err(_) => return Ok(()),
    };

    let form = form.as_node();
    let generic_error = node_to_text(descend_to(form, ".errornote", "post error form")?.as_node());

    // Iterate over all `errors` nodes.
    let specific_errors = descend_iter(form, ".errors", "post error form")?
        .flat_map(|node| -> Result<_, Error> {
            // List their attributes and keep only those starting with `field-`. Those correspond
            // to the field nodes, where input is expected.
            let attrs = list_attributes_form_node(node.as_node(), "errors")?
                .into_iter()
                .filter_map(|name| name.strip_prefix("field-").map(str::to_string))
                .collect_vec();

            // In the node, list errors.
            let errors = descend_iter(node.as_node(), ".errorlist", "errors")?
                .flat_map(|node| descend_iter(node.as_node(), "li", "errorlist").unwrap())
                .map(|node| node_to_text(node.as_node()))
                .collect_vec();

            // Combine the 2, into an `attrs.len()*errors.len()` vec.
            Ok(attrs
                .into_iter()
                .flat_map(|attr| errors.iter().map(move |err| format!("{}: {}", attr, err)))
                .collect_vec())
        })
        .flatten()
        .collect_vec();

    Err(Error::GuidePostFormError(
        url.to_string(),
        generic_error,
        specific_errors,
    ))
}
