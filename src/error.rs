use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    JSON(serde_json::Error),
    IO(io::Error),
    DB(diesel::result::Error),
    Pool(r2d2::Error),
    Git(git2::Error),
    InvalidRef(String),
    Unauthorized,
    MissingOwners,
    #[cfg(feature = "s3")]
    UploadS3(rusoto_core::RusotoError<rusoto_s3::PutObjectError>),
    DisallowedRegistry(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JSON(ref err) => err.fmt(f),
            Error::IO(ref err) => err.fmt(f),
            Error::DB(ref err) => err.fmt(f),
            Error::Pool(ref err) => err.fmt(f),
            Error::Git(ref err) => err.fmt(f),
            Error::InvalidRef(ref status) => write!(f, "failed to push a ref: {}", status),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::MissingOwners => write!(f, "No owners provided"),
            #[cfg(feature = "s3")]
            Error::UploadS3(ref err) => err.fmt(f),
            Error::DisallowedRegistry(ref krate, ref registry) => {
                write!(f, "Crate {}'s registry {} is not allowed", krate, registry)
            }
        }
    }
}

impl error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JSON(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DB(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::Pool(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err)
    }
}
