use actix_web::web;
use std::collections::HashMap;

use config::{Config, Value};

const CONFIG_FILENAME: &str = "Config.toml";
const CONFIG_APPLICATION_TABLE: &str = "application";

macro_rules! config_failed_format {
    () => {
        format!(
            "Unable to load config file. Please create the \"{:?}\" file.",
            CONFIG_FILENAME
        )
        .as_str()
    };
}

pub fn load_application_config() -> HashMap<String, Value> {
    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILENAME))
        .build()
        .expect(config_failed_format!());
    let application_config = settings.get_table(CONFIG_APPLICATION_TABLE).unwrap();
    application_config
}

pub fn configure(cfg: &mut web::ServiceConfig) {}
