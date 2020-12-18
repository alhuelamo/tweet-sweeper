use std::env;
use std::fs;
use std::io;

use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct ApiSecrets {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ApiUser {
    pub token: String,
    pub token_secret: String,
    pub display_name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ApiConfig {
    pub app: ApiSecrets,
    pub user: ApiUser,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub delete_days: i64,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub api: ApiConfig,
    pub app: AppConfig,
}

pub fn load_default_configuration() -> io::Result<Config> {
    let path = get_conf_path()?;
    get_configuration(path)
}

pub fn get_conf_path() -> io::Result<std::path::PathBuf> {
    let mut path = env::current_dir()?;
    path.push("configuration.toml");
    Ok(path)
}

pub fn get_configuration(path: std::path::PathBuf) -> io::Result<Config> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_configurartion_from_str() {
        let config = "\
[api.app]
key = \"my_key\"
secret = \"my_secret\"

[api.user]
token = \"my_token\"
token_secret = \"my_token_secret\"
display_name = \"alhuelamo\"

[app]
delete_days = 15
";
        let actual: Config = toml::from_str(&config).unwrap();
        let expected = Config {
            api: ApiConfig {
                app: ApiSecrets {
                    key: String::from("my_key"),
                    secret: String::from("my_secret"),
                },
                user: ApiUser {
                    token: String::from("my_token"),
                    token_secret: String::from("my_token_secret"),
                    display_name: String::from("alhuelamo"),
                },
            },
            app: AppConfig {
                delete_days: 15
            }
        };
        assert_eq!(actual, expected);
    }

}
