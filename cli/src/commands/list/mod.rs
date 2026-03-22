use anyhow::Result;
use comfy_table::presets;
use stree::{config::Config, s3::{S3, list::{BucketListOption, ObjectListOption}}};

use crate::utils::table::get_table;

#[derive(Debug)]
pub struct ListOption {
  pub bucket: Option<String>,
  pub limit: Option<i32>,
  pub marker: Option<String>,
  pub prefix: Option<String>
}

pub async fn list(config: &Config, option: ListOption) -> Result<()> {
    let client = S3::new(config);
    let mut table= get_table(presets::NOTHING);

    if let Some(bucket) = option.bucket {
      table.set_header(vec!["Name", "Size", "Last modified"]);
      let objects = client.object_list(ObjectListOption {
        bucket,
        limit: option.limit,
        marker: option.marker,
        prefix: option.prefix 
      }).await; 

      for object in objects {
        table.add_row(vec![
          "📜 ".to_owned() + &object.key,
          object.size.to_string(),
          object.last_modified
        ]);
      }
    } else {
      table.set_header(vec!["Name", "Creation date"]);
      let buckets = client.bucket_list(BucketListOption {
        token: option.marker,
        limit: option.limit,
        prefix: option.prefix,
      }).await; 

      for bucket in buckets {
        table.add_row(vec![
          "📂 ".to_owned() + &bucket.name,
          bucket.creation_date,
        ]);
      }
    }
    println!("{table}");
    Ok(())
}