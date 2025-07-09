use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] serde_json::Error),
}

pub fn init(config: Config) {
    assert!(
        config.default_radix >= 2,
        "default_radix can not be smaller than 2"
    );
    assert!(
        config.default_radix <= 36,
        "default_radix can not be greater than 36"
    );
    CONFIG.set(config).expect("Config already initialized");
}

pub fn get() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}

pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<Config, Error> {
    let file = std::fs::File::open(path)?;
    let config = serde_json::from_reader(file)?;
    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub trace_filter: Option<String>,
    pub port: u16,
    pub default_radix: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            trace_filter: None,
            port: 23472,
            default_radix: 10,
        }
    }
}
