use ornaguide_rs::{data::OrnaData, error::Error, guide::OrnaAdminGuide};

use crate::cli;

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
    skills::perform(data, fix, guide)?;
    items::perform(data, fix, guide)?;
    monsters::perform(data, fix, guide)?;
    pets::perform(data, fix, guide)?;

    Ok(())
}

/// Execute a CLI subcommand on matching.
pub fn cli(
    command: cli::match_::Command,
    guide: &OrnaAdminGuide,
    mut data: OrnaData,
) -> Result<(), Error> {
    let fix = command.fix;
    match command.c {
        Some(cli::match_::Subcommand::Items) => items::perform(&mut data, fix, guide),
        Some(cli::match_::Subcommand::Monsters) => monsters::perform(&mut data, fix, guide),
        Some(cli::match_::Subcommand::Pets) => monsters::perform(&mut data, fix, guide),
        Some(cli::match_::Subcommand::Skills) => skills::perform(&mut data, fix, guide),
        Some(cli::match_::Subcommand::StatusEffects) => {
            status_effects::perform(&mut data, fix, guide)
        }
        None => all(&mut data, fix, guide),
    }
}
