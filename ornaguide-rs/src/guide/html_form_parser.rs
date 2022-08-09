use std::ops::Deref;

use kuchiki::{parse_html, traits::TendrilSink, Attributes, ElementData, NodeData, NodeRef};

use crate::{error::Error, utils::html::descend_to};

/// Parsed form, csrf token included.
#[derive(Debug, Default)]
pub struct ParsedForm {
    /// List of (key, value) pairs that were extracted from the form. The key component is the name
    /// of the field.
    pub fields: Vec<(String, String)>,
    /// The csrf token to pass when editing the item.
    pub csrfmiddlewaretoken: String,
}

/// Find the csrfmiddlewaretoken in the form.
fn find_csrfmiddlewaretoken(form: &NodeRef) -> String {
    let node = descend_to(form, "[name=\"csrfmiddlewaretoken\"]", "form").unwrap();
    let node = node.as_node();

    if let NodeData::Element(ElementData {
        name,
        attributes,
        template_contents: _,
    }) = node.data()
    {
        assert_eq!(&name.local, "input");
        let attributes = attributes.borrow();
        attributes.get("value").unwrap().to_string()
    } else {
        panic!("Failed to find csrfmiddlewaretoken");
    }
}

/// Find the value of the given `input` field.
fn find_input_field_value(attrs: &Attributes) -> Option<String> {
    let type_ = attrs.get("type").unwrap().to_string();
    match type_.as_str() {
        "text" | "number" => Some(attrs.get("value").unwrap_or_default().to_string()),
        "checkbox" => {
            if attrs.get("checked").is_some() {
                Some("on".to_string())
            } else {
                None
            }
        }
        _ => panic!("Unknown input type: {}", type_),
    }
}

fn find_select_field_value(select: &NodeRef) -> Vec<String> {
    let mut values = Vec::new();
    for child in select.children() {
        if let NodeData::Element(ElementData {
            name,
            attributes,
            template_contents: _,
        }) = child.data()
        {
            if &name.local.to_string() == "option" {
                let attributes = attributes.borrow();
                if attributes.contains("selected") {
                    values.push(attributes.get("value").unwrap().to_string());
                }
            }
        }
    }
    values
}

fn find_textarea_field_value(select: &NodeRef) -> String {
    select.text_contents()
}

/// Find the value of the given field in the form and add it to `fields`.
///
/// All fields have an id that is `#id_{name}` with `name` the name of the field (`tier`,
/// `image_name`, ...).
/// The function tries to find it in the page and, depending on the value type, calls the
/// appropriate helper function to push the value(s) into `fields`.
fn add_field_value(
    form: &NodeRef,
    field_name: &str,
    fields: &mut Vec<(String, String)>,
) -> Result<(), Error> {
    let html_id = format!("#id_{}", field_name);
    let field_node = form
        .select(&html_id)
        .map_err(|()| {
            Error::HTMLParsingError(format!(
                "Failed to select html id {} in guide form parsing",
                html_id
            ))
        })?
        .next()
        .ok_or_else(|| {
            Error::HTMLParsingError(format!("No node {} in guide form parsing", html_id))
        })?;
    let field_node = field_node.as_node();
    if let NodeData::Element(ElementData {
        name,
        attributes,
        template_contents: _,
    }) = field_node.data()
    {
        let tag = name.local.to_string();
        match tag.deref() {
            "input" => {
                if let Some(value) = find_input_field_value(&attributes.borrow()) {
                    fields.push((field_name.to_string(), value))
                }
            }
            "select" => {
                let values = find_select_field_value(field_node);
                for value in values {
                    fields.push((field_name.to_string(), value));
                }
            }
            "textarea" => {
                fields.push((
                    field_name.to_string(),
                    find_textarea_field_value(field_node),
                ));
            }
            _ => panic!("Unknown node tag for field {}: {}", field_name, tag.deref()),
        };
    } else {
        panic!("Failed to find node with id {}", html_id)
    }
    Ok(())
}

