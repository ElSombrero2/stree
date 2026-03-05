use crate::s3::{S3, types::{bucket::StreeBucket, object::StreeObject}};

#[derive(Debug)]
pub struct ObjectListOption {
  pub bucket: String,
  pub limit: Option<i32>,
  pub marker: Option<String>,
  pub prefix: Option<String>
}

#[derive(Debug)]
pub struct BucketListOption {
  pub token: Option<String>,
  pub limit: Option<i32>,
  pub prefix: Option<String>,
}

impl S3 {
  pub async fn object_list(&self, option: ObjectListOption) -> Vec<StreeObject> {
    let res= self.client.list_objects()
    .bucket(option.bucket)
    .set_marker(option.marker)
    .set_max_keys(option.limit)
    .set_prefix(option.prefix)
    .send().await;

    if let Ok(res) = res {
      return res.contents().iter().map(StreeObject::new).collect();
    }

    vec![]
  }

  pub async fn bucket_list(&self, option: BucketListOption) -> Vec<StreeBucket> {
    let res = self.client.list_buckets()
    .set_max_buckets(option.limit)
    .set_continuation_token(option.token)
    .set_prefix(option.prefix)
    .send().await;

    if let Ok(res) = res {
      return res.buckets().iter().map(StreeBucket::new).collect();
    }

    vec![]
  }
}