use crate::s3::S3;

impl S3 {
    pub async fn remove(&self, bucket: String, key: Option<String>) -> bool {
        if key.is_none() {
           return self.client.delete_bucket().bucket(bucket).send().await.is_ok();
        }
        self.client.delete_object().bucket(bucket).key(key.unwrap()).send().await.is_ok()
    }
}
