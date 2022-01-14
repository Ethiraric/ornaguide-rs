use ornaguide_rs::{error::Error, raw_items::RawItems};

fn read_all_jsons() -> Result<(), Error> {
    let items = RawItems::parse_from_file("jsons/item.json")?;
    println!("Read {} items.", items.items.len());
    let mut types = items
        .items
        .iter()
        .map(|item| item.type_.clone())
        .collect::<Vec<_>>();
    types.sort();
    types.dedup();
    println!("Item types: {:#?}", types);
    Ok(())
}

fn main() {
    read_all_jsons().unwrap_or_else(|err| eprintln!("{}", err))
}
