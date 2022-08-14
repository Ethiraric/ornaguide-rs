use ornaguide_rs::{data::OrnaData, error::Error};
use serde::{Deserialize, Serialize};

/// Generic options that can be applied to any route.
#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Options {
    /// The language in which to query.
    pub lang: Option<String>,
    /// Replace IDs with the name of the entity they refer to.
    pub deref: bool,
    /// Key by which to be sorted.
    pub sort_by: Option<String>,
    /// Whether sort should be descending (default is ascending).
    pub sort_descending: bool,
}

/// Trait to be implemented by entity holding IDs which can be dereferenced.
/// For instance, monsters have abilities that the API will by default return as IDs. Running the
/// monster through this trait will change the IDs to the abilities' names.
pub trait IdDerefable {
    /// Turn `self` to a serde value and replace IDs to names.
    fn id_deref(&self, data: &OrnaData) -> Result<serde_json::Value, Error>;
}
