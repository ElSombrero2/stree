use std::fs;
use stree::{config::Config, s3::S3};
use crate::utils::confirm::confirm;

pub async fn upload_file(cfg: &Config, bucket: String, key: String, file: String, skip: bool) {
   let client = S3::new(cfg);
   
   if client.info(bucket.clone(), key.clone(), None).await.is_some() {
        if !confirm(format!("⚠️ Warning: the {key} already exists on your bucket, do you want to"), skip) {
            return println!("Action canceled by the user!");
        }
   }

   if let Ok(bytes) = fs::read(file.clone()) {
       if client.upload(bucket.clone(), key.clone(), bytes).await {
            println!("✅ {file} uploaded with the key {key} inside 📁 {bucket}!");
       } else {
           println!("❌ An error occured on uploading file!");
       }
   } else {
        println!("❌ Can't read the file {file}, please check if you can read it and you have the access to the file!");
   }
}
