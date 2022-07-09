use itertools::Itertools;
use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide, Static},
    monsters::admin::AdminMonster,
};

use crate::{
    guide_match::misc::{fix_abilities_field, Checker},
    misc::{diff_sorted_slices, VecSkillIds},
    output::{CodexGenericMonster, OrnaData},
};

/// List monsters that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen. We can query the codex for monsters outside of their event.
fn list_missing(data: &OrnaData) -> Result<(), Error> {
    let missing_on_guide = data
        .codex
        .iter_all_monsters()
        .filter(|monster| {
            data.guide
                .find_match_for_codex_generic_monster(*monster)
                .is_err()
        })
        .collect::<Vec<_>>();

    let not_on_codex = data
        .guide
        .monsters
        .monsters
        .iter()
        .filter(|monster| {
            data.find_generic_codex_monster_from_admin_monster(monster)
                .is_err()
        })
        .collect::<Vec<_>>();

    if !missing_on_guide.is_empty() {
        println!("Monsters missing on guide:");
        for monster in missing_on_guide.iter() {
            match monster {
                CodexGenericMonster::Monster(monster) => {
                    println!(
                        "\t- [Monster] {:20} (https://playorna.com/codex/monsters/{})",
                        monster.name, monster.slug
                    )
                }
                CodexGenericMonster::Boss(boss) => {
                    println!(
                        "\t- [ Boss  ] {:20} (https://playorna.com/codex/bosses/{})",
                        boss.name, boss.slug
                    )
                }
                CodexGenericMonster::Raid(raid) => {
                    println!(
                        "\t- [ Raid  ] {:20} (https://playorna.com/codex/raids/{})",
                        raid.name, raid.slug
                    )
                }
            }
        }
    }

    if !not_on_codex.is_empty() {
        println!("Monsters not on codex:");
        for monster in not_on_codex.iter() {
            let kind = if monster.is_regular_monster() {
                "Monster"
            } else if monster.is_boss(&data.guide.static_.spawns) {
                "Boss"
            } else {
                "Raid"
            };
            println!(
                "\t-[{:^7}] {:20} (https://orna.guide/monsters?show={})",
                kind, monster.name, monster.id
            );
        }
    }

    Ok(())
}

fn fix_monster_event_spawns(
    monster: &mut AdminMonster,
    static_: &mut Static,
    expected_events: &[String],
    guide: &OrnaAdminGuide,
) -> Result<(), Error> {
    // Start by listing which events should be added and which removed.
    let mut admin_events = monster.get_events(&static_.spawns);
    admin_events.sort_by_cached_key(|event_name| {
        // Either `Event:` or `Past Event:`.
        if event_name.starts_with("Event:") {
            event_name[7..].to_string()
        } else {
            event_name[12..].to_string()
        }
    });
    let (to_add, to_remove) = diff_sorted_slices(expected_events, &admin_events);
    if !to_add.is_empty() {
        println!("\x1B[0;32mSuggest adding: {:?}\x1B[0m", to_add);
    }
    if !to_remove.is_empty() {
        println!("\x1B[0;31mSuggest removing: {:?}\x1B[0m", to_remove);
    }

    // Remove unneeded events by filtering the `Vec`.
    if !to_remove.is_empty() {
        monster.spawns.retain(|spawn_id| {
            if let Some(spawn) = static_.spawns.iter().find(|spawn| spawn.id == *spawn_id) {
                !to_remove.iter().any(|name| **name == spawn.event_name())
            } else {
                false
            }
        });
    }

    // Add the new events.
    if !to_add.is_empty() {
        // Split into 2 `Vec`s: Those that are already on the guide, and brand new events.
        let (ids_to_add, unknown_events): (Vec<_>, Vec<_>) = to_add
            .iter()
            .map(|event_name| {
                if let Some(spawn) = static_
                    .iter_events()
                    .find(|spawn| spawn.event_name() == **event_name)
                {
                    (Some(spawn.id), None)
                } else {
                    (None, Some(*event_name))
                }
            })
            .unzip();

        // For events we already know, push the ids in the monster.
        for spawn_id in ids_to_add.into_iter().flatten() {
            monster.spawns.push(spawn_id);
        }

        // For the others, create the events on the guide.
        if !unknown_events.is_empty() {
            for event_name in unknown_events.iter().flatten() {
                guide.admin_add_spawn(&format!("Past Event: {}", event_name))?;
            }
            static_.spawns = guide.admin_retrieve_spawns_list()?;
        }
        // Then add them to the monster.
        for spawn in unknown_events.iter().flatten().filter_map(|name| {
            static_
                .spawns
                .iter()
                .find(|spawn| spawn.event_name() == **name)
        }) {
            monster.spawns.push(spawn.id);
        }
    }
    Ok(())
}

