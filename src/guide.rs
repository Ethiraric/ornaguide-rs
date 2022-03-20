use crate::{error::Error, items::raw::RawItem};

mod http;
mod ornaguide;

/// A source of information from the game. On the site, this represents the public API.
/// Note that the info can be fetched locally from a cache.
pub trait Guide {
    /// If not already done, query the API of the guide for the list of items and store it in a
    /// cache. If the cache is already fetched, return it. The latter case cannot return an `Err`.
    fn fetch_items(&mut self) -> Result<&[RawItem], Error>;

    /// Return the cache, if already fetched. This method will always return `None` before a call
    /// to `fetch_items`.
    fn get_items(&self) -> Option<&[RawItem]>;
}

pub use ornaguide::OrnaGuide;
