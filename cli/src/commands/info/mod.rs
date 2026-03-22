use comfy_table::{Cell, Row, presets};
use stree::{config::Config, s3::S3};

use crate::utils::table;

pub async fn get_info (config: &Config, bucket: String, key: String, version: Option<String>) {
  let client = S3::new(config);

  let info = client.info(bucket, key.clone(), version).await;
  if let Some(info) = info {
    let mut table = table::get_table(presets::UTF8_BORDERS_ONLY);
    table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);


    let mut row= Row::new();
    row.add_cell(Cell::new(""));
    row.add_cell(Cell::new("File Information"));
    table.set_header(row);
    let empty = String::from("-");
    
    table.add_row(vec!["File", &format!(": {}", info.key)]);
    table.add_row(vec!["Size", &format!(": {} KB", info.size)]);
    table.add_row(vec!["Last modified", &format!(": {}", info.last_modified)]);
    table.add_row(vec!["Version Id", &format!(": {}", info.current_version.unwrap_or(empty.clone()))]);
    table.add_row(vec!["Content Type", &format!(": {}", info.content_type)]);
    table.add_row(vec!["Expires", &format!(": {}", info.expires.unwrap_or(empty.clone()))]);
    table.add_row(vec!["Encryption", &format!(": {}", info.encryption.unwrap_or(empty))]);

  
    println!("{table}");
  } else {
    println!("File {key} not found");
  }
}