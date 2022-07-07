use itertools::Itertools;
use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    pets::admin::AdminPet,
};

use crate::{
    guide_match::misc::{check_field, check_field_debug, fix_abilities_field},
    misc::VecSkillIds,
    output::OrnaData,
};

/// List pets that are either:
///   - On the guide, but missing on the codex.
///   - On the codex, but missing on the guide.
/// None of these should happen.
fn list_missing(data: &OrnaData) -> Result<(), Error> {
    let missing_on_guide = data
        .codex
        .followers
        .followers
        .iter()
        .filter(|follower| {
            data.guide
                .pets
                .find_match_for_codex_follower(*follower)
                .is_err()
        })
        .collect::<Vec<_>>();

    let not_on_codex = data
        .guide
        .pets
        .pets
        .iter()
        .filter(|pet| data.codex.followers.find_match_for_admin_pet(pet).is_err())
        .collect::<Vec<_>>();

    if !missing_on_guide.is_empty() {
        println!("Followers missing on guide:");
        for follower in missing_on_guide.iter() {
            println!(
                "\t- {} (https://playorna.com/codex/followers/{})",
                follower.name, follower.slug
            );
        }
    }

    if !not_on_codex.is_empty() {
        println!("Pets not on codex:");
        for pet in not_on_codex.iter() {
            println!("\t- {} (https://orna.guide/pets?show={})", pet.name, pet.id);
        }
    }

    Ok(())
}

/// Compare fields of every codex follower and their counterpart on the guide.
/// Attempt to fix discrepancies.
fn check_fields(data: &OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    for follower in data.codex.followers.followers.iter() {
        if let Ok(pet) = data.guide.pets.find_match_for_codex_follower(follower) {
            // Name
            check_field(
                "name",
                &pet.name,
                pet.id,
                &pet.name,
                &follower.name,
                fix,
                |pet: &mut AdminPet, name| {
                    pet.name = name.clone();
                    Ok(())
                },
                |id| guide.admin_retrieve_pet_by_id(id),
                |pet| guide.admin_save_pet(pet),
            )?;
            // Image name
            check_field(
                "image_name",
                &pet.name,
                pet.id,
                &pet.image_name,
                &follower.icon,
                fix,
                |pet: &mut AdminPet, image_name| {
                    pet.image_name = image_name.clone();
                    Ok(())
                },
                |id| guide.admin_retrieve_pet_by_id(id),
                |pet| guide.admin_save_pet(pet),
            )?;
            // Description
            let follower_description = if !follower.description.is_empty() {
                follower.description.clone()
            } else {
                ".".to_string()
            };
            check_field(
                "description",
                &pet.name,
                pet.id,
                &pet.description,
                &follower_description,
                fix,
                |pet: &mut AdminPet, description| {
                    pet.description = description.to_string();
                    Ok(())
                },
                |id| guide.admin_retrieve_pet_by_id(id),
                |pet| guide.admin_save_pet(pet),
            )?;
            // Tier
            check_field(
                "tier",
                &pet.name,
                pet.id,
                &pet.tier,
                &follower.tier,
                fix,
                |skill: &mut AdminPet, tier| {
                    skill.tier = *tier;
                    Ok(())
                },
                |id| guide.admin_retrieve_pet_by_id(id),
                |pet| guide.admin_save_pet(pet),
            )?;
            // Abilities
            let expected_skills_uris = follower
                .abilities
                .iter()
                .map(|ability| ability.uri.clone())
                .sorted()
                .collect::<Vec<_>>();
            let pet_skills_uris = pet.skills.guide_skill_ids_to_codex_uri(data);
            check_field_debug(
                "abilities",
                &pet.name,
                pet.id,
                &pet_skills_uris,
                &expected_skills_uris,
                fix,
                |pet: &mut AdminPet, _| {
                    fix_abilities_field(pet, &pet_skills_uris, data, &expected_skills_uris, |pet| {
                        &mut pet.skills
                    })
                },
                |id| guide.admin_retrieve_pet_by_id(id),
                |pet| guide.admin_save_pet(pet),
            )?;
        }
    }
    Ok(())
}

/// Check for any mismatch between the guide pets and the codex pets.
pub fn perform(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    list_missing(data)?;
    check_fields(data, fix, guide)?;
    Ok(())
}
