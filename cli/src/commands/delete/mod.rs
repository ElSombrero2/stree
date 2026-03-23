use stree::{config::Config, s3::S3};

use crate::utils::confirm::confirm;

pub async fn remove(cfg: &Config, bucket: String, key: Option<String>, skip: bool) {
    let client = S3::new(cfg);
    let token = if let Some(key) = key.clone() { key } else { bucket.clone() };
    let message = format!("⚠️ Warning: you are going to permanently delete the file {}! Are you sure?", &token);
    if confirm(message, skip) {
        if client.remove(bucket, key).await {
            println!("✅ {token} successfully removed");
        } else {
            println!("❌ An error occured, please try again later!");
        }
    } else {
        println!("Action canceled by the user!");
    }
}

