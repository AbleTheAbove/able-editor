use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Configuration {
    log: Option<bool>,
}
const DEFAULT_CONFIG: Configuration = Configuration { log: Some(true) };
pub fn load_config() -> Configuration {
    let filename = "assets/config.toml";

    let contents = fs::read_to_string(filename);
    match contents {
        Ok(o) => {
            println!("{}", o);
            let decoded: Configuration = toml::from_str(&o).unwrap_or(DEFAULT_CONFIG);
            return decoded;
        }
        Err(e) => {
            println!("{}", e);
            return DEFAULT_CONFIG;
        }
    }
}
