use anyhow::Result;
use stree::{config::Config, get_client};

#[derive(Debug)]
pub struct ListOption {
  pub bucket: Option<String>,
  pub limit: Option<i32>,
  pub marker: Option<String>,
  pub prefix: Option<String>
}

pub async fn list(config: &Config, option: ListOption) -> Result<()> {
    let client = get_client(config).await.unwrap();
    if let Some(bucket) = option.bucket {
      let res= client.list_objects()
      .bucket(bucket)
      .set_marker(option.marker)
      .set_max_keys(option.limit)
      .set_prefix(option.prefix)
      .send().await;

      if let Ok(res) = res {
        for data in res.contents() {
          println!("📇 {}\t\t{}", data.key().unwrap(), data.last_modified().unwrap());
        }
      }
    } else {
      let res = client.list_buckets()
      .set_max_buckets(option.limit)
      .send().await;
      if let Ok(result) = res {
         for bucket in result.buckets() {
          println!("📁 {}", bucket.name().unwrap());
      }
      }
    }
    Ok(())
}