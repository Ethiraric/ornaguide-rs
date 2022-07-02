use std::fmt::{Debug, Display};

use itertools::Itertools;
use ornaguide_rs::{
    codex::{CodexElement, CodexItem, ItemStatusEffects},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    items::admin::AdminItem,
};

use crate::{
    misc::{diff_sorted_slices, sanitize_guide_name},
    output::OrnaData,
};

/// List items that are on the guide and not the codex, or on the codex and not on the guide.
fn list_missing(data: &OrnaData) -> Result<(), Error> {
    let missing_on_guide = data
        .codex
        .items
        .items
        .iter()
        // Filter out developer items.
        .filter(|item| item.name != "Orna")
        // Filter out items we know nothing about.
        .filter(|item| {
            item.slug != "balins-left-b2db2fdb"
                && item.slug != "blinders"
                && item.slug != "naggeneens-song"
                && item.slug != "ravens-feathers"
                && item.slug != "soul-blade"
                && item.slug != "steadfast-charm"
                && item.slug != "super-exp-potion"
        })
        .filter(|item| data.guide.items.find_match_for_codex_item(item).is_err())
        .collect::<Vec<_>>();

    let not_on_codex = data
        .guide
        .items
        .items
        .iter()
        // Filter out the old Spellcaster's Ring.
        .filter(|item| item.name != "Mage's Ring")
        .filter(|item| data.codex.items.find_match_for_admin_item(item).is_err())
        .collect::<Vec<_>>();

    if !missing_on_guide.is_empty() {
        println!("Items missing on guide:");
        for item in missing_on_guide.iter() {
            println!(
                "\t- {:20} (https://playorna.com/codex/items/{})",
                item.name, item.slug
            );
        }
    }

    if !not_on_codex.is_empty() {
        println!("Items not on codex:");
        for item in not_on_codex.iter() {
            println!(
                "\t- {:20} (https://orna.guide/items?show={})",
                item.name, item.id
            );
        }
    }

    Ok(())
}

/// Return an iterator over the status afflictions a weapon with the given element may inflict.
fn get_iter_element_statuses(element: Option<&CodexElement>) -> std::vec::IntoIter<String> {
    match element {
        Some(CodexElement::Fire) => vec!["Burning".to_string()].into_iter(),
        Some(CodexElement::Water) => vec!["Frozen".to_string()].into_iter(),
        Some(CodexElement::Earthen) => vec!["Rot".to_string()].into_iter(),
        Some(CodexElement::Lightning) => vec!["Paralyzed".to_string()].into_iter(),
        Some(CodexElement::Holy) => vec!["Blind".to_string()].into_iter(),
        Some(CodexElement::Dark) => vec!["Asleep".to_string()].into_iter(),
        Some(CodexElement::Arcane) => vec![
            "Burning".to_string(),
            "Frozen".to_string(),
            "Rot".to_string(),
            "Paralyzed".to_string(),
        ]
        .into_iter(),
        Some(CodexElement::Dragon) => vec!["Blight".to_string()].into_iter(),
        _ => vec![].into_iter(),
    }
}

