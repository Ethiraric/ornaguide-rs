use std::{collections::HashMap, sync::RwLock};

use ornaguide_rs::{codex::translation::LocaleDB, data::OrnaData, error::Error as OError};

use lazy_static::{__Deref, lazy_static};

use crate::error::{Error, ToErrorable};

lazy_static! {
    pub static ref DATA: Result<RwLock<OrnaData>, OError> =
        OrnaData::load_from("output").map(RwLock::new);
}

/// Run a callable with a reference to the `OrnaData`.
/// The data given is localized to the given locale. If a locale is specified but not found, an
/// error is returned.
pub fn with_data<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&OrnaData) -> Result<T, Error>,
{
    let lock = DATA.as_ref().to_internal_server_error()?;
    let lock2 = lock.read();
    let data = lock2
        .as_ref()
        .map_err(|err| OError::Misc(format!("{}", err)))
        .to_internal_server_error()?
        .deref();

    f(data)
}

lazy_static! {
    pub static ref LOCALE_DATA: Result<RwLock<HashMap<String, OrnaData>>, Error> =
        generate_locale_data().map(RwLock::new);
}

/// Run a callable with a reference to an `OrnaData` instance, translated to the given locale, if
/// any. The default locale is `en`.
pub fn with_locale_data<F, T>(f: F, lang: &Option<String>) -> Result<T, Error>
where
    F: FnOnce(&OrnaData) -> Result<T, Error>,
{
    // If `lang` is `None` or `en`, get the default data. Avoids a `HashMap` lookup for the most
    // common case.
    match lang.as_ref().map(String::as_str) {
        None | Some("en") => with_data(f),
        Some(lang) => {
            let lock = LOCALE_DATA.as_ref().map_err(Error::clone)?;
            let lock2 = lock.read();
            let locale_data = lock2
                .as_ref()
                .map_err(|err| OError::Misc(format!("{}", err)))
                .to_internal_server_error()?
                .deref();

            if let Some(data) = locale_data.get(lang) {
                f(data)
            } else {
                Err(OError::Misc(format!("Failed to find locale {}", lang)))
                    .to_internal_server_error()
            }
        }
    }
}

fn generate_locale_data() -> Result<HashMap<String, OrnaData>, Error> {
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
            // Translate items names and descriptions.
            for item in localized.guide.items.items.iter_mut() {
                if let Some(localized_item) = data
                    .codex
                    .items
                    .find_by_uri(&item.codex_uri)
                    .and_then(|codex_item| db.item(&codex_item.slug))
                {
                    item.name = localized_item.name.clone();
                    item.description = localized_item.description.clone();
                }
            }

            // Translate monsters names.
            for monster in localized.guide.monsters.monsters.iter_mut() {
                if let Some(generic_monster) =
                    data.codex.find_generic_monster_from_uri(&monster.codex_uri)
                {
                    let name = match generic_monster {
                        ornaguide_rs::data::CodexGenericMonster::Monster(x) => {
                            db.monster_name(&x.slug)
                        }
                        ornaguide_rs::data::CodexGenericMonster::Boss(x) => db.boss_name(&x.slug),
                        ornaguide_rs::data::CodexGenericMonster::Raid(x) => db.raid_name(&x.slug),
                    };
                    if let Some(name) = name {
                        if monster.name == "Arisen Quetzalcoatl" {
                            println!("{} -> {}", monster.name, name);
                        }
                        monster.name = name.to_string()
                    }
                }
            }

            // Translate skills names and descriptions.
            for skill in localized.guide.skills.skills.iter_mut() {
                if let Some(localized_skill) = data
                    .codex
                    .skills
                    .find_by_uri(&skill.codex_uri)
                    .and_then(|codex_skill| db.skill(&codex_skill.slug))
                {
                    skill.name = localized_skill.name.clone();
                    skill.description = localized_skill.description.clone();
                }
            }

            // Translate pets names and descriptions.
            for pet in localized.guide.pets.pets.iter_mut() {
                if let Some(localized_pet) = data
                    .codex
                    .followers
                    .find_by_uri(&pet.codex_uri)
                    .and_then(|codex_pet| db.follower(&codex_pet.slug))
                {
                    pet.name = localized_pet.name.clone();
                    pet.description = localized_pet.description.clone();
                }
            }

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
