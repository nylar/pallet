#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "s3")]
pub mod s3;

#[cfg(feature = "local")]
pub type Storage = local::Local;
#[cfg(feature = "s3")]
pub type Storage = s3::S3;

pub(crate) fn crate_path(name: &str, version: &str) -> String {
    format!("crates/{}/{}-{}.crate", name, name, version)
}
