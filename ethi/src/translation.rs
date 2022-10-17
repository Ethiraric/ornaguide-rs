use ornaguide_rs::{
    codex::translation::LocaleDB, data::OrnaData, error::Error, guide::OrnaAdminGuide,
};

/// Execute a CLI subcommand on translations.
pub fn cli(
    args: &[&str],
    guide: &OrnaAdminGuide,
    data: OrnaData,
    mut locales: LocaleDB,
) -> Result<(), Error> {
    match args {
        ["missing"] => {
            let missing = crate::codex::fetch::missing_translations(guide, &data, &locales)?;
            locales.merge_with(missing);
            locales.save_to("output/i18n")
        }
        [locale] => crate::codex::fetch::translations(&guide, &data, locale)?
            .save_to(&format!("output/i18n/{}.json", locale)),
        _ => Err(Error::Misc(format!(
            "Invalid CLI `translation` arguments: {:?}",
            &args
        ))),
    }
}
