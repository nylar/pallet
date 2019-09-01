use std::time::Duration;

use crate::commands::Server;
use crate::error::Error;

use rusoto_core::Region;
use rusoto_credential::{AwsCredentials, StaticProvider};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3 as _};

#[derive(Clone)]
pub struct S3 {
    client: S3Client,
    bucket: String,
    region: Region,
    credentials: AwsCredentials,
}

impl S3 {
    pub fn new(server: &Server) -> Self {
        let aws_auth = StaticProvider::new_minimal(
            server.s3_opts.s3_access_key.to_owned(),
            server.s3_opts.s3_secret_key.to_owned(),
        );

        let aws_dispatcher = rusoto_core::request::HttpClient::new().unwrap();

        let client = S3Client::new_with(
            aws_dispatcher,
            aws_auth.clone(),
            server.s3_opts.s3_region.clone(),
        );

        let credentials = AwsCredentials::new(
            &server.s3_opts.s3_access_key,
            &server.s3_opts.s3_secret_key,
            None,
            None,
        );

        S3 {
            client,
            bucket: server.s3_opts.s3_bucket.to_owned(),
            region: server.s3_opts.s3_region.clone(),
            credentials,
        }
    }

    pub fn put(&self, name: &str, version: &str, content: &[u8]) -> Result<(), Error> {
        let key = super::crate_path(name, version);

        self.client
            .put_object(PutObjectRequest {
                bucket: self.bucket.to_owned(),
                key,
                body: Some(content.to_vec().into()),
                ..Default::default()
            })
            .with_timeout(Duration::from_secs(10)) // TODO: Make configurable
            .sync()
            .map_err(Error::UploadS3)?;
        Ok(())
    }

    pub fn get(&self, name: &str, version: &str) -> Result<String, Error> {
        let key = super::crate_path(name, version);

        let req = GetObjectRequest {
            bucket: self.bucket.to_owned(),
            key,
            ..Default::default()
        };

        Ok(req.get_presigned_url(
            &self.region,
            &self.credentials,
            &PreSignedRequestOption {
                expires_in: Duration::from_secs(10), // TODO: Make configurable
            },
        ))
    }
}
