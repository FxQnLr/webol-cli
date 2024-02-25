use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub apikey: String,
    pub server: String,
}

impl Config {
    pub fn load() -> Result<Config, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("~/.config/webol-cli.toml"))
            .add_source(config::File::with_name("webol-cli.toml"))
            .add_source(config::Environment::with_prefix("WEBOL_CLI_").separator("_"))
            .build()?;

        builder.try_deserialize()
    }
}
