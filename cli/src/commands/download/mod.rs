use std::fs;

use stree::{config::Config, s3::S3};

fn get_file_name(key: String) -> String {
    let splited_key: Vec<&str> = key.split('/').collect();
    return String::from(splited_key[splited_key.len() - 1]);
}

pub async fn download (cfg: &Config, bucket: String, path: String, keys: Vec<String>) {
    let client = S3::new(cfg);

    for key in keys {
        if let Some(bytes) = client.download(bucket.clone(), key.clone()).await {
            let path = format!("{}{}" , path.clone(), get_file_name(key.clone())).replace("//", "/");
            if fs::write(path.clone(), bytes).is_ok() {
                println!("✅ '{path}' saved!");  
            } else {
                println!("❌ Can't download '{path}'");
            }
        }
    }
}

