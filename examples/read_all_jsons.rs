use ornaguide_rs::{
    error::Error,
    items::{ArmorItem, Item},
    raw_items::RawItems,
};

fn emit_warning(warning_name: &str, item_name: &str, warning: &str) {
    println!("warning: {}: {} ({})", item_name, warning, warning_name);
}

#[allow(dead_code)]
fn warn_item_no_source(items: &RawItems) {
    for item in items.items.iter() {
        let arena = item.arena;
        let quest = item.quests.is_some() && !item.quests.as_ref().unwrap().is_empty();
        let dropped = item.dropped_by.is_some() && !item.dropped_by.as_ref().unwrap().is_empty();
        if !arena && !quest && !dropped {
            emit_warning("item_no_source", item.name.as_str(), "item has no source");
        }
        // if !((arena && !quest && !dropped)
        //     || (!arena && quest && !dropped)
        //     || (!arena && !quest && dropped))
        // {
        //     println!(
        //         "{}: {} {} {}",
        //         item.name,
        //         if arena { "arena" } else { "." },
        //         if quest { "quest" } else { "." },
        //         if dropped { "dropped" } else { "." }
        //     );
        // }
    }
}

#[allow(dead_code)]
fn warn_weapon_no_category(items: &RawItems) {
    for item in items.items.iter() {
        if item.type_ == "Weapon" && item.category.is_none() {
            emit_warning(
                "weapon_no_category",
                item.name.as_str(),
                "weapon has no category",
            );
        }
    }
}

#[allow(dead_code)]
fn warn_weapon_no_materials(items: &RawItems) {
    for item in items.items.iter() {
        if item.type_ == "Weapon" && item.materials.is_none() {
            emit_warning(
                "weapon_no_materials",
                item.name.as_str(),
                "weapon has no materials",
            );
        }
    }
}

#[allow(dead_code)]
fn warn_legs_no_materials(items: &RawItems) {
    for item in items.items.iter() {
        if item.type_ == "Legs" && item.materials.is_none() {
            emit_warning(
                "legs_no_materials",
                item.name.as_str(),
                "legs has no materials",
            );
        }
    }
}

#[allow(dead_code)]
fn warn_morri_item_materials(items: &RawItems) {
    for item in items.items.iter() {
        if item.type_ == "Legs"
            || item.type_ == "Off-hand"
            || item.type_ == "Weapon"
            || item.type_ == "Armor"
        {
            if let Some(dropped_by) = &item.dropped_by {
                if let Some(mats) = &item.materials {
                    if dropped_by
                        .iter()
                        .any(|dropped_by| dropped_by.name == "Arisen Morrigan")
                    {
                        if let [mat1, mat2] = mats.as_slice() {
                            if (mat1.name != "Cursed Ortanite" && mat2.name != "Cursed Ortanite")
                                || (mat1.name != "Greater Soul" && mat2.name != "Greater Soul")
                            {
                                emit_warning(
                                    "morri_item_materials",
                                    item.name.as_str(),
                                    format!(
                                        "invalid Arisen Morrigan item materials: {:?}",
                                        mats.iter()
                                            .map(|mat| mat.name.as_str())
                                            .collect::<Vec<_>>()
                                    )
                                    .as_str(),
                                );
                            }
                        } else {
                            emit_warning(
                                "morri_item_materials",
                                item.name.as_str(),
                                format!(
                                    "missing or extra Arisen Morrigan item materials: {:?}",
                                    mats.iter().map(|mat| mat.name.as_str()).collect::<Vec<_>>()
                                )
                                .as_str(),
                            );
                        }
                    } else if dropped_by
                        .iter()
                        .any(|dropped_by| dropped_by.name == "The Morrigan")
                    {
                        if let [mat1] = mats.as_slice() {
                            if mat1.name != "Cursed Ortanite" {
                                emit_warning(
                                    "morri_item_materials",
                                    item.name.as_str(),
                                    format!(
                                        "invalid The Morrigan item material: {:?}",
                                        mats.iter()
                                            .map(|mat| mat.name.as_str())
                                            .collect::<Vec<_>>()
                                    )
                                    .as_str(),
                                );
                            }
                        } else {
                            emit_warning(
                                "morri_item_materials",
                                item.name.as_str(),
                                format!(
                                    "missing or extra The Morrigan item materials: {:?}",
                                    mats.iter().map(|mat| mat.name.as_str()).collect::<Vec<_>>()
                                )
                                .as_str(),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn read_all_jsons() -> Result<(), Error> {
    let raw_items = RawItems::parse_from_file("jsons/item.json")?;
    println!("Read {} items.", raw_items.items.len());
    let _items = raw_items
        .items
        .iter()
        .cloned()
        .map(Item::try_from)
        .collect::<Result<Vec<_>, Error>>()?;
    // non_unique_source(&items);
    let mut types = raw_items
        .items
        .iter()
        .map(|item| item.type_.clone())
        .collect::<Vec<_>>();
    types.sort();
    types.dedup();
    println!("Item types: {:#?}", types);
    let armors = raw_items
        .items
        .into_iter()
        .filter_map(|item| match item.type_.as_str() {
            "Armor" => Some(ArmorItem::try_from(item.clone()).map_err(|e| (item, e))),
            _ => None,
        })
        .collect::<Vec<_>>();
    println!("Armors: {:#?}", armors.len());
    for result in armors.iter() {
        match result {
            Ok(_) => {}
            Err((item, e)) => eprintln!("Item: {}: {}", item.name, e),
        }
    }
    Ok(())
}

fn main() {
    read_all_jsons().unwrap_or_else(|err| eprintln!("{}", err))
}
