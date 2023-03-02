use ornaguide_rs::{
    codex::translation::LocaleDB, data::OrnaData, error::Error, guide::OrnaAdminGuide,
};

use crate::cli;

/// Execute a CLI subcommand on translations.
pub fn cli(
    command: cli::translation::Command,
    guide: &OrnaAdminGuide,
    data: OrnaData,
    mut locales: LocaleDB,
) -> Result<(), Error> {
    match command {
        cli::translation::Command::Missing => {
            let missing = crate::codex::fetch::missing_translations(guide, &data, &locales)?;
            locales.merge_with(missing);
            locales.save_to("output/i18n")
        }
        cli::translation::Command::Fetch(locale) => {
            crate::codex::fetch::translations(guide, &data, &locale.locale)?
                .save_to(&format!("output/i18n/{}.json", &locale.locale))
        }
    }
}
