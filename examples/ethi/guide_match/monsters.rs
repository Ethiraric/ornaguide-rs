use ornaguide_rs::{error::Error, guide::OrnaAdminGuide};

use crate::output::{CodexGenericMonster, OrnaData};

/// List items that are either:
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

/// Check for any mismatch between the guide items and the codex monsters.
pub fn perform(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    // check_stats(data, fix, guide)?;
    Ok(())
}
