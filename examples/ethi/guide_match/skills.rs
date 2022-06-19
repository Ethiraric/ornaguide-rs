use ornaguide_rs::{error::Error, guide::OrnaAdminGuide};

use crate::output::OrnaData;

/// List skills that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen.
fn list_missing(data: &OrnaData) -> Result<(), Error> {
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
/// Check for any mismatch between the guide skills and the codex skills.
pub fn perform(data: &mut OrnaData, _: bool, _: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    // check_fields(data, fix, guide)?;
    Ok(())
}
