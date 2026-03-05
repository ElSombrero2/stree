use aws_sdk_s3::types::Object;


pub struct StreeObject {
  pub key: String,
  pub size: i64,
  pub last_modified: String,
}

impl StreeObject {
  pub fn new(object: &Object) -> StreeObject {
    StreeObject {
      key: object.key.clone().unwrap_or_default(),
      size: object.size.unwrap_or_default(),
      last_modified: object.last_modified.unwrap().to_string(),
    }
  }
}
