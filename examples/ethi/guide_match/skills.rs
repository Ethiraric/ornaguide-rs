use itertools::Itertools;
use ornaguide_rs::{
    codex::SkillStatusEffects,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
};

use crate::{
    guide_match::checker::{fix_status_effects_field, Checker},
    output::OrnaData,
};

/// List skills that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen.
fn list_missing(data: &OrnaData) -> Result<(), Error> {
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
        .filter(|skill| data.guide.skills.get_match_for_codex_skill(*skill).is_err())
        .collect_vec();

    let not_on_codex = data
        .guide
        .skills
        .skills
        .iter()
        // Passive skills are not on the codex.
        .filter(|skill| skill.type_ != guide_passive_id)
        .filter(|skill| data.codex.skills.find_match_for_admin_skill(skill).is_err())
        .collect_vec();

    if !missing_on_guide.is_empty() {
        println!("Skills missing on guide:");
        for skill in missing_on_guide.iter() {
            println!(
                "\t- {} (https://playorna.com/codex/spells/{})",
                skill.name, skill.slug
            );
        }
    }

    if !not_on_codex.is_empty() {
        println!("Skills not on codex:");
        for skill in not_on_codex.iter() {
            println!(
                "\t- {} (https://orna.guide/skills?show={})",
                skill.name, skill.id
            );
        }
    }

    Ok(())
}

/// Compare fields of every codex skill and their counterpart on the guide.
/// Attempt to fix discrepancies.
fn check_fields(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for codex_skill in data.codex.skills.skills.iter() {
        if let Ok(admin_skill) = data.guide.skills.get_match_for_codex_skill(codex_skill) {
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
                |skill, _| {
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
    list_missing(data)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
