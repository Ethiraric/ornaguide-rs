use ornaguide_rs::{data::OrnaData, error::Error as OError};

use crate::error::{Error, ToErrorable};

/// Replace the skill type ID in `json` with the skill type name.
pub fn deref_skill_type(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .static_
            .skill_types
            .iter()
            .find(|type_| (type_.id as u64) == id)
            .map(|type_| type_.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!("Failed to find skill type #{}", id)))
                .to_internal_server_error()
        }
    } else {
        Err(OError::Misc(
            "Skill type is not of json type u64".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the item type ID in `json` with the item type name.
pub fn deref_item_type(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .static_
            .item_types
            .iter()
            .find(|type_| (type_.id as u64) == id)
            .map(|type_| type_.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!("Failed to find item type #{}", id)))
                .to_internal_server_error()
        }
    } else {
        Err(OError::Misc(
            "Item type is not of json type u64".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the item category ID in `json` with the item category name.
pub fn deref_item_category(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .static_
            .item_categories
            .iter()
            .find(|category| (category.id as u64) == id)
            .map(|category| category.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!(
                "Failed to find item category #{}",
                id
            )))
            .to_internal_server_error()
        }
    } else {
        Err(OError::Misc(
            "Item category is not of json type u64".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the element ID in `json` with the element name.
pub fn deref_element(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .static_
            .elements
            .iter()
            .find(|element| (element.id as u64) == id)
            .map(|element| element.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!("Failed to find element #{}", id))).to_internal_server_error()
        }
    } else {
        Err(OError::Misc("Element is not of json type u64".to_string())).to_internal_server_error()
    }
}

/// Replace the skill ID in `json` with the skill name.
pub fn deref_skill(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .skills
            .skills
            .iter()
            .find(|skill| (skill.id as u64) == id)
            .map(|skill| skill.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!("Failed to find skill #{}", id))).to_internal_server_error()
        }
    } else {
        Err(OError::Misc("Skill is not of json type u64".to_string())).to_internal_server_error()
    }
}

/// Replace the monster familiy ID in `json` with the monster familiy name.
pub fn deref_monster_family(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(id) = json.as_u64() {
        if let Some(s) = data
            .guide
            .static_
            .monster_families
            .iter()
            .find(|family| (family.id as u64) == id)
            .map(|family| family.name.as_str())
        {
            *json = serde_json::Value::String(s.to_string());
            Ok(())
        } else {
            Err(OError::Misc(format!(
                "Failed to find monster family #{}",
                id
            )))
            .to_internal_server_error()
        }
    } else {
        Err(OError::Misc(
            "Monster family is not of json type u64".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the status effects IDs in `json` with the status effects' names.
pub fn deref_status_effects(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .static_
                        .status_effects
                        .iter()
                        .find(|status| (status.id as u64) == id)
                        .map(|status| status.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!(
                            "Failed to find status effect #{}",
                            id
                        )))
                        .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "A status effect is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Status effects array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the monster IDs in `json` with the monsters' names.
pub fn deref_monsters(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .monsters
                        .monsters
                        .iter()
                        .find(|monster| (monster.id as u64) == id)
                        .map(|monster| monster.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find monster #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "A monster id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Monster array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the equipped_by IDs in `json` with the equipped_bys' names.
pub fn deref_equipped_bys(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .static_
                        .equipped_bys
                        .iter()
                        .find(|equipped_by| (equipped_by.id as u64) == id)
                        .map(|equipped_by| equipped_by.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find equipped_by #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "An equipped_by id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Equipped bys array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the item IDs in `json` with the items' names.
pub fn deref_items(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .items
                        .items
                        .iter()
                        .find(|item| (item.id as u64) == id)
                        .map(|item| item.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find item #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "An item id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Item array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the skill IDs in `json` with the skills' names.
pub fn deref_skills(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .skills
                        .skills
                        .iter()
                        .find(|skill| (skill.id as u64) == id)
                        .map(|skill| skill.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find skill #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "An skill id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Skill array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the spawn IDs in `json` with the spawns' names.
pub fn deref_spawns(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .static_
                        .spawns
                        .iter()
                        .find(|spawn| (spawn.id as u64) == id)
                        .map(|spawn| spawn.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find spawn #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "An spawn id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Spawn array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}

/// Replace the element IDs in `json` with the elements' names.
pub fn deref_elements(json: &mut serde_json::Value, data: &OrnaData) -> Result<(), Error> {
    if let Some(ids) = json.as_array() {
        let names = ids
            .iter()
            .map(|id| {
                if let Some(id) = id.as_u64() {
                    if let Some(s) = data
                        .guide
                        .static_
                        .elements
                        .iter()
                        .find(|element| (element.id as u64) == id)
                        .map(|element| element.name.as_str())
                    {
                        Ok(serde_json::Value::String(s.to_string()))
                    } else {
                        Err(OError::Misc(format!("Failed to find element #{}", id)))
                            .to_internal_server_error()
                    }
                } else {
                    Err(OError::Misc(
                        "An element id is not of json type u64".to_string(),
                    ))
                    .to_internal_server_error()
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        *json = serde_json::Value::Array(names);
        Ok(())
    } else {
        Err(OError::Misc(
            "Spawn array is not of json type array".to_string(),
        ))
        .to_internal_server_error()
    }
}
