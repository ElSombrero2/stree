use crate::s3::S3;

impl S3 {
    pub async fn download (&self, bucket: String, key: String) -> Option<Vec<u8>> {
        if let Ok(obj) = self.client.get_object().bucket(bucket).key(key).send().await {
            if let Ok(byte_arr) = obj.body.collect().await {
                 return Some(byte_arr.to_vec());
            } 
        }
        return None;
    }
}
