use stree::{config::Config, s3::S3};

pub async fn remove(cfg: &Config, bucket: String, key: Option<String>) {
    let client = S3::new(cfg);
    if client.remove(bucket.clone(), key.clone()).await {
        let token = if let Some(key) = key { key } else { bucket };
        println!("✅ {token} successfully removed");
    } else {
        println!("❌ An error occured, please try again later!");
    }
}
