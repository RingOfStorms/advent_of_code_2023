use std::{env::var, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub aoc_session: String,
}

impl Config {}

fn get_var(var_name: &str) -> String {
    var(var_name).unwrap_or("".to_owned())
}

pub fn get_config() -> &'static Config {
    let config = CONFIG.get_or_init(|| Config {
        aoc_session: get_var("AOC_SESSION"),
    });
    config
}
