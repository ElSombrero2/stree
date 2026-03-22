use aws_sdk_s3::{Client, config::{BehaviorVersion, Credentials, Region}};

use crate::config::Config;

pub mod list;
pub mod delete;
pub mod download;
pub mod info;
pub mod types;

pub struct S3 { client: Client }

impl S3 {
  pub fn new(config: &Config) -> S3 {
    let credential = Credentials::builder()
    .access_key_id(config.credential.clone().unwrap().access_key.unwrap_or_default())
    .secret_access_key(config.credential.clone().unwrap().secret_key.unwrap())
    .provider_name("stree-client")
    .build();

    let cfg = aws_sdk_s3::config::Builder::new()
    .region(Region::new(config.region.clone().unwrap()))
    .endpoint_url(config.endpoint.clone().unwrap())
    .credentials_provider(credential)
    .behavior_version(BehaviorVersion::latest())
    .force_path_style(config.force_path_style.unwrap_or(false)).build();

    let client = aws_sdk_s3::Client::from_conf(cfg);

    S3 { client }
  }
}
