


use config::{Config, Environment, File};
use std::env;
pub fn get_config() -> Config {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let env_settings = Environment::default();

    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name("config/config"))
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        .add_source(File::with_name(&format!("config/config-{}", run_mode)).required(false))
        // Add in the constants file
        .add_source(File::with_name("config/constants.json"))
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        .add_source(env_settings.prefix("app").separator("_"))
        // You may also programmatically change settings
        .build()
        .unwrap();

    // Now that we're done, let's access our configuration

    // You can deserialize (and thus freeze) the entire configuration as
    s
}