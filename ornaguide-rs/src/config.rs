use std::sync::RwLock;

use dotenv::dotenv;

use crate::error::{Error,ErrorKind};

use lazy_static::lazy_static;

/// General configuration options.
/// Options' values are taken from (in order, first takes precedence):
///   - Environment
///   - `.env` file
///   - Default value (if applicable).
#[derive(Debug)]
pub struct Config {
    /// Whether to output URLs that are queried.
    /// Default: false
    /// Environment variable: `ORNAGUIDERS_DEBUG_URLS`
    pub debug_urls: bool,
}

lazy_static! {
    pub static ref CONFIG: Result<RwLock<Config>, Error> = load().map(RwLock::new);
}

/// Load the config from the environment.
fn load() -> Result<Config, Error> {
    let _ = dotenv().map_err(|err| ErrorKind::Misc(format!("Failed to load .env: {}", err)))?;
    let config = Config {
        debug_urls: dotenv::var("ORNAGUIDERS_DEBUG_URLS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()?,
    };

    Ok(config)
}

/// Run a callable with a reference to the `Config` instance.
pub fn with_config<F, T>(f: F) -> Result<T, Error>
where
    F: FnOnce(&Config) -> Result<T, Error>,
{
    let config = CONFIG
        .as_ref()
        .map_err(|err| ErrorKind::Misc(format!("{}", err)))?;
    let config = config
        .read()
        .map_err(|err| ErrorKind::Misc(format!("{}", err)))?;
    f(&config)
}

/// Return the `debug_urls` config value.
pub fn debug_urls() -> Result<bool, Error> {
    with_config(|config| Ok(config.debug_urls))
}