/// Compare a single stat and print an error message if they differ.
/// Return whether the stats matched.
fn check_stat<AS, CS, Fixer>(
    stat_name: &str,
    admin_item: &AdminItem,
    admin_stat: AS,
    codex_stat: CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Display,
    CS: Display,
    Fixer: FnOnce(&mut AdminItem, &CS),
{
    if admin_stat != codex_stat {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m codex= {:<20} guide= {:<20}",
            admin_item.name, stat_name, codex_stat, admin_stat
        );
        if fix {
            let mut item = guide.admin_retrieve_item_by_id(admin_item.id)?;
            fixer(&mut item, &codex_stat);
            guide.admin_save_item(item)?;
            guide.admin_retrieve_item_by_id(admin_item.id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare a single stat and prints an error message if they differ.
/// Requires `Debug` instead of `Display`.
/// Returns whether the stats matched.
fn check_stat_debug<AS, CS, Fixer>(
    stat_name: &str,
    admin_item: &AdminItem,
    admin_stat: &AS,
    codex_stat: &CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Debug,
    CS: Debug,
    Fixer: FnOnce(&mut AdminItem, &CS),
{
    if admin_stat != codex_stat {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:<80?}\nguide= {:?}",
            admin_item.name, stat_name, codex_stat, admin_stat
        );
        if fix {
            let mut item = guide.admin_retrieve_item_by_id(admin_item.id)?;
            fixer(&mut item, codex_stat);
            guide.admin_save_item(item)?;
            guide.admin_retrieve_item_by_id(admin_item.id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare the dropped_by list for an item.
/// The steps to accomplish that are longer than for other fields, so the piece of code is isolated
/// in its own function.
fn check_item_dropped_by(
    data: &OrnaData,
    fix: bool,
    guide: &OrnaAdminGuide,
    codex_item: &CodexItem,
    guide_item: &AdminItem,
) -> Result<(), Error> {
    // TODO*ethiraric, 10/06/2022): Refactor this mess.

    // List monster uris that drop the item.
    let guide_uris = data
        .guide
        .monsters
        .monsters
        .iter()
        .filter_map(|monster| {
            monster
                .drops
                .iter()
                .find(|id| **id == guide_item.id)
                .map(|_| monster)
        })
        // Filter out entries without a codex_uri.
        // This should remove Vulcan and The Fools entries.
        .filter(|monster| !monster.codex_uri.is_empty())
        // Map them to their codex_uris.
        .map(|monster| monster.codex_uri.clone())
        .sorted()
        .collect::<Vec<_>>();
    let codex_uris = codex_item
        .dropped_by
        .iter()
        .map(|drop_by| drop_by.uri.clone())
        .sorted()
        .collect::<Vec<_>>();
    let ok = check_stat_debug(
        "dropped_by",
        guide_item,
        &guide_uris,
        &codex_uris,
        false,
        |_, _| {},
        guide,
    )?;

    if ok {
        return Ok(());
    }

    // List the codex uris of monsters we should edit.
    let (addenda, to_remove) = diff_sorted_slices(&codex_uris, &guide_uris);
    if addenda.is_empty() && to_remove.is_empty() {
        return Ok(());
    }
    if !addenda.is_empty() {
        println!("\x1b[0;32mSuggest adding: {:?}\x1b[0m", addenda);
    }
    if !to_remove.is_empty() {
        println!("\x1b[0;31mSuggest removing: {:?}\x1b[0m", to_remove);
    }

    if fix && !addenda.is_empty() && !to_remove.is_empty() {
        // Edit all monsters we should add the drop to.
        for monster_id in addenda
            .iter()
            .filter_map(|uri| data.codex.find_generic_monster_from_uri(uri))
            .filter_map(|codex_monster| {
                data.guide
                    .find_match_for_codex_generic_monster(codex_monster)
                    .map_err(|err| println!("{}", err))
                    .ok()
                    .map(|monster| monster.id)
            })
        {
            let mut monster = guide.admin_retrieve_monster_by_id(monster_id)?;
            println!(
                "Adding drop {} (#{}) to monster {} (#{})",
                guide_item.name, guide_item.id, monster.name, monster.id
            );
            // Guard editing a monster we might already have added the item to, but not refreshed
            // the jsons yet.
            if monster.drops.contains(&guide_item.id) {
                continue;
            }
            monster.drops.push(guide_item.id);
            guide.admin_save_monster(monster)?;
            guide.admin_retrieve_monster_by_id(monster_id)?;
        }

        // Edit all monsters we should remove the drop from.
        for monster_id in to_remove
            .iter()
            .filter_map(|uri| data.codex.find_generic_monster_from_uri(uri))
            .filter_map(|codex_monster| {
                data.guide
                    .find_match_for_codex_generic_monster(codex_monster)
                    .map_err(|err| println!("{}", err))
                    .ok()
                    .map(|monster| monster.id)
            })
        {
            let mut monster = guide.admin_retrieve_monster_by_id(monster_id)?;
            println!(
                "Removing drop {} (#{}) from monster {} (#{})",
                guide_item.name, guide_item.id, monster.name, monster.id
            );
            if let Some(pos) = monster
                .drops
                .iter()
                .position(|item_id| *item_id == guide_item.id)
            {
                monster.drops.remove(pos);
                guide.admin_save_monster(monster)?;
                guide.admin_retrieve_monster_by_id(monster_id)?;
            }
        }
    }
    Ok(())
}

/// Check for mismatches in the stats.
fn check_stats(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let guide_weapon_id = data
        .guide
        .static_
        .item_types
        .iter()
        .find(|type_| type_.name == "Weapon")
        .unwrap()
        .id;
    for codex_item in data.codex.items.items.iter() {
        if let Ok(guide_item) = data.guide.items.find_match_for_codex_item(codex_item) {
            // Icon
            check_stat(
                "icon",
                guide_item,
                &guide_item.image_name,
                &codex_item.icon,
                fix,
                |item, icon| {
                    item.image_name = icon.to_string();
                },
                guide,
            )?;
            // Description
            check_stat(
                "description",
                guide_item,
                &guide_item.description,
                &codex_item.description,
                fix,
                |item, description| {
                    item.description = description.to_string();
                },
                guide,
            )?;
            // Attack
            check_stat(
                "attack",
                guide_item,
                guide_item.attack,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.attack)
                    .unwrap_or(0),
                fix,
                |item, attack| {
                    item.attack = *attack;
                },
                guide,
            )?;
            // Magic
            check_stat(
                "magic",
                guide_item,
                guide_item.magic,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.magic)
                    .unwrap_or(0),
                fix,
                |item, magic| {
                    item.magic = *magic;
                },
                guide,
            )?;
            // HP
            check_stat(
                "hp",
                guide_item,
                guide_item.hp,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.hp)
                    .unwrap_or(0),
                fix,
                |item, hp| {
                    item.hp = *hp;
                },
                guide,
            )?;
            // Mana
            check_stat(
                "mana",
                guide_item,
                guide_item.mana,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.mana)
                    .unwrap_or(0),
                fix,
                |item, mana| {
                    item.mana = *mana;
                },
                guide,
            )?;
            // Defense
            check_stat(
                "defense",
                guide_item,
                guide_item.defense,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.defense)
                    .unwrap_or(0),
                fix,
                |item, defense| {
                    item.defense = *defense;
                },
                guide,
            )?;
            // Resistance
            check_stat(
                "resistance",
                guide_item,
                guide_item.resistance,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.resistance)
                    .unwrap_or(0),
                fix,
                |item, resistance| {
                    item.resistance = *resistance;
                },
                guide,
            )?;
            // Ward
            check_stat(
                "ward",
                guide_item,
                guide_item.ward,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.ward)
                    .unwrap_or(0),
                fix,
                |item, ward| {
                    item.ward = *ward;
                },
                guide,
            )?;
            // Dexterity
            check_stat(
                "dexterity",
                guide_item,
                guide_item.dexterity,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.dexterity)
                    .unwrap_or(0),
                fix,
                |item, dexterity| {
                    item.dexterity = *dexterity;
                },
                guide,
            )?;
            // Crit
            check_stat(
                "crit",
                guide_item,
                guide_item.crit,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.crit)
                    .unwrap_or(0),
                fix,
                |item, crit| {
                    item.crit = *crit;
                },
                guide,
            )?;
            // Foresight
            check_stat(
                "foresight",
                guide_item,
                guide_item.foresight,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.foresight)
                    .unwrap_or(0),
                fix,
                |item, foresight| {
                    item.foresight = *foresight;
                },
                guide,
            )?;
            // Adorn slots
            check_stat(
                "adorn slots",
                guide_item,
                guide_item.base_adornment_slots,
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.adornment_slots)
                    .unwrap_or(0),
                fix,
                |item, slots| {
                    item.base_adornment_slots = *slots;
                    item.has_slots = *slots != 0;
                },
                guide,
            )?;
            // Element
            check_stat(
                "element",
                guide_item,
                guide_item
                    .element
                    .map(|element_id| {
                        data.guide
                            .static_
                            .elements
                            .iter()
                            .find(|element| element.id == element_id)
                            .unwrap()
                            .name
                            .clone()
                    })
                    .unwrap_or_default(),
                codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.element.as_ref())
                    .map(|element| element.to_string())
                    .unwrap_or_default(),
                fix,
                |item, element| {
                    if element.is_empty() {
                        item.element = None;
                    } else {
                        let guide_element = data
                            .guide
                            .static_
                            .elements
                            .iter()
                            .find(|el| el.name == *element);
                        if let Some(guide_element) = guide_element {
                            item.element = Some(guide_element.id);
                        } else {
                            println!("Failed to find element {} on guide", element);
                        }
                    }
                },
                guide,
            )?;
            // Ability
            check_stat(
                "ability",
                guide_item,
                guide_item
                    .ability
                    .and_then(|ability_id| {
                        data.guide
                            .skills
                            .skills
                            .iter()
                            .find(|skill| skill.id == ability_id)
                    })
                    .map(|skill| sanitize_guide_name(&skill.name))
                    .unwrap_or_default(),
                codex_item
                    .ability
                    .as_ref()
                    .map(|ability| ability.name.clone())
                    .unwrap_or_default(),
                fix,
                |item, ability_name| {
                    if ability_name.is_empty() {
                        item.ability = None;
                    } else {
                        let guide_ability = data
                            .guide
                            .skills
                            .skills
                            .iter()
                            .find(|skill| skill.name == *ability_name);
                        if let Some(guide_ability) = guide_ability {
                            item.ability = Some(guide_ability.id);
                        } else {
                            println!("Failed to find ability {} on guide", ability_name);
                        }
                    }
                },
                guide,
            )?;
            // Causes
            check_stat_debug(
                "causes",
                guide_item,
                &guide_item
                    .causes
                    .iter()
                    .map(|cause_id| {
                        data.guide
                            .static_
                            .status_effects
                            .iter()
                            .find(|status| status.id == *cause_id)
                            .unwrap_or_else(|| panic!("Failed to find guide cause {}", cause_id))
                            .name
                            .clone()
                    })
                    .sorted()
                    .collect::<Vec<_>>(),
                &codex_item
                    .causes
                    .to_guide_names()
                    .into_iter()
                    // TODO(ethiraric, 04/06/2022): Remove this chain and the dedup call below once
                    // the codex fixes elemental statuses for weapons.
                    .chain(if guide_item.type_ == guide_weapon_id {
                        get_iter_element_statuses(
                            codex_item
                                .stats
                                .as_ref()
                                .and_then(|stats| stats.element.as_ref()),
                        )
                    } else {
                        Vec::<String>::new().into_iter()
                    })
                    .sorted()
                    .dedup()
                    .collect::<Vec<_>>(),
                fix,
                |item, codex_causes| {
                    item.causes = codex_causes
                        .iter()
                        .filter_map(|cause_name| {
                            data.guide
                                .static_
                                .status_effects
                                .iter()
                                .find(|cause| cause.name == *cause_name)
                                .map(|cause| cause.id)
                        })
                        .collect();
                },
                guide,
            )?;
            // Cures
            check_stat_debug(
                "cures",
                guide_item,
                &guide_item
                    .cures
                    .iter()
                    .map(|cure_id| {
                        data.guide
                            .static_
                            .status_effects
                            .iter()
                            .find(|status| status.id == *cure_id)
                            .unwrap_or_else(|| panic!("Failed to find guide cures {}", cure_id))
                            .name
                            .clone()
                    })
                    .sorted()
                    .collect::<Vec<_>>(),
                &codex_item.cures.to_guide_names(),
                fix,
                |item, codex_cures| {
                    item.cures = codex_cures
                        .iter()
                        .filter_map(|cure_name| {
                            data.guide
                                .static_
                                .status_effects
                                .iter()
                                .find(|cure| cure.name == *cure_name)
                                .map(|cure| cure.id)
                        })
                        .collect();
                },
                guide,
            )?;
            // Gives
            check_stat_debug(
                "gives",
                guide_item,
                &guide_item
                    .gives
                    .iter()
                    .map(|give_id| {
                        data.guide
                            .static_
                            .status_effects
                            .iter()
                            .find(|status| status.id == *give_id)
                            .unwrap_or_else(|| panic!("Failed to find guide give {}", give_id))
                            .name
                            .clone()
                    })
                    .sorted()
                    .collect::<Vec<_>>(),
                &codex_item.gives.to_guide_names(),
                fix,
                |item, codex_give| {
                    item.gives = codex_give
                        .iter()
                        .filter_map(|give_name| {
                            data.guide
                                .static_
                                .status_effects
                                .iter()
                                .find(|give| give.name == *give_name)
                                .map(|give| give.id)
                        })
                        .collect();
                },
                guide,
            )?;
            // Immunities
            check_stat_debug(
                "immunities",
                guide_item,
                &guide_item
                    .prevents
                    .iter()
                    .map(|immunity_id| {
                        data.guide
                            .static_
                            .status_effects
                            .iter()
                            .find(|status| status.id == *immunity_id)
                            .unwrap_or_else(|| {
                                panic!("Failed to find guide immunity {}", immunity_id)
                            })
                            .name
                            .clone()
                    })
                    .sorted()
                    .collect::<Vec<_>>(),
                &codex_item.immunities.to_guide_names(),
                fix,
                |item, codex_immunity| {
                    item.prevents = codex_immunity
                        .iter()
                        .filter_map(|immunity_name| {
                            data.guide
                                .static_
                                .status_effects
                                .iter()
                                .find(|immunity| immunity.name == *immunity_name)
                                .map(|immunity| immunity.id)
                        })
                        .collect();
                },
                guide,
            )?;
            // Dropped by
            check_item_dropped_by(data, fix, guide, codex_item, guide_item)?;
            // Upgrade Materials
            check_stat_debug(
                "upgrade materials",
                guide_item,
                &guide_item
                    .materials
                    .iter()
                    .map(|material_id| {
                        data.guide
                            .items
                            .items
                            .iter()
                            .find(|item| item.id == *material_id)
                            .unwrap_or_else(|| {
                                panic!("Failed to find guide material {}", material_id)
                            })
                            .codex_uri
                            .clone()
                    })
                    .sorted()
                    .collect::<Vec<_>>(),
                &codex_item
                    .upgrade_materials
                    .iter()
                    .map(|material| material.uri.clone())
                    .sorted()
                    .collect::<Vec<_>>(),
                fix,
                |item, materials| {
                    item.materials = materials
                        .iter()
                        .map(|material_uri| {
                            data.guide
                                .items
                                .items
                                .iter()
                                .find(|item| item.codex_uri == *material_uri)
                                .unwrap_or_else(|| {
                                    panic!("Failed to find material with uri {}", material_uri)
                                })
                        })
                        .map(|item| item.id)
                        .collect();
                },
                guide,
            )?;
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide items and the codex items.
pub fn perform(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    check_stats(data, fix, guide)?;
    Ok(())
}
