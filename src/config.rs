use config::Config;
use once_cell::sync::Lazy;

pub static SETTINGS: Lazy<Config> = Lazy::new(setup);

fn setup() -> Config {
    #[cfg(not(debug_assertions))]
    let builder = Config::builder().add_source(config::File::with_name(
        format!("{}/webol-cli.toml", dirs::config_dir().unwrap().to_string_lossy()).as_str(),
    ));

    #[cfg(debug_assertions)]
    let builder = Config::builder().add_source(config::File::with_name("webol-cli.toml"));

    builder
        .add_source(config::Environment::with_prefix("WEBOL_CLI_").separator("_"))
        .build()
        .unwrap()
}
