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
