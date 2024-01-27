use crate::{Error, Result};
use std::env;


#[allow(non_snake_case)]
pub struct Config {
    pub WEB_FOLDER: String,
}


impl Config{
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            WEB_FOLDER: env::var("WEB_FOLDER")?,
        })
    }
}
