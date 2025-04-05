use ornaguide_rs::{data::OrnaData, error::Error, guide::OrnaAdminGuide};

use crate::cli;

pub mod fetch;

/// Execute a CLI subcommand on the codex.
pub fn cli(
    command: &cli::codex::Command,
    guide: &OrnaAdminGuide,
    data: &OrnaData,
) -> Result<(), Error> {
    match command {
        cli::codex::Command::Bugs => {
            crate::codex_bugs::check(data, guide);
            Ok(())
        }
        cli::codex::Command::Missing => fetch::missing(guide, data),
    }
}
