use std::collections::BinaryHeap;

use aws_sdk_s3::types::{CommonPrefix, Object};

use crate::s3::{S3, types::{bucket::StreeBucket, object::StreeObject}};

#[derive(Debug)]
pub struct ObjectListOption {
    pub bucket: String,
    pub limit: Option<i32>,
    pub marker: Option<String>,
    pub prefix: Option<String>,
    pub directory: Option<String>,
}

#[derive(Debug)]
pub struct BucketListOption {
    pub token: Option<String>,
    pub limit: Option<i32>,
    pub prefix: Option<String>,
}

impl S3 {
    pub async fn object_list(&self, option: ObjectListOption) -> BinaryHeap<StreeObject> {
        let prefix =  if let Some(directory) = &option.directory {
            if directory.ends_with("/") { option.directory.clone() }
            else { Some(directory.to_owned() + "/") }
        } 
        else { option.prefix };

        let res = self.client.list_objects_v2()
            .bucket(option.bucket)
            .set_max_keys(option.limit)
            .set_prefix(prefix.clone())
            .set_delimiter(Some("/".to_string()))
            .send()
            .await;

        let mut heap: BinaryHeap<StreeObject> = BinaryHeap::new();

        if let Ok(res) = res {
            let contents = res.contents();
            let common_prefixes = res.common_prefixes(); 

            common_prefixes.iter().for_each(|common_prefix: &CommonPrefix| {
                heap.push(StreeObject::new_folder(common_prefix, option.directory.is_some(), &prefix)); 
            });
            contents.iter().for_each(|content: &Object| {
                heap.push(StreeObject::new(content, option.directory.is_some(), &prefix));
            });
        }
        heap
    }

    pub async fn bucket_list(&self, option: BucketListOption) -> Vec<StreeBucket> {
        let res = self.client.list_buckets()
            .set_max_buckets(option.limit)
            .set_continuation_token(option.token)
            .set_prefix(option.prefix)
            .send().await;

        if let Ok(res) = res {
            return res.buckets().iter().map(StreeBucket::new).collect();
        }

        vec![]
    }
}
