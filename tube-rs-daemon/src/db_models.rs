use crate::schema::{default_options, option_groups, video_options, videos};
use serde::{Deserialize, Serialize};

// Table videos
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "videos"]
pub struct Video {
    pub id: String,
    pub title: String,
    pub added_at: String,
    pub downloaded: bool,
    pub path: String,
    pub quality: String,
}

// Table options
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct VideoOption {
    pub id: i32,
    pub flag: String,
    pub val: Option<String>,
    pub video_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "video_options"]
pub struct InsertVideoOption {
    pub flag: String,
    pub val: Option<String>,
    pub video_id: String,
}

// Table option_groups
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptionGroup {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "option_groups"]
pub struct InsertOptionGroup {
    pub name: String,
}

// Table default_options
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct DefaultOption {
    pub id: i32,
    pub flag: String,
    pub val: Option<String>,
    pub group_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "default_options"]
pub struct InsertDefaultOption {
    pub flag: String,
    pub val: Option<String>,
    pub group_id: i32,
}
