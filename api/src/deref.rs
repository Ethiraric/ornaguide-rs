use ornaguide_rs::{
    data::OrnaData,
    error::{ErrorKind},
};

use crate::error::{Error, ToErrorable};

/// Dereference an ID contained in a `Value::Number` and replace the node with a `String`.
/// If a type is found to not be of the required type, an error is returned.
/// If the dereference function fails, an error is returned.
fn deref_id<'a, F>(json: &mut serde_json::Value, id_to_str: F, kind: &str) -> Result<(), Error>
where
    F: FnOnce(u64) -> Option<&'a str>,
{
    if let Some(id) = json.as_u64() {
        if let Some(s) = id_to_str(id) {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(ErrorKind::Misc(format!("Failed to find {} #{}", kind, id)).into_err())
                .to_internal_server_error()
        }
    } else {
        Err(ErrorKind::Misc(format!("Json node {} is not of json type u64", kind)).into_err())
            .to_internal_server_error()
    }
}

/// Dereference an array of IDs containing `Value::Number`s and replace the nodes with `String`s.
/// If a type is found to not be of the required type, an error is returned.
/// If the dereference function fails, an error is returned.
fn deref_ids<'a, F>(json: &mut serde_json::Value, id_to_str: F, kind: &str) -> Result<(), Error>
where
    F: Fn(u64) -> Option<&'a str>,
{
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = id_to_str(id) {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(ErrorKind::Misc(format!("Failed to find {} #{}", kind, id)).into_err())
                            .to_internal_server_error()
                    }
                } else {
                    Err(
                        ErrorKind::Misc(format!("Json node {} is not of json type u64", kind))
                            .into_err(),
                    )
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(ErrorKind::Misc(format!("Array of {} is not of json type array", kind)).into_err())
            .to_internal_server_error()
    }
}

/// Replace the skill type ID in `json` with the skill type name.
pub fn deref_skill_type(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .static_
                .skill_types
                .iter()
                .find(|type_| (type_.id as u64) == id)
                .map(|type_| type_.name.as_str())
        },
        "skill type",
    )
}

/// Replace the item type ID in `json` with the item type name.
pub fn deref_item_type(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .static_
                .item_types
                .iter()
                .find(|type_| (type_.id as u64) == id)
                .map(|type_| type_.name.as_str())
        },
        "item type",
    )
}

/// Replace the item category ID in `json` with the item category name.
pub fn deref_item_category(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .static_
                .item_categories
                .iter()
                .find(|category| (category.id as u64) == id)
                .map(|category| category.name.as_str())
        },
        "item category",
    )
}

/// Replace the element ID in `json` with the element name.
pub fn deref_element(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .static_
                .elements
                .iter()
                .find(|element| (element.id as u64) == id)
                .map(|element| element.name.as_str())
        },
        "element",
    )
}

/// Replace the skill ID in `json` with the skill name.
pub fn deref_skill(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .skills
                .skills
                .iter()
                .find(|skill| (skill.id as u64) == id)
                .map(|skill| skill.name.as_str())
        },
        "skill",
    )
}

/// Replace the monster familiy ID in `json` with the monster familiy name.
pub fn deref_monster_family(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_id(
        json,
        |id| {
            data.guide
                .static_
                .monster_families
                .iter()
                .find(|family| (family.id as u64) == id)
                .map(|family| family.name.as_str())
        },
        "monster family",
    )
}

/// Replace the status effects IDs in `json` with the status effects' names.
pub fn deref_status_effects(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .static_
                .status_effects
                .iter()
                .find(|status| (status.id as u64) == id)
                .map(|status| status.name.as_str())
        },
        "status effect",
    )
}

/// Replace the monster IDs in `json` with the monsters' names.
pub fn deref_monsters(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .monsters
                .monsters
                .iter()
                .find(|status| (status.id as u64) == id)
                .map(|status| status.name.as_str())
        },
        "monster",
    )
}

/// Replace the equipped_by IDs in `json` with the equipped_bys' names.
pub fn deref_equipped_bys(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .static_
                .equipped_bys
                .iter()
                .find(|equipped_by| (equipped_by.id as u64) == id)
                .map(|equipped_by| equipped_by.name.as_str())
        },
        "equipped_by",
    )
}

/// Replace the item IDs in `json` with the items' names.
pub fn deref_items(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .items
                .items
                .iter()
                .find(|item| (item.id as u64) == id)
                .map(|item| item.name.as_str())
        },
        "item",
    )
}

/// Replace the skill IDs in `json` with the skills' names.
pub fn deref_skills(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .skills
                .skills
                .iter()
                .find(|skill| (skill.id as u64) == id)
                .map(|skill| skill.name.as_str())
        },
        "skill",
    )
}

/// Replace the spawn IDs in `json` with the spawns' names.
pub fn deref_spawns(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .static_
                .spawns
                .iter()
                .find(|spawn| (spawn.id as u64) == id)
                .map(|spawn| spawn.name.as_str())
        },
        "spawn",
    )
}

/// Replace the element IDs in `json` with the elements' names.
pub fn deref_elements(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    deref_ids(
        json,
        |id| {
            data.guide
                .static_
                .elements
                .iter()
                .find(|element| (element.id as u64) == id)
                .map(|element| element.name.as_str())
        },
        "element",
    )
}
