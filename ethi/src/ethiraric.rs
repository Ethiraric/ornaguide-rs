use ornaguide_rs::{data::OrnaData, error::Error, guide::OrnaAdminGuide};

pub mod summons;

/// Execute a CLI subcommand for ethiraric.
pub fn cli(args: &[&str], _: &OrnaAdminGuide, _: OrnaData) -> Result<(), Error> {
    match args {
        ["summons", file] => summons::summons(file),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `ethiraric` arguments: {:?}",
            &args
        ))),
    }
}
