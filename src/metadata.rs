use std::collections::HashMap;

use crate::types::CrateName;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub name: CrateName,
    pub vers: Version,
    pub deps: Vec<Dependency>,
    pub cksum: String,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
    pub links: Option<String>,
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.vers == other.vers
    }
}

impl Eq for Metadata {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub name: String,
    pub req: VersionReq,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    pub target: Option<String>,
    pub kind: Kind,
    pub registry: Option<String>,
    pub package: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Normal,
    Build,
    Dev,
}
