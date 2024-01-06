use std::sync::RwLock;

use dotenv::dotenv;

use ornaguide_rs::error::{Error, Kind};

use lazy_static::lazy_static;

/// General configuration options.
/// Options' values are taken from (in order, first takes precedence):
///   - Environment
///   - `.env` file
///   - Default value (if applicable).
#[derive(Debug)]
pub struct Config {
    /// Host string to `orna.guide`. Must not have a trailing slash.
    /// Can be used to have a localhost mirror.
    /// Default: `https://orna.guide`
    /// Environment variable: `ORNAGUIDE_HOST`
    pub ornaguide_host: String,
    /// Connection cookie to the guide.
    /// Default: None, errors if missing.
    /// Environment variable: `ORNAGUIDE_COOKIE`
    pub ornaguide_cookie: String,
    /// Number of seconds to wait between each call to the guide.
    /// Default: 0
    /// Environment variable: `ORNAGUIDE_SLEEP`
    pub ornaguide_sleep: u32,
    /// Host string to `playorna.com`. Must not have a trailing slash.
    /// Can be used to have a localhost mirror.
    /// Default: `https://playorna.com`
    /// Environment variable: `PLAYORNA_HOST`
    pub playorna_host: String,
    /// Number of seconds to wait between each call to playorna.
    /// Default: 0
    /// Environment variable: `PLAYORNA_SLEEP`
    pub playorna_sleep: u32,
}

lazy_static! {
    pub static ref CONFIG: Result<RwLock<Config>, Error> = load().map(RwLock::new);
}

fn sanitize_config(config: &mut Config) {
    if config.playorna_host.ends_with('/') {
        config.playorna_host = config.playorna_host.trim_end_matches('/').to_string();
    }
    if config.ornaguide_host.ends_with('/') {
        config.ornaguide_host = config.ornaguide_host.trim_end_matches('/').to_string();
    }
}

/// Load the config from the environment.
fn load() -> Result<Config, Error> {
    let _ = dotenv().map_err(|err| Kind::Misc(format!("Failed to load .env: {err}")))?;
    let mut config = Config {
        ornaguide_host: dotenv::var("ORNAGUIDE_HOST")
            .unwrap_or_else(|_| "https://orna.guide".to_string()),
        ornaguide_cookie: dotenv::var("ORNAGUIDE_COOKIE").map_err(|err| {
            Kind::Misc(format!(
                "Failed to get ORNAGUIDE_COOKIE env variable: {err}"
            ))
        })?,
        ornaguide_sleep: dotenv::var("ORNAGUIDE_SLEEP")
            .unwrap_or_else(|_| "0".to_string())
            .parse()?,
        playorna_host: dotenv::var("PLAYORNA_HOST")
            .unwrap_or_else(|_| "https://playorna.com".to_string()),
        playorna_sleep: dotenv::var("PLAYORNA_SLEEP")
            .unwrap_or_else(|_| "0".to_string())
            .parse()?,
    };
    sanitize_config(&mut config);

    Ok(config)
}

/// Run a callable with a reference to the `Config` instance.
#[allow(clippy::module_name_repetitions)]
pub fn with_config<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&Config) -> Result<T, Error>,
{
    let config = CONFIG
        .as_ref()
        .map_err(|err| Kind::Misc(format!("{err}")))?;
    let config = config.read().map_err(|err| Kind::Misc(format!("{err}")))?;
    f(&config)
}

/// Return the `ornaguide_sleep` config value.
pub fn ornaguide_sleep() -> Result<u32, Error> {
    with_config(|config| Ok(config.ornaguide_sleep))
}

/// Return the `playorna_sleep` config value.
pub fn playorna_sleep() -> Result<u32, Error> {
    with_config(|config| Ok(config.playorna_sleep))
}
