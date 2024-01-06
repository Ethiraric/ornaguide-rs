use std::collections::HashMap;

use ornaguide_rs::{
    data::OrnaData,
    error::{Error as OError, Kind},
};

use lazy_static::lazy_static;

use crate::error::{Error, ToErrorable};

mod translations;

lazy_static! {
    pub static ref DATA: Result<OrnaData, OError> = OrnaData::load_from("data/current_entries");
}

/// Run a callable with a reference to the `OrnaData`.
/// The data given is localized to the given locale. If a locale is specified but not found, an
/// error is returned.
#[allow(clippy::module_name_repetitions)]
pub fn with_data<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&'static OrnaData) -> Result<T, Error>,
{
    let data = DATA.as_ref().to_internal_server_error()?;
    f(data)
}

lazy_static! {
    pub static ref LOCALE_DATA: Result<HashMap<String, OrnaData>, Error> =
        translations::generate_locale_data();
}

/// Run a callable with a reference to an `OrnaData` instance, translated to the given locale, if
/// any. The default locale is `en`.
#[allow(clippy::module_name_repetitions)]
pub fn with_locale_data<F, T>(f: F, lang: &Option<String>) -> Result<T, Error>
where
    F: FnOnce(&'static OrnaData) -> Result<T, Error>,
{
    // If `lang` is `None` or `en`, get the default data. Avoids a `HashMap` lookup for the most
    // common case.
    match lang.as_ref().map(String::as_str) {
        None | Some("en") => with_data(f),
        Some(lang) => {
            let locale_data = LOCALE_DATA.as_ref().map_err(Error::clone)?;

            if let Some(data) = locale_data.get(lang) {
                f(data)
            } else {
                Err(Kind::Misc(format!("Failed to find locale {lang}")).into_err())
                    .to_internal_server_error()
            }
        }
    }
}
