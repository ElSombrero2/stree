use std::{cmp::Ordering, ops::Deref};
use aws_sdk_s3::{primitives::DateTime, types::{CommonPrefix, Object}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreeObjectKind {
    Group,
    File,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StreeObject {
    pub key: String,
    pub size: Option<i64>,
    pub kind: StreeObjectKind,
    pub last_modified: Option<DateTime>,
}

impl StreeObject {
    pub fn new(object: &Object, will_remove_prefix: bool, prefix: &Option<String>) -> Self {
        let key = Self::format_key(object.key().unwrap_or_default().to_string(), will_remove_prefix, prefix);

        Self {
            key,
            size: object.size,
            last_modified: object.last_modified,
            kind: StreeObjectKind::File,
        }
    }

    pub fn new_folder(common_prefix: &CommonPrefix, will_remove_prefix: bool, prefix: &Option<String>) -> Self {
        let key = Self::format_key(common_prefix.prefix().unwrap_or_default().to_string(), will_remove_prefix, prefix);
        Self {
            key,
            size: None,
            last_modified: None,
            kind: StreeObjectKind::Group,
        }
    }

    fn format_key(key: String, will_remove_prefix: bool, prefix: &Option<String>) -> String {
        if will_remove_prefix && let Some(directory) = prefix {
            return key.replacen(directory.deref(), "", 1);
        }
        key
    }
}

impl Ord for StreeObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for StreeObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

