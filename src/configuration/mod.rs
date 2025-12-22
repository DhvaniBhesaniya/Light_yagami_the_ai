use config::Config;
mod setting;
use lazy_static::lazy_static;
// use crate::error::{ConfigError, Result};
use serde::de::Deserialize;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(setting::get_config());
}

// pub fn get(property:String) -> String{
//     CONFIG.lock().unwrap().get(&property).unwrap()
// }

pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
    let res = CONFIG.read().unwrap().get(key);
    match res {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to get config key {}: {}", key, e);
            panic!("Configuration key not found");
        }
    }
}

pub fn get_res<'de, T: Deserialize<'de>>(key: &str) -> Result<T, config::ConfigError> {
    CONFIG.read().unwrap().get(key)
}
