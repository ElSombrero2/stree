use crate::s3::{S3, types::info::StreeInfoObject};

impl S3 {
  pub async fn info (&self, bucket: String, key: String, version: Option<String>) -> Option<StreeInfoObject> {
    if let Ok(res) = self.client.head_object()
    .bucket(bucket)
    .key(key.clone())
    .set_version_id(version)
    .send().await {
      return Some(StreeInfoObject::new(key, res));
    }
    None
  }
}