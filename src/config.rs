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
pub struct ApiConf {
    pub app: ApiSecrets,
    pub user: ApiUser,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AppConf {
    pub delete_days: i64,
    pub ignore_liked_by_me: bool,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub api: ApiConf,
    pub app: AppConf,
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
ignore_liked_by_me = true
";
        let actual: Config = toml::from_str(&config).unwrap();
        let expected = Config {
            api: ApiConf {
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
            app: AppConf {
                delete_days: 15,
                ignore_liked_by_me: true,
            }
        };
        assert_eq!(actual, expected);
    }

}
