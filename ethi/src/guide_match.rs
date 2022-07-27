use ornaguide_rs::{data::OrnaData, error::Error, guide::OrnaAdminGuide};

pub mod checker;
pub mod items;
pub mod misc;
pub mod monsters;
pub mod pets;
pub mod skills;
pub mod status_effects;

/// Match all entities from codex to the guide.
pub fn all(data: &mut OrnaData, fix: bool, guide: &OrnaAdminGuide) -> Result<(), Error> {
    status_effects::perform(data, fix, guide)?;
    items::perform(data, fix, guide)?;
    skills::perform(data, fix, guide)?;
    monsters::perform(data, fix, guide)?;
    pets::perform(data, fix, guide)?;

    Ok(())
}
