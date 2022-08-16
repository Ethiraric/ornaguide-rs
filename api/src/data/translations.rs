use std::collections::HashMap;

use ornaguide_rs::{
    codex::translation::{GenericMonsterTranslation, LocaleDB, TranslationFor},
    data::{CodexGenericMonster, OrnaData},
};

use crate::{
    data::with_data,
    error::{Error, ToErrorable},
};

/// Translate a set of entities.
/// The translation is gotten using `translation_getter`.
fn translate_with<E, F, T>(entities: &mut [E], translation_getter: F)
where
    F: Fn(&E) -> Option<T>,
    T: TranslationFor<E>,
{
    for entity in entities.iter_mut() {
        if let Some(translation) = translation_getter(entity) {
            translation.apply_to(entity);
        }
    }
}

/// Generate multiple versions of `OrnaData`s, one for each locale we know of.
pub(crate) fn generate_locale_data() -> Result<HashMap<String, OrnaData>, Error> {
    with_data(|data| {
        let mut ret = HashMap::new();
        for (lang, db) in LocaleDB::load_from("output/i18n")
            .and_then(|mut db| {
                db.merge_with(LocaleDB::load_from("output/i18n/manual")?);
                Ok(db)
            })
            .to_internal_server_error()?
            .locales
            .into_iter()
        {
            let mut localized: OrnaData = data.clone();

            // Translate items.
            translate_with(&mut localized.guide.items.items, |item| {
                data.codex
                    .items
                    .find_by_uri(&item.codex_uri)
                    .and_then(|codex_item| db.item(&codex_item.slug))
                    .cloned()
            });

            // Translate monsters.
            translate_with(&mut localized.guide.monsters.monsters, |monster| {
                data.codex
                    .find_generic_monster_from_uri(&monster.codex_uri)
                    .and_then(|codex_monster| match codex_monster {
                        CodexGenericMonster::Monster(x) => db
                            .monster(&x.slug)
                            .cloned()
                            .map(GenericMonsterTranslation::Monster),
                        CodexGenericMonster::Boss(x) => db
                            .boss(&x.slug)
                            .cloned()
                            .map(GenericMonsterTranslation::Boss),
                        CodexGenericMonster::Raid(x) => db
                            .raid(&x.slug)
                            .cloned()
                            .map(GenericMonsterTranslation::Raid),
                    })
            });

            // Translate skills.
            translate_with(&mut localized.guide.skills.skills, |skill| {
                data.codex
                    .skills
                    .find_by_uri(&skill.codex_uri)
                    .and_then(|codex_skill| db.skill(&codex_skill.slug))
                    .cloned()
            });

            // Translate pets.
            translate_with(&mut localized.guide.pets.pets, |pet| {
                data.codex
                    .followers
                    .find_by_uri(&pet.codex_uri)
                    .and_then(|codex_follower| db.follower(&codex_follower.slug))
                    .cloned()
            });

            // Translate status effects.
            for status in localized.guide.static_.status_effects.iter_mut() {
                if let Some(localized_effect) = db.status(&status.name) {
                    status.name = localized_effect.to_string();
                }
            }
            // Translate spawns.
            for spawn in localized.guide.static_.spawns.iter_mut() {
                if let Some(localized_spawn) = db.spawn(&spawn.name) {
                    spawn.name = localized_spawn.to_string();
                }
            }
            // Translate monster families.
            for family in localized.guide.static_.monster_families.iter_mut() {
                if let Some(localized_family) = db.spawn(&family.name) {
                    family.name = localized_family.to_string();
                }
            }

            ret.insert(lang.clone(), localized);
        }

        Ok(ret)
    })
}
