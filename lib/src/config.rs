use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufReader},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonstkaConfigError {
    #[error("cannot load config file")]
    Load(#[from] io::Error),
    #[error("cannot parse config file")]
    Parse(#[from] serde_yaml::Error),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct MonstkaConfig {
    /// The api version of the agent config file
    pub topic: String,
    pub uri: String,
    pub group_id: String,
    pub random_word_generator_api_url: String,
    pub partitions: Vec<i32>,
}


impl MonstkaConfig {
    /// Load a MonstkaConfig from a file.
    ///
    /// Arguments:
    ///
    /// * `path`: The path to the config file.
    ///
    /// Returns:
    ///
    /// A Result<MonstkaConfig>
    pub fn load(path: &str) -> Result<Self> {
        let file = File::open(path).map_err(MonstkaConfigError::Load)?;
        let reader = BufReader::new(file);
        let config: MonstkaConfig =
            serde_yaml::from_reader(reader).map_err(MonstkaConfigError::Parse)?;

        Ok(config)
    }
}
