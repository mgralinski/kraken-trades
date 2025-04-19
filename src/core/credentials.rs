use serde::Deserialize;
use std::{io::Error, path::Path};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
}

impl Credentials {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let data = std::fs::read_to_string(path)?;
        let creds = serde_json::from_str::<Credentials>(&data)?;
        Ok(creds)
    }
}
