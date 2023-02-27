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
    skills::perform(data, fix, guide)?;
    items::perform(data, fix, guide)?;
    monsters::perform(data, fix, guide)?;
    pets::perform(data, fix, guide)?;

    Ok(())
}

/// Execute a CLI subcommand on matching.
pub fn cli(args: &[&str], guide: &OrnaAdminGuide, mut data: OrnaData) -> Result<(), Error> {
    match args {
        ["status_effects"] => status_effects::perform(&mut data, false, guide),
        ["status_effects", "--fix"] => status_effects::perform(&mut data, true, guide),
        ["skills"] => skills::perform(&mut data, false, guide),
        ["skills", "--fix"] => skills::perform(&mut data, true, guide),
        ["items"] => items::perform(&mut data, false, guide),
        ["items", "--fix"] => items::perform(&mut data, true, guide),
        ["monsters"] => monsters::perform(&mut data, false, guide),
        ["monsters", "--fix"] => monsters::perform(&mut data, true, guide),
        ["pets"] => pets::perform(&mut data, false, guide),
        ["pets", "--fix"] => pets::perform(&mut data, true, guide),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `match` arguments: {:?}",
            &args
        ))),
    }
}
