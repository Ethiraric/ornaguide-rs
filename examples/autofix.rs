use std::path::Path;

use dotenv::dotenv;

use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, CachedGuide, Guide, OrnaAdminGuide},
    items::{ItemMaterial, RawItem},
};

fn set_item_materials_to<G: AdminGuide>(
    guide: &G,
    raw_item: &RawItem,
    materials: &[u32],
) -> Result<(), Error> {
    println!(
        "Setting materials of item #{} {} to {:?}",
        raw_item.id, raw_item.name, materials
    );
    let mut item = guide.admin_retrieve_item_by_id(raw_item.id)?;
    if item.materials.len() != materials.len()
        || !materials.iter().all(|mat| item.materials.contains(mat))
    {
        item.materials = materials.to_vec();
        guide.admin_save_item(item)
    } else {
        println!("Guide is okay. Please refresh cache.",);
        Ok(())
    }
}

fn add_item_materials<G: AdminGuide>(
    guide: &G,
    raw_item: &RawItem,
    materials: &[u32],
) -> Result<(), Error> {
    println!(
        "Adding materials {:?} to item #{} {}",
        materials, raw_item.id, raw_item.name
    );
    let mut item = guide.admin_retrieve_item_by_id(raw_item.id)?;
    let mut edited = false;
    for mat in materials {
        if !item.materials.contains(mat) {
            item.materials.push(*mat);
            edited = true;
        }
    }
    if edited {
        guide.admin_save_item(item)
    } else {
        println!(
            "Guide already has materials {:?} for item #{} {}. Please refresh cache.",
            materials, raw_item.id, raw_item.name
        );
        Ok(())
    }
}

/// Walk through most The Morrigan and Arisen Morrigan drops and check that they have the correct
/// materials. For those we check, it should be Cursed Ortanite for The Morrigan drops, or Cursed
/// Ortanite + Greater Souls for Arisen Morrigan drops.
#[allow(dead_code)]
fn fix_morri_item_materials<G: AdminGuide>(guide: &G, items: &[RawItem]) -> Result<(), Error> {
    let co = items
        .iter()
        .find(|item| item.name == "Cursed Ortanite")
        .unwrap()
        .id;
    let gs = items
        .iter()
        .find(|item| item.name == "Greater Soul")
        .unwrap()
        .id;

    for item in items
        .iter()
        .filter(|item| {
            // Filter out items that are not equippable (and not accessories).
            item.type_ == "Legs"
                || item.type_ == "Off-hand"
                || item.type_ == "Weapon"
                || item.type_ == "Armor"
                || item.type_ == "Head"
        })
        .filter(|item| {
            // Those items are annoying. Realm Katar requires Realm Ore and Cursed Ortanite (Arisen
            // Realm Katar is still Cursed Ortanite and Greater Soul). Morrigan's Scroll requires
            // Nightstone and Cursed Ortanite (though again, the Arisen version is still Cursed
            // Ortanite and Greater Soul).
            item.name != "Realm Katar" && item.name != "Morrigan's Scroll"
        })
        .filter(|item| {
            // Filter out items that are not dropped by The Morrigan or Arisen Morrigan.
            item.dropped_by.is_some()
                && item.dropped_by.as_ref().unwrap().iter().any(|monster| {
                    monster.name == "Arisen Morrigan" || monster.name == "The Morrigan"
                })
        })
    {
        if item
            .dropped_by
            .as_ref()
            .unwrap()
            .iter()
            .any(|monster| monster.name == "Arisen Morrigan")
        {
            match item.materials.as_deref() {
                Some([mat1, mat2]) => {
                    // Must have Greater Soul and Cursed Ortanite
                    if !((mat1.name == "Greater Soul" && mat2.name == "Cursed Ortanite")
                        || (mat2.name == "Greater Soul" && mat1.name == "Cursed Ortanite"))
                    {
                        set_item_materials_to(guide, item, &[co, gs])?;
                    }
                }
                _ => set_item_materials_to(guide, item, &[co, gs])?,
            }
        } else if item
            .dropped_by
            .as_ref()
            .unwrap()
            .iter()
            .any(|monster| monster.name == "The Morrigan")
        {
            match item.materials.as_deref() {
                // Must have Cursed Ortanite
                Some([ItemMaterial { id: _, ref name }]) if name == "Cursed Ortanite" => {}
                _ => set_item_materials_to(guide, item, &[co])?,
            }
        } else {
            panic!("Items here should be either a drop from The Morrigan or Arisen Morrigan")
        }
    }

    Ok(())
}

