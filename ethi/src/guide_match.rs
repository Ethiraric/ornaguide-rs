use ornaguide_rs::{error::Error, guide::OrnaAdminGuide};

use crate::output::OrnaData;

pub mod checker;
pub mod items;
pub mod misc;
pub mod monsters;
pub mod pets;
pub mod skills;

/// Match all entities from codex to the guide.
pub fn all(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    items::perform(data, fix, guide)?;
    monsters::perform(data, fix, guide)?;
    skills::perform(data, fix, guide)?;
    pets::perform(data, fix, guide)?;

    Ok(())
}
