use std::path::Path;

use ornaguide_rs::{
    error::Error,
    guide::{CachedGuide, Guide},
    items::{Item, RawItem},
};

/// Try to parse and convert every raw item to an item. For every conversion that fails, get the
/// error string and concatenate them into a giant multi-line string.
fn parse_items(raw_items: &[RawItem]) -> Result<(), Error> {
    let error_string = raw_items
        .iter()
        .cloned()
        .map(Item::try_from)
        .filter_map(|result| match result {
            Ok(_) => None,
            Err(err) => Some(format!("{}", err)),
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !error_string.is_empty() {
        Err(Error::Misc(error_string))
    } else {
        Ok(())
    }
}

fn parse() -> Result<(), Error> {
    let mut cache = CachedGuide::from_directory(Path::new("./jsons/"))?;

    parse_items(cache.fetch_items()?)?;

    Ok(())
}

fn main() {
    match parse() {
        Ok(_) => println!("OK"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
