use ornaguide_rs::{data::OrnaData, error::Error, guide::OrnaAdminGuide};

pub mod fetch;

/// Execute a CLI subcommand on the codex.
pub fn cli(args: &[&str], guide: &OrnaAdminGuide, data: OrnaData) -> Result<(), Error> {
    match args {
        ["bugs"] => crate::codex_bugs::check(&data, guide),
        ["missing"] => fetch::missing(guide, &data).map(|_| ()),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `codex` arguments: {:?}",
            &args
        ))),
    }
}
