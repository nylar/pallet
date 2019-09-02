use std::collections::HashSet;
use std::io::Read;

use crate::error::Error;

use serde::Deserialize;

const CRATES_IO: &str = "https://github.com/rust-lang/crates.io-index";

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    dl: String,
    api: Option<String>,
    #[serde(rename = "allowed-registries")]
    allowed_registries: Option<Vec<String>>,
    #[serde(skip)]
    registries: HashSet<String>,
}

impl Config {
    pub fn open<R: Read>(r: R, registry: &str) -> Result<Self, Error> {
        let mut config: Config = serde_json::from_reader(r)?;

        // Always allow crates.io and the current registry
        let mut registries = [CRATES_IO.to_owned(), registry.to_owned()]
            .iter()
            .cloned()
            .collect::<HashSet<String>>();

        // Add any additional registries
        if let Some(ref allowed_registries) = config.allowed_registries {
            for registry in allowed_registries {
                registries.insert(registry.to_owned());
            }
        }

        config.registries = registries;

        Ok(config)
    }

    pub fn registry_allowed(&self, registry: &str) -> bool {
        self.registries.contains(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REGISTRY: &str = "https://github.com/nylar/pallet-index";
    const ALT_REGISTRY: &str = "https://github.com/nylar/pallet-mirror";

    fn test_default_registries() {
        let json = r#"{
    "dl": "localhost:8080/api/v1/crates/{crate}/{version}/download",
    "api": "localhost:8080/"
}"#;

        let config = Config::open(json.as_bytes(), REGISTRY).unwrap();

        assert!(config.registry_allowed(CRATES_IO));
        assert!(config.registry_allowed(REGISTRY));
        assert!(!config.registry_allowed(ALT_REGISTRY));
    }

    fn test_additional_registries() {
        let json = r#"{
    "dl": "localhost:8080/api/v1/crates/{crate}/{version}/download",
    "api": "localhost:8080/",
    "allowed-registries": ["https://github.com/rust-lang/crates.io-index", "https://github.com/nylar/pallet-mirror"]
}"#;

        let config = Config::open(json.as_bytes(), REGISTRY).unwrap();

        assert!(config.registry_allowed(CRATES_IO));
        assert!(config.registry_allowed(REGISTRY));
        assert!(config.registry_allowed(ALT_REGISTRY));
    }
}
