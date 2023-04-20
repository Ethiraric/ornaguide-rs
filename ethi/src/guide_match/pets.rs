use itertools::Itertools;
use ornaguide_rs::{
    data::OrnaData,
    error::{Error, ErrorKind},
    guide::{AdminGuide, OrnaAdminGuide},
    pets::admin::AdminPet,
};

use crate::{
    guide_match::checker::{fix_abilities_field, Checker},
    retry_once,
};

use super::misc::CodexAbilities;

/// List pets that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen.
fn list_missing(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    let missing_on_guide = data
        .codex
        .followers
        .followers
        .iter()
        .filter(|follower| data.guide.pets.get_by_slug(&follower.slug).is_err())
        .collect_vec();
    let not_on_codex = data
        .guide
        .pets
        .pets
        .iter()
        .filter(|pet| data.codex.followers.get_by_uri(&pet.codex_uri).is_err())
        .collect_vec();

    if !missing_on_guide.is_empty() {
        println!("{} followers missing on guide:", missing_on_guide.len());
        for follower in missing_on_guide.iter() {
            println!(
                "\t- {} (https://playorna.com/codex/followers/{})",
                follower.name, follower.slug
            );
        }
    }
    if !not_on_codex.is_empty() {
        println!("{} pets not on codex:", not_on_codex.len());
        for pet in not_on_codex.iter() {
            println!("\t- {} (https://orna.guide/pets?show={})", pet.name, pet.id);
        }
    }

    // Create the new pets on the guide, if asked to.
    if fix && !missing_on_guide.is_empty() {
        for pet in missing_on_guide.iter() {
            retry_once!(guide.admin_add_pet(pet.try_to_admin_pet(&data.guide)?))?;
        }

        // Retrieve the new list of pets, and keep only those we didn't know of before.
        let all_pets = retry_once!(guide.admin_retrieve_pets_list())?;
        let new_pets = all_pets
            .iter()
            .filter(|pet| data.guide.pets.find_by_id(pet.id).is_none())
            .filter_map(
                // Retrieve the `AdminPet` entry.
                |pet| match retry_once!(guide.admin_retrieve_pet_by_id(pet.id)) {
                    Ok(x) => Some(x),
                    Err(x) => {
                        println!(
                            "Failed to retrieve pet #{} (https://orna.guide/pets?show={}): {}",
                            pet.id, pet.id, x
                        );
                        None
                    }
                },
            )
            .collect_vec();

        // Log what was added.
        println!(
            "Added {}/{} pets on the guide:",
            new_pets.len(),
            missing_on_guide.len()
        );
        for pet in new_pets.iter() {
            println!(
                "\t\x1B[0;32m- {:20} (https://orna.guide/pets?show={})\x1B[0m",
                pet.name, pet.id
            );
        }

        // Add pets into the data, so it can be used later.
        data.guide.pets.pets.extend(new_pets);
    }

    Ok(())
}

/// Compare fields of every codex follower and their counterpart on the guide.
/// Attempt to fix discrepancies.
fn check_fields(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for follower in data.codex.followers.followers.iter() {
        if let Ok(pet) = data.guide.pets.get_by_slug(&follower.slug) {
            let check = Checker {
                entity_name: &pet.name,
                entity_id: pet.id,
                fix,
                golden: |id| guide.admin_retrieve_pet_by_id(id),
                saver: |pet| guide.admin_save_pet(pet),
            };

            // Name
            check.display(
                "name",
                &pet.name,
                &follower.name,
                |pet: &mut AdminPet, name| {
                    pet.name = name.clone();
                    Ok(())
                },
            )?;

            // Image name
            check.display(
                "image_name",
                &pet.image_name,
                &follower.icon,
                |pet: &mut AdminPet, image_name| {
                    pet.image_name = image_name.clone();
                    Ok(())
                },
            )?;

            // Description
            let follower_description = if !follower.description.is_empty() {
                follower.description.clone()
            } else {
                ".".to_string()
            };
            check.display(
                "description",
                &pet.description,
                &follower_description,
                |pet: &mut AdminPet, description| {
                    pet.description = description.to_string();
                    Ok(())
                },
            )?;

            // Tier
            check.display(
                "tier",
                &pet.tier,
                &follower.tier,
                |skill: &mut AdminPet, tier| {
                    skill.tier = *tier;
                    Ok(())
                },
            )?;

            // Abilities
            let pet_skills_ids = pet
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
                .collect_vec();
            let expected_skills_ids = follower
                .abilities
                .try_to_guide_ids(&data.guide.skills)
                // TODO(ethiraric, 27/07/2022): Add diagnostics.
                .unwrap_or_else(|err| match err {
                    Error {
                        kind: ErrorKind::PartialCodexFollowerAbilitiesConversion(ok, _),
                        ..
                    } => ok,
                    _ => panic!("try_to_guide_ids returned a weird error"),
                })
                .into_iter()
                .sorted()
                .collect_vec();
            // TODO(ethiraric, 17/10/22): Remove once we cycle all events. Skill slugs were
            // kebab-caseified.
            if !expected_skills_ids.is_empty() {
                check.skill_id_vec(
                    "abilities",
                    &pet_skills_ids,
                    &expected_skills_ids,
                    |pet: &mut AdminPet, _| {
                        fix_abilities_field(
                            pet,
                            &pet_skills_ids,
                            data,
                            &expected_skills_ids,
                            |pet| &mut pet.skills,
                        )
                    },
                    data,
                )?;
            } else {
                // println!("Follower {} has no ability on codex.", follower.name);
            }
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide pets and the codex pets.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    println!("\x1B[0;35mMatching Pets\x1B[0m");
    list_missing(data, fix, guide)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
