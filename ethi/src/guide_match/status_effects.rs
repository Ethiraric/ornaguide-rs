use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    misc::codex_effect_name_to_guide_name,
};

use crate::retry_once;

/// List status effects that are on the codex and not the guide, or on the codex and not on the
/// guide.
fn list_missing(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    // Make iterators over all item status effects: causes, cures, immunities and gives.
    let items = data.codex.items.items.iter();
    let item_causes = items
        .clone()
        .flat_map(|item| item.causes.iter().map(|cause| cause.name.as_str()));
    let item_cures = items
        .clone()
        .flat_map(|item| item.cures.iter().map(|cure| cure.name.as_str()));
    let item_immunities = items.clone().flat_map(|item| {
        item.immunities
            .iter()
            .map(|immunity| immunity.name.as_str())
    });
    let item_gives = items
        .clone()
        .flat_map(|item| item.gives.iter().map(|give| give.name.as_str()));

    // Make iterators over all skill status effects: causes and gives.
    let skills = data.codex.skills.skills.iter();
    let skill_causes = skills
        .clone()
        .flat_map(|skill| skill.causes.iter().map(|cause| cause.effect.as_str()));
    let skill_gives = skills
        .clone()
        .flat_map(|skill| skill.gives.iter().map(|give| give.effect.as_str()));

    // Chain those iterators into a giant single iterator. Dedup and sort by their guide names.
    let codex_status_effects = item_causes
        .chain(item_cures)
        .chain(item_immunities)
        .chain(item_gives)
        .chain(skill_causes)
        .chain(skill_gives)
        .unique()
        .map(codex_effect_name_to_guide_name)
        .sorted()
        .collect_vec();

    // List those that are missing on the guide and the codex.
    let missing_on_guide = codex_status_effects
        .iter()
        .filter(|name| {
            !data
                .guide
                .static_
                .status_effects
                .iter()
                .any(|effect| effect.name == **name)
        })
        .collect_vec();
    let not_on_codex = data
        .guide
        .static_
        .status_effects
        .iter()
        .filter(|effect| {
            !codex_status_effects
                .iter()
                .any(|codex_effect| effect.name == *codex_effect)
        })
        .collect_vec();

    // Display what's missing.
    if !missing_on_guide.is_empty() {
        println!(
            "{} status effects missing on guide:",
            missing_on_guide.len()
        );
        for item in missing_on_guide.iter() {
            println!("\t- {}", item);
        }
    }

    if !not_on_codex.is_empty() {
        println!("{} status effects not on codex:", not_on_codex.len());
        for item in not_on_codex.iter() {
            println!("\t- {}", item.name);
        }
    }

    // Create the new status effects on the guide, if asked to.
    if fix && !missing_on_guide.is_empty() {
        for status in missing_on_guide.iter() {
            retry_once!(guide.admin_add_status_effect(*status))?;
        }

        data.guide.static_.status_effects =
            retry_once!(guide.admin_retrieve_status_effects_list())?;
    }

    Ok(())
}

/// Check for any mismatch between the guide status effects and the codex status effects.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data, fix, guide)?;
    Ok(())
}
