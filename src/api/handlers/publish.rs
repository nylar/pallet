use std::collections::HashMap;
use std::sync::Arc;

use crate::metadata::{Dependency, Kind, Metadata};
use crate::models::{
    krate::{Krate, NewKrate},
    krateowner::NewKrateOwner,
    owner::Owner,
    version::NewVersion,
};
use crate::Application;

use bytes::Buf;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use warp::reject::custom;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateUpload {
    name: String,
    vers: semver::Version,
    deps: Vec<CrateDependency>,
    features: HashMap<String, Vec<String>>,
    authors: Vec<String>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    readme: Option<String>,
    readme_file: Option<String>,
    keywords: Vec<String>,
    categories: Vec<String>,
    license: Option<String>,
    license_file: Option<String>,
    repository: Option<String>,
    links: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateDependency {
    name: String,
    version_req: semver::VersionReq,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Kind,
    registry: Option<String>,
    explicit_name_in_toml: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessfulResponse {
    warnings: HashMap<String, Vec<String>>,
}

impl SuccessfulResponse {
    pub fn new() -> Self {
        let mut warnings = HashMap::new();
        warnings.insert("invalid_categories".to_owned(), Vec::new());
        warnings.insert("invalid_badges".to_owned(), Vec::new());
        warnings.insert("other".to_owned(), Vec::new());

        SuccessfulResponse { warnings }
    }
}

// TODO: I shouldn't block the request
pub fn publish(
    owner: Owner,
    mut body: warp::body::FullBody,
    app: Arc<Application>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = app.pool.get().unwrap();

    // TODO: Replace the body handling with a Filter?
    let metadata_length = body.get_u32_le();

    let mut metadata_json = vec![0; metadata_length as usize];
    body.copy_to_slice(&mut metadata_json);

    let crate_upload: CrateUpload = serde_json::from_slice(&metadata_json).map_err(custom)?;

    let crate_length = body.get_u32_le();

    let mut crate_bytes = vec![0; crate_length as usize];
    body.copy_to_slice(&mut crate_bytes);

    let hash = Sha256::digest(&crate_bytes);

    let deps = crate_upload
        .deps
        .iter()
        .map(|dep| Dependency {
            name: dep.name.to_owned(),
            req: dep.version_req.clone(),
            features: dep.features.clone(),
            optional: dep.optional,
            default_features: dep.default_features,
            target: dep.target.clone(),
            kind: dep.kind,
            registry: dep.registry.clone(),
            package: dep.explicit_name_in_toml.clone(),
        })
        .collect::<Vec<_>>();

    // Check each dependency isn't used a registry that isn't allowed
    app.dependency_registry_allowed(&deps).map_err(custom)?;

    let metadata = Metadata {
        name: crate_upload.name,
        vers: crate_upload.vers,
        deps,
        cksum: format!("{:x}", hash),
        features: crate_upload.features,
        yanked: false,
        links: None,
    };

    let krate = match Krate::by_name(&conn, &metadata.name).map_err(custom)? {
        Some(k) => {
            // Check we have permission to perform acctions on this crate.
            super::has_crate_permission(&conn, k.id, owner.id)?;
            k
        }
        None => {
            let new_krate = NewKrate {
                name: &metadata.name,
                description: crate_upload.description.as_ref().map(|x| &**x),
            };

            let krate = new_krate.save(&conn).map_err(custom)?;

            let new_krate_owner = NewKrateOwner {
                krate_id: krate.id,
                owner_id: owner.id,
            };

            new_krate_owner.save(&conn).map_err(custom)?;

            krate
        }
    };

    let new_version = NewVersion {
        krate_id: krate.id,
        vers: &metadata.vers.to_string(),
        yanked: false,
    };

    new_version.save(&conn).map_err(custom)?;

    // Upload to storage
    app.storage
        .put(&metadata.name, &new_version.vers, &crate_bytes)
        .map_err(custom)?;

    // Save to registry
    crate::add_crate(&app, &metadata).map_err(custom)?;

    let resp = SuccessfulResponse::new();

    Ok(warp::reply::json(&resp))
}
