use std::{collections::HashMap, sync::RwLock};

use ornaguide_rs::{data::OrnaData, error::Error as OError};

use lazy_static::{__Deref, lazy_static};

use crate::error::{Error, ToErrorable};

mod translations;

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
        translations::generate_locale_data().map(RwLock::new);
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
