use crate::{
    error::Error,
    items::{admin::AdminItem, raw::RawItem},
    monsters::{admin::AdminMonster, raw::RawMonster},
};

mod cache;
pub(crate) mod html_parser;
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

    /// If not already done, query the API of the guide for the list of monsters and store it in a
    /// cache. If the cache is already fetched, return it. The latter case cannot return an `Err`.
    fn fetch_monsters(&mut self) -> Result<&[RawMonster], Error>;

    /// Return the cache, if already fetched. This method will always return `None` before a call
    /// to `fetch_monsters`.
    fn get_monsters(&self) -> Option<&[RawMonster]>;
}

/// A read-write access to the administrator panel of the guide.
pub trait AdminGuide {
    /// Retrieve the item with the given id from the guide.
    fn admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error>;
    /// Save the given item to the guide.
    fn admin_save_item(&self, item: AdminItem) -> Result<(), Error>;

    /// Retrieve the monster with the given id from the guide.
    fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<AdminMonster, Error>;
    /// Save the given monster to the guide.
    fn admin_save_monster(&self, monster: AdminMonster) -> Result<(), Error>;
}

pub use cache::CachedGuide;
pub use ornaguide::{OrnaAdminGuide, OrnaGuide};
