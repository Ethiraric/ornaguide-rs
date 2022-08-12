use std::sync::RwLock;

use ornaguide_rs::data::OrnaData;

use lazy_static::{__Deref, lazy_static};

use crate::error::{Error, ToErrorable};

lazy_static! {
    pub static ref DATA: Result<RwLock<OrnaData>, ornaguide_rs::error::Error> =
        OrnaData::load_from("output").map(RwLock::new);
}

/// Run a callable with a reference to the `OrnaData`.
pub fn with_data<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&OrnaData) -> Result<T, Error>,
{
    let lock = DATA.as_ref().to_bad_request()?;
    let lock2 = lock.read();
    let data = lock2
        .as_ref()
        .map_err(|err| ornaguide_rs::error::Error::Misc(format!("{}", err)))
        .to_bad_request()?
        .deref();

    f(data)
}
