use aws_sdk_s3::primitives::ByteStream;

use crate::s3::S3;

impl S3 {
    pub async fn upload(&self, bucket: String, key: String, file: Vec<u8>) -> bool {
        self.client.put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(file)).send().await.is_ok()
    }
}
