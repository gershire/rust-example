use log4rs;
use log::info;
use lazy_static::lazy_static;
use std::env;
use crate::settings::Settings;

const DEFAULT_CONFIG_PATH: &str = "resources/app-conf.yml";
const DEFAULT_LOG_CONFIG_PATH: &str = "resources/log4rs-conf.yml";

lazy_static! {
    static ref CONFIG_PATH: String = env::var("CONFIG_PATH").unwrap_or(DEFAULT_CONFIG_PATH.to_string());
    static ref LOG_CONFIG_PATH: String = env::var("LOG_CONFIG_PATH").unwrap_or(DEFAULT_LOG_CONFIG_PATH.to_string());
}

pub(crate) fn load_config() -> Settings {
    log4rs::init_file(LOG_CONFIG_PATH.to_string(), Default::default())
        .expect("log4rs config file not found");
    info!("Loading application configuration from {}", &**CONFIG_PATH);
    let conf = Settings::new(CONFIG_PATH.to_string())
        .expect("configuration loading error");
    conf
}