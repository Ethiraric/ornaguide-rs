use ornaguide_rs::{error::Error, items::Armor, raw_items::RawItems};

#[allow(dead_code)]
fn non_unique_source(items: &RawItems) {
    for item in items.items.iter() {
        let arena = item.arena;
        let quest = item.quests.is_some() && !item.quests.as_ref().unwrap().is_empty();
        let dropped = item.dropped_by.is_some() && !item.dropped_by.as_ref().unwrap().is_empty();
        if !((arena && !quest && !dropped)
            || (!arena && quest && !dropped)
            || (!arena && !quest && dropped))
        {
            println!(
                "{}: {} {} {}",
                item.name,
                if arena { "arena" } else { "." },
                if quest { "quest" } else { "." },
                if dropped { "dropped" } else { "." }
            );
        }
    }
}

fn read_all_jsons() -> Result<(), Error> {
    let items = RawItems::parse_from_file("jsons/item.json")?;
    println!("Read {} items.", items.items.len());
    // non_unique_source(&items);
    let mut types = items
        .items
        .iter()
        .map(|item| item.type_.clone())
        .collect::<Vec<_>>();
    types.sort();
    types.dedup();
    println!("Item types: {:#?}", types);
    let armors = items
        .items
        .into_iter()
        .filter_map(|item| match item.type_.as_str() {
            "Armor" => Some(Armor::try_from(item.clone()).map_err(|e| (item, e))),
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
