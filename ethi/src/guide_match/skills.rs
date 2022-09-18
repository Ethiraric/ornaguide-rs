use itertools::Itertools;
use ornaguide_rs::{
    codex::SkillStatusEffects,
    data::OrnaData,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    skills::admin::AdminSkill,
};

use crate::{
    guide_match::checker::{fix_status_effects_field, Checker},
    retry_once,
};

/// List skills that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen.
fn list_missing(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    // Passives are not listed on the codex. We get the id to filter out passive skills.
    let guide_passive_id = data
        .guide
        .static_
        .skill_types
        .iter()
        .find(|type_| type_.name == "Passive")
        .map(|type_| type_.id)
        .unwrap();

    let missing_on_guide = data
        .codex
        .skills
        .skills
        .iter()
        .filter(|skill| data.guide.skills.get_by_slug(&skill.slug).is_err())
        .collect_vec();
    let not_on_codex = data
        .guide
        .skills
        .skills
        .iter()
        // Passive skills are not on the codex.
        .filter(|skill| skill.type_ != guide_passive_id)
        .filter(|skill| data.codex.skills.get_by_uri(&skill.codex_uri).is_err())
        .collect_vec();

    if !missing_on_guide.is_empty() {
        println!("{} skills missing on guide:", missing_on_guide.len());
        for skill in missing_on_guide.iter() {
            println!(
                "\t- {:20} (https://playorna.com/codex/spells/{}/)",
                skill.name, skill.slug
            );
        }
    }
    if !not_on_codex.is_empty() {
        println!("{} skills not on codex:", not_on_codex.len());
        for skill in not_on_codex.iter() {
            println!(
                "\t- {:20} (https://orna.guide/skills?show={})",
                skill.name, skill.id
            );
        }
    }

    // Create the new skills on the guide, if asked to.
    if fix && !missing_on_guide.is_empty() {
        for skill in missing_on_guide.iter() {
            retry_once!(guide.admin_add_skill(skill.try_to_admin_skill(&data.guide.static_)?))?;
        }

        // Retrieve the new list of skills, and keep only those we didn't know of before.
        let all_skills = retry_once!(guide.admin_retrieve_skills_list())?;
        let new_skills = all_skills
            .iter()
            .filter(|skill| data.guide.skills.find_by_id(skill.id).is_none())
            .filter_map(
                // Retrieve the `AdminSkill` entry.
                |skill| match retry_once!(guide.admin_retrieve_skill_by_id(skill.id)) {
                    Ok(x) => Some(x),
                    Err(x) => {
                        println!(
                            "Failed to retrieve skill #{} (https://orna.guide/skills?show={}): {}",
                            skill.id, skill.id, x
                        );
                        None
                    }
                },
            )
            .collect_vec();

        // Log what was added.
        println!(
            "Added {}/{} skills on the guide:",
            new_skills.len(),
            missing_on_guide.len()
        );
        for skill in new_skills.iter() {
            println!(
                "\t\x1B[0;32m- {:20} (https://orna.guide/skills?show={})\x1B[0m",
                skill.name, skill.id
            );
        }

        // Add skills into the data, so it can be used later.
        data.guide.skills.skills.extend(new_skills);
    }

    Ok(())
}

/// Compare fields of every codex skill and their counterpart on the guide.
/// Attempt to fix discrepancies.
fn check_fields(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for codex_skill in data.codex.skills.skills.iter() {
        if let Ok(admin_skill) = data.guide.skills.get_by_slug(&codex_skill.slug) {
            let check = Checker {
                entity_name: &admin_skill.name,
                entity_id: admin_skill.id,
                fix,
                golden: |id| guide.admin_retrieve_skill_by_id(id),
                saver: |skill| guide.admin_save_skill(skill),
            };

            // Description
            let codex_description = if !codex_skill.description.is_empty() {
                codex_skill.description.clone()
            } else {
                ".".to_string()
            };
            check.display(
                "description",
                &admin_skill.description,
                &codex_description,
                |skill, description| {
                    skill.description = description.clone();
                    Ok(())
                },
            )?;

            // Tier
            check.display(
                "tier",
                &admin_skill.tier,
                &codex_skill.tier,
                |skill, tier| {
                    skill.tier = *tier;
                    Ok(())
                },
            )?;

            // Bought at arcanist
            check.display(
                "bought",
                &admin_skill.bought,
                &codex_skill.bought_at_arcanist(),
                |skill, bought| {
                    skill.bought = *bought;
                    Ok(())
                },
            )?;

            // Causes
            let admin_causes = admin_skill.causes.iter().cloned().sorted().collect_vec();
            let codex_causes = codex_skill
                .causes
                .try_to_guide_ids(&data.guide.static_)?
                .into_iter()
                .sorted()
                .collect_vec();
            check.status_effect_id_vec(
                "causes",
                &admin_causes,
                &codex_causes,
                |skill: &mut AdminSkill, _| {
                    fix_status_effects_field(skill, &admin_causes, data, &codex_causes, |skill| {
                        &mut skill.causes
                    })
                },
                data,
            )?;

            // Gives
            // I think I have no way of translating those two.
            if codex_skill.slug != "CerusDefendPhys" && codex_skill.slug != "CerusDefendMag" {
                let admin_gives = admin_skill.gives.iter().cloned().sorted().collect_vec();
                let codex_gives = codex_skill
                    .gives
                    .try_to_guide_ids(&data.guide.static_)?
                    .into_iter()
                    .sorted()
                    .collect_vec();
                check.debug("gives", &admin_gives, &codex_gives, |skill, _| {
                    fix_status_effects_field(skill, &admin_gives, data, &codex_gives, |skill| {
                        &mut skill.gives
                    })
                })?;
            }
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide skills and the codex skills.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data, fix, guide)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
