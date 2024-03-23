use std::{env, path::Path};

use config::{Config as ConfigParser, ConfigError, File, FileFormat};
use serde::Deserialize;

use crate::utils::finder;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct App {
    pub host: String,
    pub port: u16,
    pub app_name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Topics {
    pub actions: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kafka {
    pub broker: String,
    pub group_id: String,
    pub topics: Topics,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub app: App,
    pub kafka: Kafka,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let cf = ConfigParser::builder()
            .add_source(File::from_str(
                include_str!("./default.yml"),
                FileFormat::Yaml,
            ))
            .add_source(
                File::from(
                    finder::find(&env::current_dir().unwrap(), Path::new("env.yml")).unwrap(),
                )
                .required(false),
            )
            .build()?;

        let mut cf = cf.try_deserialize::<Self>()?;
        Ok(cf)
    }
}
