use std::fmt::{Debug, Display};

use ornaguide_rs::error::Error;

use crate::{misc::diff_sorted_slices, output::OrnaData};

/// Compare the list of elements in a field and remove or add depending on what is expected.
/// Need to be able to convert to a common type (usually `String`).
pub fn fix_vec_field<'a, AdminEntity, AdminToVec, T: 'a, FnRemove, FnAdd>(
    admin: &mut AdminEntity,
    admin_to_vec: AdminToVec,
    expected_vec: &[T],
    fn_remove: FnRemove,
    fn_add: FnAdd,
) -> Result<(), Error>
where
    AdminToVec: FnOnce(&mut AdminEntity) -> Result<&'a Vec<T>, Error>,
    T: std::cmp::Ord + std::fmt::Debug,
    FnRemove: FnOnce(&mut AdminEntity, &Vec<&T>) -> Result<(), Error>,
    FnAdd: FnOnce(&mut AdminEntity, &Vec<&T>) -> Result<(), Error>,
{
    // Start by listing the elements from the guide.
    let admin_vec = admin_to_vec(admin)?;

    // Compute the diff between it and that from the codex.
    let (to_add, to_remove) = diff_sorted_slices(expected_vec, admin_vec);
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

/// Compare the list of abilities registered on the guide to those on the codex.
/// The match is made based on the codex_uri (that which is registered on the admin skill, and that
/// which is indicated on the codex).
pub fn fix_abilities_field<'a, AdminEntity, EntitySkillsGetter>(
    entity: &mut AdminEntity,
    entity_uris: &'a Vec<String>,
    data: &OrnaData,
    expected_skills_uris: &[String],
    entity_skills_getter: EntitySkillsGetter,
) -> Result<(), Error>
where
    EntitySkillsGetter: Fn(&mut AdminEntity) -> &mut Vec<u32>,
{
    fix_vec_field(
        entity,
        |_| -> Result<&'a Vec<String>, Error> { Ok(entity_uris) },
        expected_skills_uris,
        |entity, to_remove| {
            // Retain only skills that do not match any URI in `to_remove`.
            entity_skills_getter(entity).retain(|skill_id| {
                data.guide
                    .skills
                    .find_skill_by_id(*skill_id)
                    // If a matching skill is found, remove it if its URI is within `to_remove`.
                    .map(|skill| !to_remove.iter().any(|uri| **uri == skill.codex_uri))
                    // If a matching skill is not found, panic (this shouldn't happen).
                    .unwrap_or_else(|err| {
                        panic!("An entity has invalid skill id #{}: {}", skill_id, err)
                    })
            });
            Ok(())
        },
        |entity, to_add| {
            // Convert URIs to ids.
            let ids_to_add = to_add.iter().filter_map(|skill_codex_uri| {
                data.guide
                    .skills
                    .find_skill_by_codex_uri(skill_codex_uri)
                    .map(|skill| skill.id)
                    .map_err(|err| println!("{}", err))
                    .ok()
            });

            // Push ids into entity.
            let entity_skills = entity_skills_getter(entity);
            for skill_id in ids_to_add {
                entity_skills.push(skill_id);
            }
            Ok(())
        },
    )
}

/// Compare a single field and print an error message if they differ.
/// Requires `Debug` instead of `Display`.
/// Return whether the stats matched.
#[allow(clippy::too_many_arguments)]
pub fn check_field_debug<AdminEntity, AS, CS, Fixer, GuideRetriever, GuideSaver>(
    field_name: &str,
    entity_name: &str,
    entity_id: u32,
    admin_field: &AS,
    codex_field: &CS,
    fix: bool,
    fixer: Fixer,
    guide_retriever: GuideRetriever,
    guide_saver: GuideSaver,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Debug,
    CS: Debug + ?Sized,
    Fixer: FnOnce(&mut AdminEntity, &CS) -> Result<(), Error>,
    GuideRetriever: Fn(u32) -> Result<AdminEntity, Error>,
    GuideSaver: FnOnce(AdminEntity) -> Result<(), Error>,
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:<80?}\nguide= {:?}",
            entity_name, field_name, codex_field, admin_field
        );
        if fix {
            let mut entity = guide_retriever(entity_id)?;
            fixer(&mut entity, codex_field)?;
            guide_saver(entity)?;
            guide_retriever(entity_id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Compare a single field and print an error message if they differ.
/// Return whether the stats matched.
#[allow(clippy::too_many_arguments)]
pub fn check_field<AS, CS, Fixer, AdminEntity, GuideRetriever, GuideSaver>(
    field_name: &str,
    entity_name: &str,
    entity_id: u32,
    admin_field: &AS,
    codex_field: &CS,
    fix: bool,
    fixer: Fixer,
    guide_retriever: GuideRetriever,
    guide_saver: GuideSaver,
) -> Result<bool, Error>
where
    AS: PartialEq<CS> + Display,
    CS: Display,
    Fixer: FnOnce(&mut AdminEntity, &CS) -> Result<(), Error>,
    GuideRetriever: Fn(u32) -> Result<AdminEntity, Error>,
    GuideSaver: FnOnce(AdminEntity) -> Result<(), Error>,
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m codex= {:<20} guide= {:<20}",
            entity_name, field_name, codex_field, admin_field
        );
        if fix {
            let mut entity = guide_retriever(entity_id)?;
            fixer(&mut entity, codex_field)?;
            guide_saver(entity)?;
            guide_retriever(entity_id)?;
        }
        Ok(false)
    } else {
        Ok(true)
    }
}
