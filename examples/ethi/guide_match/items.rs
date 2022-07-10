use itertools::Itertools;
use ornaguide_rs::{
    codex::{CodexElement, ItemStatusEffects},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide, VecElements},
};

use crate::{
    guide_match::misc::{
        fix_option_field, fix_staus_effects_field, fix_vec_field, fix_vec_id_field, Checker,
    },
    misc::{sanitize_guide_name, VecStatusEffectIds},
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
fn get_iter_element_statuses(element: Option<&CodexElement>) -> std::vec::IntoIter<&str> {
    match element {
        Some(CodexElement::Fire) => vec!["Burning"].into_iter(),
        Some(CodexElement::Water) => vec!["Frozen"].into_iter(),
        Some(CodexElement::Earthen) => vec!["Rot"].into_iter(),
        Some(CodexElement::Lightning) => vec!["Paralyzed"].into_iter(),
        Some(CodexElement::Holy) => vec!["Blind"].into_iter(),
        Some(CodexElement::Dark) => vec!["Asleep"].into_iter(),
        Some(CodexElement::Arcane) => vec!["Burning", "Frozen", "Rot", "Paralyzed"].into_iter(),
        Some(CodexElement::Dragon) => vec!["Blight"].into_iter(),
        _ => vec![].into_iter(),
    }
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
            let check = Checker {
                entity_name: &guide_item.name,
                entity_id: guide_item.id,
                fix,
                golden: |id| guide.admin_retrieve_item_by_id(id),
                saver: |item| guide.admin_save_item(item),
            };

            // Icon
            check.display(
                "icon",
                &guide_item.image_name,
                &codex_item.icon,
                |item, icon| {
                    item.image_name = icon.to_string();
                    Ok(())
                },
            )?;

            // Description
            check.display(
                "description",
                &guide_item.description,
                &codex_item.description,
                |item, description| {
                    item.description = description.to_string();
                    Ok(())
                },
            )?;

            // Attack
            check.display(
                "attack",
                &guide_item.attack,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.attack)
                    .unwrap_or(0),
                |item, attack| {
                    item.attack = *attack;
                    Ok(())
                },
            )?;

            // Magic
            check.display(
                "magic",
                &guide_item.magic,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.magic)
                    .unwrap_or(0),
                |item, magic| {
                    item.magic = *magic;
                    Ok(())
                },
            )?;

            // HP
            check.display(
                "hp",
                &guide_item.hp,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.hp)
                    .unwrap_or(0),
                |item, hp| {
                    item.hp = *hp;
                    Ok(())
                },
            )?;

            // Mana
            check.display(
                "mana",
                &guide_item.mana,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.mana)
                    .unwrap_or(0),
                |item, mana| {
                    item.mana = *mana;
                    Ok(())
                },
            )?;

            // Defense
            check.display(
                "defense",
                &guide_item.defense,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.defense)
                    .unwrap_or(0),
                |item, defense| {
                    item.defense = *defense;
                    Ok(())
                },
            )?;

            // Resistance
            check.display(
                "resistance",
                &guide_item.resistance,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.resistance)
                    .unwrap_or(0),
                |item, resistance| {
                    item.resistance = *resistance;
                    Ok(())
                },
            )?;

            // Ward
            check.display(
                "ward",
                &guide_item.ward,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.ward)
                    .unwrap_or(0),
                |item, ward| {
                    item.ward = *ward;
                    Ok(())
                },
            )?;

            // Dexterity
            check.display(
                "dexterity",
                &guide_item.dexterity,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.dexterity)
                    .unwrap_or(0),
                |item, dexterity| {
                    item.dexterity = *dexterity;
                    Ok(())
                },
            )?;

            // Crit
            check.display(
                "crit",
                &guide_item.crit,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.crit)
                    .unwrap_or(0),
                |item, crit| {
                    item.crit = *crit;
                    Ok(())
                },
            )?;

            // Foresight
            check.display(
                "foresight",
                &guide_item.foresight,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.foresight)
                    .unwrap_or(0),
                |item, foresight| {
                    item.foresight = *foresight;
                    Ok(())
                },
            )?;

            // Adorn slots
            check.display(
                "adorn slots",
                &guide_item.base_adornment_slots,
                &codex_item
                    .stats
                    .as_ref()
                    .and_then(|stats| stats.adornment_slots)
                    .unwrap_or(0),
                |item, slots| {
                    item.base_adornment_slots = *slots;
                    item.has_slots = *slots != 0;
                    Ok(())
                },
            )?;

            // Element
            let guide_element = &guide_item.element.map(|element_id| {
                data.guide
                    .static_
                    .elements
                    .find_element_by_id(element_id)
                    .unwrap()
                    .name
                    .as_str()
            });
            let codex_element = &codex_item
                .stats
                .as_ref()
                .and_then(|stats| stats.element.as_ref())
                .map(|element| element.to_string());
            check.debug(
                "element",
                guide_element,
                &codex_element.as_deref(),
                |item, element| {
                    fix_option_field(
                        item,
                        |item| Ok(&mut item.element),
                        element,
                        |element| Ok(data.guide.static_.elements.get_element_by_name(element)?.id),
                    )
                },
            )?;

            // Ability
            let guide_ability = guide_item
                .ability
                .and_then(|ability_id| {
                    data.guide
                        .skills
                        .skills
                        .iter()
                        .find(|skill| skill.id == ability_id)
                })
                .map(|skill| sanitize_guide_name(&skill.name));
            let codex_ability = codex_item
                .ability
                .as_ref()
                .map(|ability| ability.name.as_str());
            check.debug(
                "ability",
                &guide_ability,
                &codex_ability,
                |item, ability_name| {
                    fix_option_field(
                        item,
                        |item| Ok(&mut item.ability),
                        ability_name,
                        |ability_name| {
                            data.guide
                                .skills
                                .get_offhand_skill_from_name(ability_name)
                                .map(|skill| skill.id)
                        },
                    )
                },
            )?;

            // Causes
            let guide_causes = guide_item
                .causes
                .guide_status_effect_ids_to_guide_names(data);
            let codex_causes = codex_item
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
                    Vec::<&str>::new().into_iter()
                })
                .sorted()
                .dedup()
                .collect::<Vec<_>>();
            check.debug(
                "causes",
                &guide_causes,
                &codex_causes,
                |item, codex_causes| {
                    fix_staus_effects_field(
                        "causes",
                        item,
                        &guide_causes,
                        data,
                        codex_causes,
                        |item| &mut item.causes,
                    )
                },
            )?;

            // Cures
            let guide_cures = guide_item
                .cures
                .guide_status_effect_ids_to_guide_names(data);
            check.debug(
                "cures",
                &guide_cures,
                &codex_item.cures.to_guide_names(),
                |item, codex_cures| {
                    fix_staus_effects_field(
                        "cures",
                        item,
                        &guide_cures,
                        data,
                        codex_cures,
                        |item| &mut item.cures,
                    )
                },
            )?;

            // Gives
            let guide_gives = guide_item
                .gives
                .guide_status_effect_ids_to_guide_names(data);
            check.debug(
                "gives",
                &guide_gives,
                &codex_item.gives.to_guide_names(),
                |item, codex_gives| {
                    fix_staus_effects_field(
                        "gives",
                        item,
                        &guide_gives,
                        data,
                        codex_gives,
                        |item| &mut item.gives,
                    )
                },
            )?;

            // Immunities
            let guide_immunities = guide_item
                .prevents
                .guide_status_effect_ids_to_guide_names(data);
            check.debug(
                "immunities",
                &guide_immunities,
                &codex_item.immunities.to_guide_names(),
                |item, codex_immunities| {
                    fix_staus_effects_field(
                        "immunities",
                        item,
                        &guide_immunities,
                        data,
                        codex_immunities,
                        |item| &mut item.prevents,
                    )
                },
            )?;

            // Dropped by
            let guide_dropped_by_uris = data
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
                .map(|monster| monster.codex_uri.as_str())
                .sorted()
                .collect::<Vec<_>>();
            let codex_dropped_by_uris = codex_item
                .dropped_by
                .iter()
                .map(|dropped_by| dropped_by.uri.as_str())
                .sorted()
                .collect::<Vec<_>>();
            check.debug(
                "dropped_by",
                &guide_dropped_by_uris,
                &codex_dropped_by_uris,
                |item, dropped_by| {
                    fix_vec_field(
                        item,
                        |_| Ok(&guide_dropped_by_uris),
                        dropped_by,
                        |_, uris| {
                            // For each monster that is missing a drop.
                            for uri in uris.iter() {
                                // Fetch the monster.
                                let id = data.guide.monsters.get_match_for_codex_uri(uri)?.id;
                                let mut monster = guide.admin_retrieve_monster_by_id(id)?;
                                // Check whether the drop was not just missing from the cache.
                                if !monster.drops.contains(&guide_item.id) {
                                    // Add the drop to the monster and save it.
                                    monster.drops.push(guide_item.id);
                                    guide.admin_save_monster(monster)?;
                                    guide.admin_retrieve_monster_by_id(id)?;
                                }
                            }
                            Ok(())
                        },
                        |_, uris| {
                            // For each monster thet has one too much a drop.
                            for uri in uris.iter() {
                                // Fetch the monster.
                                let id = data.guide.monsters.get_match_for_codex_uri(uri)?.id;
                                let mut monster = guide.admin_retrieve_monster_by_id(id)?;
                                // Check whether the drop was not just present in the cache.
                                if monster.drops.contains(&guide_item.id) {
                                    // Remove the drop from the monster and save it.
                                    monster.drops.retain(|id| *id != guide_item.id);
                                    guide.admin_save_monster(monster)?;
                                    guide.admin_retrieve_monster_by_id(id)?;
                                }
                            }
                            Ok(())
                        },
                    )
                },
            )?;

            // Upgrade Materials
            let guide_upgrade_materials = guide_item
                .materials
                .iter()
                .map(|material_id| {
                    data.guide
                        .items
                        .items
                        .iter()
                        .find(|item| item.id == *material_id)
                        .unwrap_or_else(|| panic!("Failed to find guide material {}", material_id))
                        .codex_uri
                        .as_str()
                })
                .sorted()
                .collect::<Vec<_>>();
            let codex_upgrade_materials = codex_item
                .upgrade_materials
                .iter()
                .map(|material| material.uri.as_str())
                .sorted()
                .collect::<Vec<_>>();
            check.debug(
                "upgrade materials",
                &guide_upgrade_materials,
                &codex_upgrade_materials,
                |item, materials| {
                    fix_vec_id_field(
                        "upgrade_materials",
                        item,
                        &guide_upgrade_materials,
                        materials,
                        |item| &mut item.materials,
                        |id| Ok(&data.guide.items.get_item_by_id(id)?.codex_uri),
                        |uri| Ok(data.guide.items.get_item_by_uri(uri)?.id),
                    )
                },
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
