use itertools::Itertools;
use ornaguide_rs::{
    data::{CodexGenericMonster, OrnaData},
    error::{Error, Kind},
    guide::{AdminGuide, OrnaAdminGuide},
    monsters::admin::AdminMonster,
};

use crate::{
    guide_match::{
        checker::{fix_abilities_field, fix_option_field, fix_spawn_field, Checker},
        misc::{CodexAbilities, EventsNames},
    },
    retry_once,
};

/// List monsters that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen. We can query the codex for monsters outside of their event.
fn list_missing(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let missing_on_guide = data
        .codex
        .iter_all_monsters()
        .filter(|monster| {
            data.guide
                .find_match_for_codex_generic_monster(*monster)
                .is_err()
        })
        .sorted_by_key(ornaguide_rs::data::CodexGenericMonster::name)
        .collect_vec();

    let not_on_codex = data
        .guide
        .monsters
        .monsters
        .iter()
        .filter(|monster| {
            data.find_generic_codex_monster_from_admin_monster(monster)
                .is_err()
        })
        .sorted_by_key(|monster| &monster.name)
        .collect_vec();

    if !missing_on_guide.is_empty() {
        println!("{} monsters missing on guide:", missing_on_guide.len());
        for monster in &missing_on_guide {
            match monster {
                CodexGenericMonster::Monster(monster) => {
                    println!(
                        "\t- [Monster] {:20} (https://playorna.com/codex/monsters/{})",
                        monster.name, monster.slug
                    );
                }
                CodexGenericMonster::Boss(boss) => {
                    println!(
                        "\t- [ Boss  ] {:20} (https://playorna.com/codex/bosses/{})",
                        boss.name, boss.slug
                    );
                }
                CodexGenericMonster::Raid(raid) => {
                    println!(
                        "\t- [ Raid  ] {:20} (https://playorna.com/codex/raids/{})",
                        raid.name, raid.slug
                    );
                }
            }
        }
    }
    if !not_on_codex.is_empty() {
        println!("{} monsters not on codex:", not_on_codex.len());
        for monster in &not_on_codex {
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

    // Create the new monsters on the guide, if asked to.
    if fix && !missing_on_guide.is_empty() {
        for monster in &missing_on_guide {
            retry_once!(guide.admin_add_monster(monster.to_admin_monster(&data.guide)))?;
        }

        // Retrieve the new list of monsters, and keep only those we didn't know of before.
        let all_monsters = retry_once!(guide.admin_retrieve_monsters_list())?;
        let new_monsters = all_monsters
            .iter()
            .filter(|monster| data.guide.monsters.find_by_id(monster.id).is_none())
            .filter_map(
                // Retrieve the `AdminMonster` entry.
                |monster| match retry_once!(guide.admin_retrieve_monster_by_id(monster.id)) {
                    Ok(x) => Some(x),
                    Err(x) => {
                        println!(
                            "Failed to retrieve monster #{} (https://orna.guide/monsters?show={}): {}",
                            monster.id, monster.id, x
                        );
                        None
                    }
                },
            )
            .collect_vec();

        // Log what was added.
        println!(
            "Added {}/{} monsters on the guide:",
            new_monsters.len(),
            missing_on_guide.len()
        );
        for monster in &new_monsters {
            println!(
                "\t\x1B[0;32m- {:20} (https://orna.guide/monsters?show={})\x1B[0m",
                monster.name, monster.id
            );
        }

        // Add monsters into the data, so it can be used later.
        data.guide.monsters.monsters.extend(new_monsters);
    }

    Ok(())
}

#[allow(clippy::items_after_statements, clippy::too_many_lines)]
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
            let admin_events = admin_monster
                .get_event_ids(&data.guide.static_.spawns)
                .into_iter()
                .sorted()
                .collect_vec();
            static RISE_OF_KERB_STR: &str = "Rise of Kerberos";
            static RETURN_OF_KERB_STR: &str = "Return of Kerberos";
            static RISE_OF_PHOENIX_STR: &str = "Rise of the Phoenix";
            static RETURN_OF_PHOENIX_STR: &str = "Return of the Phoenix";
            let codex_events = codex_monster
                .events()
                .iter()
                .map(std::string::String::as_str)
                // TODO(ethiraric, 14/07/2022): Remove this once codex is updated.
                .chain({
                    if admin_monster.name.contains("Kerberos") {
                        vec![RISE_OF_KERB_STR, RETURN_OF_KERB_STR].into_iter()
                    } else {
                        vec![].into_iter()
                    }
                })
                // TODO(ethiraric, 08/02/2023): Remove this once codex is updated.
                .chain({
                    // Rise/Return of the Phoenix
                    if admin_monster.spawns.contains(&28) || admin_monster.spawns.contains(&38) {
                        vec![RISE_OF_PHOENIX_STR, RETURN_OF_PHOENIX_STR].into_iter()
                    } else {
                        vec![].into_iter()
                    }
                })
                .collect_vec()
                .try_to_guide_ids(&data.guide.static_)?
                .into_iter()
                .sorted()
                .dedup()
                .collect_vec();
            check.spawn_id_vec(
                "events",
                &admin_events,
                &codex_events,
                |monster, events| {
                    fix_spawn_field(monster, &admin_events, data, events, |monster| {
                        &mut monster.spawns
                    })
                },
                data,
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
                    fix_option_field(
                        monster,
                        |monster| Ok(&mut monster.family),
                        name,
                        |name| {
                            data.guide
                                .static_
                                .monster_families
                                .iter()
                                .find(|family| family.name == **name)
                                .map(|family| family.id)
                                .ok_or_else(|| {
                                    Kind::Misc(format!(
                                        "Failed to find family {} for monster {} (#{})",
                                        name, admin_monster.name, admin_monster.id
                                    ))
                                    .into_err()
                                })
                        },
                    )
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
                .chain({
                    // TODO(ethiraric, 08/02/2023): Remove this once codex is updated.
                    if admin_monster.name.ends_with("of Olympia") {
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
                            .map_or(false, |spawn| {
                                spawn.name != "Kingdom Raid" && spawn.name != "World Raid"
                            })
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
                                monster.spawns.push(spawn.id);
                            }
                        }
                    }
                    Ok(())
                },
            )?;

            // Abilities
            let admin_ability_ids = admin_monster
                .skills
                .iter()
                .copied()
                // TODO(ethiraric, 11/07/2022): Remove filter when the codex fixes Bind and Bite.
                .filter(|id| {
                    !data
                        .guide
                        .skills
                        .get_by_id(*id)
                        .unwrap()
                        .codex_uri
                        .is_empty()
                })
                .sorted()
                .collect::<Vec<_>>();
            let expected_ids = codex_monster
                .abilities()
                .try_to_guide_ids(&data.guide.skills)
                // TODO(ethiraric, 27/07/2022): Add diagnostics.
                .unwrap_or_else(|err| match err {
                    Error {
                        kind: Kind::PartialCodexMonsterAbilitiesConversion(ok, _),
                        ..
                    } => ok,
                    _ => panic!("try_to_guide_ids returned a weird error"),
                })
                .into_iter()
                .sorted()
                .collect::<Vec<_>>();
            // TODO(ethiraric, 17/10/22): Remove once we cycle all events. Skill slugs were
            // kebab-caseified.
            if expected_ids.is_empty() {
                // println!("Monster {} has no ability on codex.", codex_monster.name());
            } else {
                check.skill_id_vec(
                    "abilities",
                    &admin_ability_ids,
                    &expected_ids,
                    |monster: &mut AdminMonster, _| {
                        fix_abilities_field(
                            monster,
                            &admin_ability_ids,
                            data,
                            &expected_ids,
                            |monster| &mut monster.skills,
                        )
                    },
                    data,
                )?;
            }
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide monsters and the codex monsters.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    println!("\x1B[0;35mMatching Monsters\x1B[0m");
    list_missing(data, fix, guide)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
