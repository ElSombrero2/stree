use comfy_table::presets;
use stree::{config::Config, s3::{list::{BucketListOption, ObjectListOption}, types::object::StreeObjectKind, S3}};

use crate::utils::table::get_table;

#[derive(Debug)]
pub struct ListOption {
    pub bucket: Option<String>,
    pub limit: Option<i32>,
    pub marker: Option<String>,
    pub prefix: Option<String>,
    pub directory: Option<String>,
}

pub async fn list(config: &Config, option: ListOption) {
    let client = S3::new(config);
    let mut table= get_table(presets::NOTHING);

    if let Some(bucket) = option.bucket {
        table.set_header(vec!["Name", "Size", "Last modified"]);
        let objects = client.object_list(ObjectListOption {
            bucket,
            limit: option.limit,
            marker: option.marker,
            prefix: option.prefix,
            directory: option.directory,
        }).await; 

        for object in objects {
            if object.kind == StreeObjectKind::File {
                table.add_row(vec![
                    "📄 ".to_owned() + &object.key,
                    object.size.unwrap_or_default().to_string(),
                    object.last_modified.unwrap().to_string()
                ]);
            } else {
                table.add_row(vec![
                    "📂 ".to_owned() + &object.key,
                    String::from("-"),
                    String::from("-"),
                ]);

            }
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
                "🪣 ".to_owned() + &bucket.name,
                bucket.creation_date,
            ]);
        }
    }
    println!("{table}");
}
