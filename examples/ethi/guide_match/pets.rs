use std::fmt::{Debug, Display};

use itertools::Itertools;
use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    pets::admin::AdminPet,
};

use crate::{
    misc::{diff_sorted_slices, VecSkillIds},
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

/// Compare a single field and print an error message if they differ.
/// Return whether the stats matched.
fn check_field<AS, CS, Fixer>(
    field_name: &str,
    pet: &AdminPet,
    admin_field: AS,
    codex_field: CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Display,
    CS: Display,
    Fixer: FnOnce(&mut AdminPet, &CS),
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m codex= {:<20} guide= {:<20}",
            pet.name, field_name, codex_field, admin_field
        );
        if fix {
            let mut admin_pet = guide.admin_retrieve_pet_by_id(pet.id)?;
            fixer(&mut admin_pet, &codex_field);
            guide.admin_save_pet(admin_pet)?;
            guide.admin_retrieve_pet_by_id(pet.id)?;
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
    admin_pet: &AdminPet,
    admin_field: AS,
    codex_field: CS,
    fix: bool,
    fixer: Fixer,
    guide: &OrnaAdminGuide,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Debug,
    CS: Debug,
    Fixer: FnOnce(&mut AdminPet, &CS),
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:<80?}\nguide= {:?}",
            admin_pet.name, field_name, codex_field, admin_field
        );
        if fix {
            let mut pet = guide.admin_retrieve_pet_by_id(admin_pet.id)?;
            fixer(&mut pet, &codex_field);
            guide.admin_save_pet(pet)?;
            guide.admin_retrieve_pet_by_id(admin_pet.id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare the list of elements in a field and remove or add depending on what is expected.
/// Need to be able to convert to a common type (usually `String`).
fn fix_vec_field<AdminEntity, AdminToVec, T, FnRemove, FnAdd>(
    admin: &mut AdminEntity,
    admin_to_vec: AdminToVec,
    expected_vec: &[T],
    fn_remove: FnRemove,
    fn_add: FnAdd,
) -> Result<(), Error>
where
    AdminToVec: FnOnce(&mut AdminEntity) -> Result<Vec<T>, Error>,
    T: std::cmp::Ord + std::fmt::Debug,
    FnRemove: FnOnce(&mut AdminEntity, &Vec<&T>) -> Result<(), Error>,
    FnAdd: FnOnce(&mut AdminEntity, &Vec<&T>) -> Result<(), Error>,
{
    // Start by listing the elements from the guide.
    let mut admin_vec = admin_to_vec(admin)?;
    admin_vec.sort_unstable();

    // Compute the diff between it and that from the codex.
    let (to_add, to_remove) = diff_sorted_slices(expected_vec, &admin_vec);
    if !to_add.is_empty() {
        println!("\x1B[0;32mSuggest adding: {:?}\x1B[0m", to_add);
    }
    if !to_remove.is_empty() {
        println!("\x1B[0;31mSuggest removing: {:?}\x1B[0m", to_remove);
    }

    // Remove unneeded elements.
    if !to_remove.is_empty() {
        fn_remove(admin, &to_remove)?;
    }
    // Add missing elements.
    if !to_add.is_empty() {
        fn_add(admin, &to_add)?;
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
                pet,
                pet.name.clone(),
                follower.name.clone(),
                fix,
                |pet, name| pet.name = name.clone(),
                guide,
            )?;
            // Image name
            check_field(
                "image_name",
                pet,
                pet.image_name.clone(),
                follower.icon.clone(),
                fix,
                |pet, image_name| pet.image_name = image_name.clone(),
                guide,
            )?;
            // Description
            check_field(
                "description",
                pet,
                pet.description.clone(),
                if !follower.description.is_empty() {
                    follower.description.clone()
                } else {
                    ".".to_string()
                },
                fix,
                |pet, description| {
                    pet.description = description.clone();
                },
                guide,
            )?;
            // Tier
            check_field(
                "tier",
                pet,
                pet.tier,
                follower.tier,
                fix,
                |skill, tier| {
                    skill.tier = *tier;
                },
                guide,
            )?;
            // Abilities
            check_field_debug(
                "abilities",
                pet,
                pet.skills.guide_skill_ids_to_codex_uri(data),
                follower
                    .abilities
                    .iter()
                    .map(|ability| ability.uri.clone())
                    .sorted()
                    .collect::<Vec<_>>(),
                fix,
                |pet, expected_skills| {
                    fix_vec_field(
                        pet,
                        |pet| Ok(pet.skills.guide_skill_ids_to_codex_uri(data)),
                        expected_skills,
                        |pet, to_remove| {
                            // Retain only skills that do not match any URI in `to_remove`.
                            pet.skills.retain(|skill_id| {
                                if let Some(skill) = data
                                    .guide
                                    .skills
                                    .skills
                                    .iter()
                                    .find(|skill| skill.id == *skill_id)
                                {
                                    !to_remove.iter().any(|uri| **uri == skill.codex_uri)
                                } else {
                                    false
                                }
                            });
                            Ok(())
                        },
                        |pet, to_add| {
                            // Convert URIs to ids.
                            let ids_to_add = to_add.iter().filter_map(|skill_codex_uri| {
                                if let Some(skill) = data
                                    .guide
                                    .skills
                                    .skills
                                    .iter()
                                    .find(|skill| skill.codex_uri == **skill_codex_uri)
                                {
                                    Some(skill.id)
                                } else {
                                    println!(
                                        "Failed to find guide skill with codex uri {}",
                                        skill_codex_uri
                                    );
                                    None
                                }
                            });

                            // Push ids into pet.
                            for skill_id in ids_to_add {
                                pet.skills.push(skill_id);
                            }
                            Ok(())
                        },
                    )
                    .unwrap()
                },
                guide,
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
