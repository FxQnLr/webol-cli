use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: String,
    pub auth: Auth,
}

#[derive(Deserialize)]
pub struct Auth {
    pub method: Method,
    pub secret: String,
}

#[derive(PartialEq, Eq, Deserialize)]
pub enum Method {
    None,
    Key,
}

impl Config {
    pub fn load() -> Result<Config, config::ConfigError> {
        let config_dir = dirs::config_dir();

        let builder = config::Config::builder();

        let builder = builder
            .set_default("auth.method", "none")?
            .set_default("auth.secret", "")?;

        let builder = if let Some(conf) = config_dir {
            let dir = conf.to_string_lossy();
            builder.add_source(
                config::File::with_name(format!("{dir}/webol-cli").as_str()).required(false),
            )
        } else {
            println!("!No config dir found");
            builder
        };

        let build = builder
            .add_source(config::File::with_name("webol-cli").required(false))
            .add_source(config::Environment::with_prefix("WEBOL_CLI").separator("_"))
            .build()?;

        build.try_deserialize()
    }
}