/// Walk through Lyonesse drops and check that they have Lyonite.
fn fix_lyonesse_items<G: AdminGuide>(guide: &G, items: &[RawItem]) -> Result<(), Error> {
    let lyonite = items.iter().find(|item| item.name == "Lyonite").unwrap().id;
    for item in items
        .iter()
        .filter(|item| {
            // Filter out items that are not equippable (and not accessories).
            item.type_ == "Legs"
                || item.type_ == "Off-hand"
                || item.type_ == "Weapon"
                || item.type_ == "Armor"
                || item.type_ == "Head"
        })
        .filter(|item| {
            // Filter out items that are not dropped by a Lyonesse monster.
            item.dropped_by.is_some()
                && item.dropped_by.as_ref().unwrap().iter().any(|monster| {
                    monster.name.contains("Lyonesse")
                        || monster.name == "Fallen Vanguard"
                        || monster.name == "King Gradlon"
                })
        })
    {
        if item.materials.is_none()
            || !item
                .materials
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.name == "Lyonite")
        {
            add_item_materials(guide, item, &[lyonite])?;
        }
    }

    Ok(())
}

/// Walk through Apollyon drops and check that they have Realm Ore.
fn fix_apollyon_items<G: AdminGuide>(guide: &G, items: &[RawItem]) -> Result<(), Error> {
    let ore = items
        .iter()
        .find(|item| item.name == "Realm Ore")
        .unwrap()
        .id;
    for item in items
        .iter()
        .filter(|item| {
            // Filter out items that are not equippable (and not accessories).
            item.type_ == "Legs"
                || item.type_ == "Off-hand"
                || item.type_ == "Weapon"
                || item.type_ == "Armor"
                || item.type_ == "Head"
        })
        .filter(|item| {
            // Filter out items that are not dropped by Apollyon.
            item.dropped_by.is_some()
                && item
                    .dropped_by
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|monster| monster.name == "Apollyon")
        })
    {
        if item.materials.is_none()
            || !item
                .materials
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.name == "Realm Ore")
        {
            add_item_materials(guide, item, &[ore])?;
        }
    }

    Ok(())
}

/// Walk through Mammon and Arisen Mammon drops and check that they have Ortanite.
fn fix_mammon_items<G: AdminGuide>(guide: &G, items: &[RawItem]) -> Result<(), Error> {
    let ortanite = items
        .iter()
        .find(|item| item.name == "Ortanite")
        .unwrap()
        .id;
    for item in items
        .iter()
        .filter(|item| {
            // Filter out items that are not equippable (and not accessories).
            item.type_ == "Legs"
                || item.type_ == "Off-hand"
                || item.type_ == "Weapon"
                || item.type_ == "Armor"
                || item.type_ == "Head"
        })
        .filter(|item| {
            // Filter out items that are not dropped by a Mammon.
            item.dropped_by.is_some()
                && item
                    .dropped_by
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|monster| monster.name == "Mammon" || monster.name == "Arisen Mammon")
        })
    {
        if item.materials.is_none()
            || !item
                .materials
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item.name == "Ortanite")
        {
            add_item_materials(guide, item, &[ortanite])?;
        }
    }

    Ok(())
}

fn autofix() -> Result<(), Error> {
    let _ = dotenv();
    let cookie = dotenv::var("ORNAGUIDE_COOKIE").unwrap();
    let mut cache = CachedGuide::from_directory(Path::new("./jsons/"))?;
    let guide = OrnaAdminGuide::new(&cookie)?;
    let raw_items = cache.fetch_items()?;

    fix_morri_item_materials(&guide, raw_items)?;
    fix_lyonesse_items(&guide, raw_items)?;
    fix_apollyon_items(&guide, raw_items)?;
    fix_mammon_items(&guide, raw_items)?;
    Ok(())
}

fn main() {
    match autofix() {
        Ok(_) => println!("OK"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
