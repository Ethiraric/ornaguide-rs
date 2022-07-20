mod fixes;

use std::path::Path;

use dotenv::dotenv;

use fixes::Fixes;
use ornaguide_rs::{
    error::Error,
    guide::{CachedGuide, Guide, OrnaAdminGuide},
    items::RawItem,
    skills::RawSkill,
};

/// Whether we do a dry run of the fixes, i.e. not committing the changes to the guide and only
/// outputting what would change.
const DRY_RUN: bool = false;

/// Returns true if the item is an equippable that can be upgraded and, as such, as materials
/// associated to it.
fn item_is_equippable(item: &RawItem) -> bool {
    item.type_ == "Legs"
        || item.type_ == "Off-hand"
        || item.type_ == "Weapon"
        || item.type_ == "Armor"
        || item.type_ == "Head"
}

/// Returns true if the item is dropped by the given monster.
fn is_item_dropped_by(item: &RawItem, whom: &str) -> bool {
    item.dropped_by.is_some()
        && item
            .dropped_by
            .as_ref()
            .unwrap()
            .iter()
            .any(|monster| monster.name == whom)
}

/// Returns true if the item needs the given material to be upgraded.
fn item_has_material(item: &RawItem, mat_name: &str) -> bool {
    item.materials.is_some()
        && item
            .materials
            .as_ref()
            .unwrap()
            .iter()
            .any(|mat| mat.name == mat_name)
}

/// Walk through most The Morrigan and Arisen Morrigan drops and check that they have the correct
/// materials. For those we check, it should be Cursed Ortanite for The Morrigan drops, or Cursed
/// Ortanite + Greater Souls for Arisen Morrigan drops.
#[allow(dead_code)]
fn fix_morri_item_materials(fixes: &Fixes, items: &[RawItem]) -> Result<(), Error> {
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
        .filter(|item| item_is_equippable(item))
        .filter(|item| {
            // Those items are annoying. Realm Katar requires Realm Ore and Cursed Ortanite (Arisen
            // Realm Katar is still Cursed Ortanite and Greater Soul). Morrigan's Scroll requires
            // Nightstone and Cursed Ortanite (though again, the Arisen version is still Cursed
            // Ortanite and Greater Soul).
            item.name != "Realm Katar" && item.name != "Morrigan's Scroll"
        })
        .filter(|item| {
            // Filter out items that are not dropped by The Morrigan or Arisen Morrigan.
            is_item_dropped_by(item, "Arisen Morrigan") || is_item_dropped_by(item, "The Morrigan")
        })
    {
        if is_item_dropped_by(item, "Arisen Morrigan") {
            if item.materials.is_some() && item.materials.as_ref().unwrap().len() == 2 {
                if !(item_has_material(item, "Cursed Ortanite")
                    && item_has_material(item, "Greater Soul"))
                {
                    fixes.set_item_materials_to(item, &[co, gs])?;
                }
            } else {
                fixes.set_item_materials_to(item, &[co, gs])?;
            }
        } else if is_item_dropped_by(item, "The Morrigan") {
            if !item_has_material(item, "Cursed Ortanite") {
                fixes.set_item_materials_to(item, &[co])?
            }
        } else {
            panic!("Items here should be either a drop from The Morrigan or Arisen Morrigan")
        }
    }

    Ok(())
}

/// Walk through Lyonesse drops and check that they have Lyonite.
fn fix_lyonesse_items(fixes: &Fixes, items: &[RawItem]) -> Result<(), Error> {
    let lyonite = items.iter().find(|item| item.name == "Lyonite").unwrap().id;
    for item in items
        .iter()
        .filter(|item| item_is_equippable(item))
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
        if !item_has_material(item, "Lyonite") {
            fixes.add_item_materials(item, &[lyonite])?;
        }
    }

    Ok(())
}

/// Walk through Apollyon drops and check that they have Realm Ore.
fn fix_apollyon_items(fixes: &Fixes, items: &[RawItem]) -> Result<(), Error> {
    let ore = items
        .iter()
        .find(|item| item.name == "Realm Ore")
        .unwrap()
        .id;
    for item in items
        .iter()
        .filter(|item| item_is_equippable(item))
        .filter(|item| is_item_dropped_by(item, "Apollyon"))
    {
        if !item_has_material(item, "Realm Ore") {
            fixes.add_item_materials(item, &[ore])?;
        }
    }

    Ok(())
}

/// Walk through Mammon and Arisen Mammon drops and check that they have Ortanite.
fn fix_mammon_items(fixes: &Fixes, items: &[RawItem]) -> Result<(), Error> {
    let ortanite = items
        .iter()
        .find(|item| item.name == "Ortanite")
        .unwrap()
        .id;
    for item in items
        .iter()
        .filter(|item| item_is_equippable(item))
        .filter(|item| {
            // Filter out items that are not dropped by a Mammon.
            is_item_dropped_by(item, "Mammon") || is_item_dropped_by(item, "Arisen Mammon")
        })
    {
        if !item_has_material(item, "Ortanite") {
            fixes.add_item_materials(item, &[ortanite])?;
        }
    }

    Ok(())
}

/// Set all `is_magic` for buff spells to true.
fn fix_buffs_is_magic(fixes: &Fixes, skills: &[RawSkill]) -> Result<(), Error> {
    for skill in skills
        .iter()
        .filter(|skill| skill.type_ == "Buff")
        .filter(|skill| !skill.is_magic)
    {
        fixes.set_skill_is_magic(skill, true)?;
    }
    Ok(())
}

/// Set all `is_magic` for passive spells to true.
fn fix_passives_is_magic(fixes: &Fixes, skills: &[RawSkill]) -> Result<(), Error> {
    for skill in skills
        .iter()
        .filter(|skill| skill.type_ == "Passive")
        .filter(|skill| !skill.is_magic)
    {
        fixes.set_skill_is_magic(skill, true)?;
    }
    Ok(())
}

fn autofix() -> Result<(), Error> {
    let _ = dotenv();
    let cookie = dotenv::var("ORNAGUIDE_COOKIE").unwrap();
    let mut cache = CachedGuide::from_directory(Path::new("./jsons/"))?;
    let guide = OrnaAdminGuide::new(&cookie)?;
    cache.fetch_items()?;
    cache.fetch_skills()?;
    let raw_items = cache.get_items().unwrap();
    let raw_skills = cache.get_skills().unwrap();
    let fixes = Fixes::new(DRY_RUN, guide);

    fix_morri_item_materials(&fixes, raw_items)?;
    fix_lyonesse_items(&fixes, raw_items)?;
    fix_apollyon_items(&fixes, raw_items)?;
    fix_mammon_items(&fixes, raw_items)?;

    fix_buffs_is_magic(&fixes, raw_skills)?;
    fix_passives_is_magic(&fixes, raw_skills)?;

    Ok(())
}

fn main() {
    match autofix() {
        Ok(_) => println!("OK"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
