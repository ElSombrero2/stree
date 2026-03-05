use aws_sdk_s3::types::Bucket;

pub struct StreeBucket {
  pub name: String,
  pub creation_date: String,
}

impl StreeBucket {
    pub fn new(bucket: &Bucket) -> StreeBucket {
      StreeBucket {
        name: bucket.name.clone().unwrap_or_default(),
        creation_date: bucket.creation_date.unwrap().to_string(),
      }
    }
}
