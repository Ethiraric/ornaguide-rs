use itertools::Itertools;
use ornaguide_rs::{
    data::{CodexGenericMonster, OrnaData},
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    monsters::admin::AdminMonster,
};

use crate::guide_match::{
    checker::{fix_abilities_field, fix_option_field, fix_spawn_field, Checker},
    misc::{CodexAbilities, EventsNames},
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
            let codex_events = codex_monster
                .events()
                .iter()
                .map(|s| s.as_str())
                // TODO(ethiraric, 14/07/2022): Remove this once codex is updated.
                .chain({
                    if admin_monster.name.contains("Kerberos") {
                        vec![RISE_OF_KERB_STR].into_iter()
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
                                    Error::Misc(format!(
                                        "Failed to find family {} for monster {} (#{})",
                                        name, admin_monster.name, admin_monster.id
                                    ))
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
            let admin_ability_ids = admin_monster
                .skills
                .iter()
                .cloned()
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
                .try_to_guide_ids(&data.guide.skills)?
                .into_iter()
                .sorted()
                .collect::<Vec<_>>();
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
    Ok(())
}

/// Check for any mismatch between the guide monsters and the codex monsters.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
