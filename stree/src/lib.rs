use anyhow::{Ok, Result};
use aws_sdk_s3::{Client, config::{BehaviorVersion, Credentials, Region}};

use crate::config::Config;

pub mod config;
/*
  [TESTING]: Add S3 client
  #file_system #improvement #high
*/

/*
  [TODO]: Create a tauri desktop
  Create Tauri desktop app based to these
  models
  ![image](https://cdn.dribbble.com/userupload/26533108/file/original-fbc4b46234c355feb1b618cebeb61fda.png?resize=752x564&vertical=center)
  ![image](https://cdn.dribbble.com/userupload/26533128/file/original-f2b9a4a2b34d3a0e044141a92c186a00.png?resize=752x564&vertical=center)
  #ui #low #improvement
*/

pub async fn get_client(config: &Config) -> Result<Client> {

  let credential = Credentials::builder()
  .access_key_id(config.credential.clone().unwrap().access_key.unwrap())
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

  Ok(client)
}
