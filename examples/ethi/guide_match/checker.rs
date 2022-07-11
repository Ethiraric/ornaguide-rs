use ornaguide_rs::error::Error;

use std::fmt::{Debug, Display};

use crate::{misc::diff_sorted_slices, output::OrnaData};

/// Compare the option in a field and fix it to what is expected.
/// The conversion function is used to translate from the codex to the guide.
pub fn fix_option_field<'a, AdminEntity, AdminToOption, T: 'a, U, FnConvert>(
    admin: &'a mut AdminEntity,
    admin_to_option: AdminToOption,
    expected_option: &Option<U>,
    fn_convert: FnConvert,
) -> Result<(), Error>
where
    AdminToOption: FnOnce(&'a mut AdminEntity) -> Result<&'a mut Option<T>, Error>,
    T: std::cmp::Ord + std::fmt::Debug,
    FnConvert: FnOnce(&U) -> Result<T, Error>,
{
    let admin_option = admin_to_option(admin)?;
    match expected_option {
        Some(expected) => {
            *admin_option = Some(fn_convert(expected)?);
        }
        None => *admin_option = None,
    }
    Ok(())
}

/// Compare the list of elements in a field and split them into a list to add and one to remove.
/// Call the given callable accordingly.
pub fn fix_vec_field<
    'a,
    AdminEntity,
    AdminToVec,
    T: 'a,
    FnRemove,
    FnAdd,
    FnToDebuggable,
    Debuggable,
>(
    admin: &mut AdminEntity,
    admin_to_vec: AdminToVec,
    expected_vec: &'a [T],
    fn_remove: FnRemove,
    fn_add: FnAdd,
    to_str: FnToDebuggable,
) -> Result<(), Error>
where
    AdminToVec: FnOnce(&mut AdminEntity) -> Result<&'a Vec<T>, Error>,
    T: std::cmp::Ord,
    FnRemove: FnOnce(&mut AdminEntity, &Vec<&'a T>) -> Result<(), Error>,
    FnAdd: FnOnce(&mut AdminEntity, &Vec<&'a T>) -> Result<(), Error>,
    FnToDebuggable: Fn(&T) -> Debuggable,
    Debuggable: std::fmt::Debug,
{
    // Start by listing the elements from the guide.
    let admin_vec = admin_to_vec(admin)?;
    // Compute the diff between it and that from the codex.
    let (to_add, to_remove) = diff_sorted_slices(expected_vec, admin_vec);
    if !to_add.is_empty() {
        println!(
            "\x1B[0;32mSuggest adding: {:?}\x1B[0m",
            to_add.iter().map(|t| to_str(t)).collect::<Vec<_>>()
        );
    }
    if !to_remove.is_empty() {
        println!(
            "\x1B[0;31mSuggest removing: {:?}\x1B[0m",
            to_remove.iter().map(|t| to_str(t)).collect::<Vec<_>>()
        );
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

/// Compare two list of ids: one from the guide and the other one from the codex.
/// Data from the codex has to be converted to guide ids before calling this function.
/// The "id -> debuggable" conversion is used only for displaying purposes.
pub fn fix_vec_id_field<AdminEntity, EntityIdsGetter, IdToDebuggable, Debuggable>(
    entity: &mut AdminEntity,
    entity_ids: &Vec<u32>,
    expected_ids: &[u32],
    entity_ids_getter: EntityIdsGetter,
    id_to_debuggable: IdToDebuggable,
) -> Result<(), Error>
where
    EntityIdsGetter: Fn(&mut AdminEntity) -> &mut Vec<u32>,
    IdToDebuggable: Fn(&u32) -> Debuggable,
    Debuggable: std::fmt::Debug,
{
    fix_vec_field(
        entity,
        |_| -> Result<&Vec<u32>, Error> { Ok(entity_ids) },
        expected_ids,
        |entity, to_remove| {
            entity_ids_getter(entity).retain(|id| !to_remove.contains(&id));
            Ok(())
        },
        |entity, to_add| {
            let entity_ids = entity_ids_getter(entity);
            for id in to_add {
                entity_ids.push(**id);
            }
            Ok(())
        },
        id_to_debuggable,
    )
}

/// Compare the list of abilities registered on the guide to those on the codex.
/// The match is made based on the codex_uri (that which is registered on the admin skill, and that
/// which is indicated on the codex).
pub fn fix_abilities_field<AdminEntity, EntitySkillsGetter>(
    entity: &mut AdminEntity,
    entity_ids: &Vec<u32>,
    data: &OrnaData,
    expected_skills_ids: &[u32],
    entity_skills_getter: EntitySkillsGetter,
) -> Result<(), Error>
where
    EntitySkillsGetter: Fn(&mut AdminEntity) -> &mut Vec<u32>,
{
    fix_vec_id_field(
        entity,
        entity_ids,
        expected_skills_ids,
        entity_skills_getter,
        // Id to debuggable
        |id| data.guide.skills.get_by_id(*id).map(|skill| &skill.name),
    )
}

/// Compare the list of status effects registered on the guide to those on the codex.
/// The match is made based on the status name. The names given in `expected_names` have to be
/// those from the guide, not from the codex.
pub fn fix_status_effects_field<AdminEntity, EntityStatusEffectsGetter>(
    entity: &mut AdminEntity,
    entity_ids: &Vec<u32>,
    data: &OrnaData,
    expected_ids: &[u32],
    entity_skills_getter: EntityStatusEffectsGetter,
) -> Result<(), Error>
where
    EntityStatusEffectsGetter: Fn(&mut AdminEntity) -> &mut Vec<u32>,
{
    fix_vec_id_field(
        entity,
        entity_ids,
        expected_ids,
        entity_skills_getter,
        // Id to debuggable
        |id| {
            data.guide
                .static_
                .status_effects
                .iter()
                .find(|status| status.id == *id)
                .map(|status| status.name.as_str())
                .ok_or_else(|| Error::Misc(format!("Failed to find status effect #{}", id)))
        },
    )
}

/// Compare a `Vec` field and print an error message if they differ.
/// The `Vec` elements are passed through a formatter.
/// Return whether the stats matched.
#[allow(clippy::too_many_arguments)]
pub fn check_field_vec_formatter<
    AdminEntity,
    AS,
    CS,
    Fixer,
    GuideRetriever,
    GuideSaver,
    AFormatter,
    CFormatter,
    ADebuggable,
    CDebuggable,
>(
    field_name: &str,
    entity_name: &str,
    entity_id: u32,
    admin_field: &Vec<AS>,
    codex_field: &Vec<CS>,
    fix: bool,
    fixer: Fixer,
    guide_retriever: GuideRetriever,
    guide_saver: GuideSaver,
    admin_formatter: AFormatter,
    codex_formatter: CFormatter,
) -> Result<bool, Error>
where
    AS: PartialEq<CS>,
    Fixer: FnOnce(&mut AdminEntity, &Vec<CS>) -> Result<(), Error>,
    GuideRetriever: Fn(u32) -> Result<AdminEntity, Error>,
    GuideSaver: FnOnce(AdminEntity) -> Result<(), Error>,
    AFormatter: Fn(&AS) -> ADebuggable,
    CFormatter: Fn(&CS) -> CDebuggable,
    ADebuggable: Debug,
    CDebuggable: Debug,
{
    if admin_field != codex_field {
        println!(
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:?}\nguide= {:?}",
            entity_name,
            field_name,
            codex_field.iter().map(codex_formatter).collect::<Vec<_>>(),
            admin_field.iter().map(admin_formatter).collect::<Vec<_>>(),
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
            "\x1B[0;34m{:30}:{:11}:\x1B[0m\ncodex= {:?}\nguide= {:?}",
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

/// Helper structure to capture the context when fixing an entity.
/// Used to reduce code duplication by providing sensible default arguments to `check_field` and
/// `check_field_debug`.
pub struct Checker<'a, AdminEntity, Retriever, Saver>
where
    Retriever: Fn(u32) -> Result<AdminEntity, Error>,
    Saver: Fn(AdminEntity) -> Result<(), Error>,
{
    /// The name of the entity we inspect.
    pub entity_name: &'a str,
    /// The id of the entity we inspect.
    pub entity_id: u32,
    /// Whether changes should be written back to the guide.
    pub fix: bool,
    /// The function used to retrieve the entity from the guide.
    pub golden: Retriever,
    /// The function used to commit the entity to the guide.
    pub saver: Saver,
}

impl<'a, AdminEntity, Retriever, Saver> Checker<'a, AdminEntity, Retriever, Saver>
where
    Retriever: Fn(u32) -> Result<AdminEntity, Error>,
    Saver: Fn(AdminEntity) -> Result<(), Error>,
{
    /// Check a particular field.
    /// The field's values (`admin_field` and `codex_field`) must implement `std::fmt::Display`.
    pub fn display<AS, CS, Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &AS,
        codex_field: &CS,
        fixer: Fixer,
    ) -> Result<bool, Error>
    where
        AS: PartialEq<CS> + Display,
        CS: Display,
        Fixer: FnOnce(&mut AdminEntity, &CS) -> Result<(), Error>,
    {
        check_field(
            field_name,
            self.entity_name,
            self.entity_id,
            admin_field,
            codex_field,
            self.fix,
            fixer,
            &self.golden,
            &self.saver,
        )
    }

    /// Check a particular field.
    /// The field's values (`admin_field` and `codex_field`) must implement `std::fmt::Debug`.
    pub fn debug<AS, CS, Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &AS,
        codex_field: &CS,
        fixer: Fixer,
    ) -> Result<bool, Error>
    where
        AS: PartialEq<CS> + Debug,
        CS: Debug + ?Sized,
        Fixer: FnOnce(&mut AdminEntity, &CS) -> Result<(), Error>,
    {
        check_field_debug(
            field_name,
            self.entity_name,
            self.entity_id,
            admin_field,
            codex_field,
            self.fix,
            fixer,
            &self.golden,
            &self.saver,
        )
    }

    /// Check a particular field.
    /// The field's values (`admin_field` and `codex_field`) are formatted through the given formatters.
    #[allow(dead_code)]
    pub fn vec<AS, CS, Fixer, AFormatter, CFormatter, ADebuggable, CDebuggable>(
        &'a self,
        field_name: &str,
        admin_field: &Vec<AS>,
        codex_field: &Vec<CS>,
        fixer: Fixer,
        admin_formatter: AFormatter,
        codex_formatter: CFormatter,
    ) -> Result<bool, Error>
    where
        AS: PartialEq<CS>,
        Fixer: FnOnce(&mut AdminEntity, &Vec<CS>) -> Result<(), Error>,
        AFormatter: Fn(&AS) -> ADebuggable,
        CFormatter: Fn(&CS) -> CDebuggable,
        ADebuggable: Debug,
        CDebuggable: Debug,
    {
        check_field_vec_formatter(
            field_name,
            self.entity_name,
            self.entity_id,
            admin_field,
            codex_field,
            self.fix,
            fixer,
            &self.golden,
            &self.saver,
            admin_formatter,
            codex_formatter,
        )
    }

    /// Check a field containing guide skill ids.
    pub fn skill_id_vec<Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &Vec<u32>,
        codex_field: &Vec<u32>,
        fixer: Fixer,
        data: &OrnaData,
    ) -> Result<bool, Error>
    where
        Fixer: FnOnce(&mut AdminEntity, &Vec<u32>) -> Result<(), Error>,
    {
        self.vec(
            field_name,
            admin_field,
            codex_field,
            fixer,
            |id| &data.guide.skills.get_by_id(*id).unwrap().name,
            |id| &data.guide.skills.get_by_id(*id).unwrap().name,
        )
    }

    /// Check a field containing guide item ids.
    pub fn item_id_vec<Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &Vec<u32>,
        codex_field: &Vec<u32>,
        fixer: Fixer,
        data: &OrnaData,
    ) -> Result<bool, Error>
    where
        Fixer: FnOnce(&mut AdminEntity, &Vec<u32>) -> Result<(), Error>,
    {
        self.vec(
            field_name,
            admin_field,
            codex_field,
            fixer,
            |id| &data.guide.items.find_by_id(*id).unwrap().name,
            |id| &data.guide.items.find_by_id(*id).unwrap().name,
        )
    }

    /// Check a field containing guide monster ids.
    pub fn monster_id_vec<Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &Vec<u32>,
        codex_field: &Vec<u32>,
        fixer: Fixer,
        data: &OrnaData,
    ) -> Result<bool, Error>
    where
        Fixer: FnOnce(&mut AdminEntity, &Vec<u32>) -> Result<(), Error>,
    {
        self.vec(
            field_name,
            admin_field,
            codex_field,
            fixer,
            |id| &data.guide.monsters.find_by_id(*id).unwrap().name,
            |id| &data.guide.monsters.find_by_id(*id).unwrap().name,
        )
    }

    /// Check a field containing guide status effects ids.
    pub fn status_effect_id_vec<Fixer>(
        &'a self,
        field_name: &str,
        admin_field: &Vec<u32>,
        codex_field: &Vec<u32>,
        fixer: Fixer,
        data: &OrnaData,
    ) -> Result<bool, Error>
    where
        Fixer: FnOnce(&mut AdminEntity, &Vec<u32>) -> Result<(), Error>,
    {
        self.vec(
            field_name,
            admin_field,
            codex_field,
            fixer,
            |id| {
                &data
                    .guide
                    .static_
                    .status_effects
                    .iter()
                    .find(|effect| effect.id == *id)
                    .unwrap()
                    .name
            },
            |id| &data.guide.items.find_by_id(*id).unwrap().name,
        )
    }
}