/// Extract given fields from an HTML page.
fn parse_html_form(
    contents: &str,
    form_root_name: &str,
    field_names: &[&str],
) -> Result<ParsedForm, Error> {
    let html = parse_html().one(contents);

    let form = descend_to(&html, form_root_name, "html")?;
    let form = form.as_node();

    let mut fields = Vec::new();
    for field_name in field_names {
        add_field_value(form, field_name, &mut fields)?;
    }

    let csrfmiddlewaretoken = find_csrfmiddlewaretoken(form);

    Ok(ParsedForm {
        fields,
        csrfmiddlewaretoken,
    })
}

/// Extract given fields from an admin item change HTML page.
pub fn parse_item_html(contents: &str, field_names: &[&str]) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#item_form", field_names)
}

/// Extract given fields from an admin monster change HTML page.
pub fn parse_monster_html(contents: &str, field_names: &[&str]) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#monster_form", field_names)
}

/// Extract given fields from an admin skill change HTML page.
pub fn parse_skill_html(contents: &str, field_names: &[&str]) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#skill_form", field_names)
}

/// Extract given fields from an admin pet add HTML page.
pub fn parse_pet_html(contents: &str, field_names: &[&str]) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#pet_form", field_names)
}

/// Extract given fields from an admin spawn add HTML page.
pub fn parse_spawn_html(contents: &str) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#spawn_form", &[])
}

/// Extract given fields from an admin status effect add HTML page.
pub fn parse_status_effect_html(contents: &str) -> Result<ParsedForm, Error> {
    parse_html_form(contents, "#statuseffect_form", &[])
}

/// Names of the fields in the admin item change page.
pub(crate) const ITEM_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "type",
    "image_name",
    "description",
    "notes",
    "hp",
    "hp_affected_by_quality",
    "mana",
    "mana_affected_by_quality",
    "attack",
    "attack_affected_by_quality",
    "magic",
    "magic_affected_by_quality",
    "defense",
    "defense_affected_by_quality",
    "resistance",
    "resistance_affected_by_quality",
    "dexterity",
    "dexterity_affected_by_quality",
    "ward",
    "ward_affected_by_quality",
    "crit",
    "crit_affected_by_quality",
    "foresight",
    "view_distance",
    "follower_stats",
    "follower_act",
    "status_infliction",
    "status_protection",
    "mana_saver",
    "has_slots",
    "base_adornment_slots",
    "rarity",
    "element",
    "equipped_by",
    "two_handed",
    "orn_bonus",
    "gold_bonus",
    "drop_bonus",
    "spawn_bonus",
    "exp_bonus",
    "boss",
    "arena",
    "category",
    "causes",
    "cures",
    "gives",
    "prevents",
    "materials",
    "price",
    "ability",
    "potion_effectiveness",
];

/// Names of the fields in the admin monster change page.
pub(crate) const MONSTER_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "family",
    "image_name",
    "boss",
    "level",
    "hp",
    "notes",
    "spawns",
    "weak_to",
    "resistant_to",
    "immune_to",
    "immune_to_status",
    "vulnerable_to_status",
    "drops",
    "skills",
];

/// Names of the fields in the admin skill change page.
pub(crate) const SKILL_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "type",
    "is_magic",
    "mana_cost",
    "description",
    "element",
    "offhand",
    "cost",
    "bought",
    "skill_power",
    "strikes",
    "modifier_min",
    "modifier_max",
    "extra",
    "buffed_by",
    "causes",
    "cures",
    "gives",
];

/// Names of the fields in the admin pet change page.
pub(crate) const PET_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "image_name",
    "description",
    "attack",
    "heal",
    "buff",
    "debuff",
    "spell",
    "protect",
    "cost",
    "cost_type",
    "limited",
    "limited_details",
    "skills",
];
