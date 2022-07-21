use std::sync::RwLock;

use ornaguide_rs::{data::OrnaData, error::Error};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATA: Result<RwLock<OrnaData>, Error> =
        OrnaData::load_from("output").map(RwLock::new);
}
