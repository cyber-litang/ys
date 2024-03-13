use std::fs::read_to_string;

use once_cell::sync::Lazy;
use serde::Deserialize;

static CONFIG: Lazy<Config> = Lazy::new(|| load_config());

#[derive(Deserialize, Debug)]
pub struct Mail {
    pub domain: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Ys {
    pub always: bool,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mail: Mail,
    pub ys: Ys,
}

fn load_config() -> Config {
    let config = read_to_string("config.toml").expect("config.toml not found");
    toml::from_str(&config).expect("config.toml is not a valid toml file")
}

pub fn get_config() -> &'static Config {
    &CONFIG
}
