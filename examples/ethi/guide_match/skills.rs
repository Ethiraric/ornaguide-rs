use std::fmt::{Debug, Display};

use itertools::Itertools;
use ornaguide_rs::{
    codex::SkillStatusEffects,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    skills::admin::AdminSkill,
};

use crate::output::OrnaData;

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
        .filter(|skill| {
            data.guide
                .skills
                .find_match_for_codex_skill(*skill)
                .is_err()
        })
        .collect::<Vec<_>>();

    let not_on_codex = data
        .guide
        .skills
        .skills
        .iter()
        // Passive skills are not on the codex.
        .filter(|skill| skill.type_ != guide_passive_id)
        .filter(|skill| data.codex.skills.find_match_for_admin_skill(skill).is_err())
        .collect::<Vec<_>>();

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

/// Compare a single field and print an error message if they differ.
/// Return whether the stats matched.
fn check_field<AS, CS, Fixer>(
    field_name: &str,
    admin_skill: &AdminSkill,
    admin_field: AS,
    codex_field: CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Display,
    CS: Display,
    Fixer: FnOnce(&mut AdminSkill, &CS),
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m codex= {:<20} guide= {:<20}",
            admin_skill.name, field_name, codex_field, admin_field
        );
        if fix {
            let mut skill = guide.admin_retrieve_skill_by_id(admin_skill.id)?;
            fixer(&mut skill, &codex_field);
            guide.admin_save_skill(skill)?;
            guide.admin_retrieve_skill_by_id(admin_skill.id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare a single field and print an error message if they differ.
/// Requires `Debug` instead of `Display`.
/// Return whether the stats matched.
fn check_field_debug<AS, CS, Fixer>(
    field_name: &str,
    admin_skill: &AdminSkill,
    admin_field: AS,
    codex_field: CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Debug,
    CS: Debug,
    Fixer: FnOnce(&mut AdminSkill, &CS),
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:<80?}\nguide= {:?}",
            admin_skill.name, field_name, codex_field, admin_field
        );
        if fix {
            let mut skill = guide.admin_retrieve_skill_by_id(admin_skill.id)?;
            fixer(&mut skill, &codex_field);
            guide.admin_save_skill(skill)?;
            guide.admin_retrieve_skill_by_id(admin_skill.id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare fields of every codex skill and their counterpart on the guide.
/// Attempt to fix discrepancies.
fn check_fields(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for codex_skill in data.codex.skills.skills.iter() {
        if let Ok(admin_skill) = data.guide.skills.find_match_for_codex_skill(codex_skill) {
            // Description
            check_field(
                "description",
                admin_skill,
                admin_skill.description.clone(),
                if !codex_skill.description.is_empty() {
                    codex_skill.description.clone()
                } else {
                    ".".to_string()
                },
                fix,
                |skill, description| {
                    skill.description = description.clone();
                },
                guide,
            )?;
            // Tier
            check_field(
                "tier",
                admin_skill,
                admin_skill.tier,
                codex_skill.tier,
                fix,
                |skill, tier| {
                    skill.tier = *tier;
                },
                guide,
            )?;
            // Causes
            check_field_debug(
                "causes",
                admin_skill,
                &admin_skill
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
                &codex_skill.causes.to_guide_names(),
                fix,
                |item, _| match codex_skill.causes.try_to_guide_ids(&data.guide.static_) {
                    Ok(ids) => item.causes = ids,
                    Err(err) => println!("Failed to convert causes for {}: {}", item.name, err),
                },
                guide,
            )?;
            // Gives
            // I think I have no way of translating those two.
            if codex_skill.slug != "CerusDefendPhys" && codex_skill.slug != "CerusDefendMag" {
                check_field_debug(
                    "gives",
                    admin_skill,
                    &admin_skill
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
                    &codex_skill.gives.to_guide_names(),
                    fix,
                    |item, _| match codex_skill.gives.try_to_guide_ids(&data.guide.static_) {
                        Ok(ids) => item.gives = ids,
                        Err(err) => println!("Failed to convert gives for {}: {}", item.name, err),
                    },
                    guide,
                )?;
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