fn check_fields(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for codex_monster in data.codex.iter_all_monsters() {
        if let Ok(admin_monster) = data
            .guide
            .find_match_for_codex_generic_monster(codex_monster)
            .cloned()
        {
            let check = Checker {
                entity_name: &admin_monster.name,
                entity_id: admin_monster.id,
                fix,
                golden: |id| guide.admin_retrieve_monster_by_id(id),
                saver: |monster| guide.admin_save_monster(monster),
            };

            // Image name
            check.display(
                "image_name",
                &admin_monster.image_name,
                codex_monster.icon(),
                |monster, image_name| {
                    monster.image_name = image_name.clone();
                    Ok(())
                },
            )?;
            // Event
            check.debug(
                "events",
                &admin_monster.get_events(&data.guide.static_.spawns),
                codex_monster.events(),
                |monster, events| {
                    fix_monster_event_spawns(monster, &mut data.guide.static_, events, guide)
                },
            )?;
            // Family
            let admin_family = admin_monster.family.as_ref().and_then(|id| {
                data.guide
                    .static_
                    .monster_families
                    .iter()
                    .find(|family| family.id == *id)
                    .map(|family| &family.name)
            });
            check.debug(
                "family",
                &admin_family,
                &codex_monster.family(),
                |monster: &mut AdminMonster, name| {
                    if name.is_none() {
                        monster.family = None;
                        Ok(())
                    } else if let Some(family) = data
                        .guide
                        .static_
                        .monster_families
                        .iter()
                        .find(|family| family.name == **name.as_ref().unwrap())
                    {
                        monster.family = Some(family.id);
                        Ok(())
                    } else {
                        Err(Error::Misc(format!(
                            "Failed to find family {} for monster {} (#{})",
                            name.as_ref().unwrap(),
                            admin_monster.name,
                            admin_monster.id
                        )))
                    }
                },
            )?;
            // Tags
            let admin_tags = admin_monster.get_raid_spawns(&data.guide.static_.spawns);
            static WRB_STR: &str = "World Raid";
            let codex_tags = codex_monster
                .tags_as_guide_spawns()
                .into_iter()
                .chain({
                    // TODO(ethiraric, 15/06/2022): Remove this once codex is updated.
                    if admin_monster.name == "Arisen Yggdrasil" || admin_monster.name == "Yggdrasil"
                    {
                        vec![WRB_STR].into_iter()
                    } else {
                        vec![].into_iter()
                    }
                })
                .sorted()
                .dedup()
                .collect::<Vec<_>>();
            check.debug(
                "tags",
                &admin_tags,
                &codex_tags,
                |monster: &mut AdminMonster, tags_strs| {
                    monster.spawns.retain(|spawn_id| {
                        data.guide
                            .static_
                            .spawns
                            .iter()
                            .find(|spawn| spawn.id == *spawn_id)
                            .map(|spawn| spawn.name != "Kingdom Raid" && spawn.name != "World Raid")
                            .unwrap_or(false)
                    });
                    for tag in tags_strs.iter() {
                        if let Some(spawn) = data
                            .guide
                            .static_
                            .spawns
                            .iter()
                            .find(|spawn| spawn.name == *tag)
                        {
                            if !monster.spawns.contains(&spawn.id) {
                                monster.spawns.push(spawn.id)
                            }
                        }
                    }
                    Ok(())
                },
            )?;
            // Abilities
            let admin_ability_uris = admin_monster.skills.guide_skill_ids_to_codex_uri(data);
            let expected_uris = codex_monster
                .abilities()
                .iter()
                .map(|ability| ability.uri.as_str())
                .sorted()
                .collect::<Vec<_>>();
            check.debug(
                "abilities",
                &admin_ability_uris,
                &expected_uris,
                |monster: &mut AdminMonster, _| {
                    fix_abilities_field(
                        monster,
                        &admin_ability_uris,
                        data,
                        &expected_uris,
                        |monster| &mut monster.skills,
                    )
                },
            )?;
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide monsters and the codex monsters.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
