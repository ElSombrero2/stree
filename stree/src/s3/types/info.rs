use aws_sdk_s3::{operation::head_object::HeadObjectOutput};


#[derive(Debug, Clone)]
pub struct StreeInfoObject {
  pub key: String,
  pub size: i64,
  pub last_modified: String,
  pub content_type: String,
  pub encryption: Option<String>,
  pub expires: Option<String>,
  pub current_version: Option<String>,
}

impl StreeInfoObject {
  pub fn new(key: String, object: HeadObjectOutput) -> StreeInfoObject {
    let encryption: Option<String> = object.server_side_encryption().map(|encryption| encryption.to_string());

    StreeInfoObject { 
      key,
      size: object.content_length.unwrap_or_default(),
      last_modified: object.last_modified.unwrap().to_string(),
      content_type: object.content_type.unwrap_or_default(),
      encryption,
      expires: object.expires_string,
      current_version: object.version_id,
    }
  }
}