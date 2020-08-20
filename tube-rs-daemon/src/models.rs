use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVideo {
    pub id: String,
    pub title: String,
    pub added_at: String,
    pub downloaded: bool,
    pub path: String,
    pub quality: String,
    pub options: Vec<NewVideoOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVideoOption {
    pub flag: String,
    pub val: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOptionGroup {
    pub name: String,
    pub default_options: Vec<NewDefaultOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDefaultOption {
    pub flag: String,
    pub val: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryIdString {
    pub id: String,
}

#[derive(Deserialize)]
pub struct QueryIdNumber {
    pub id: i32,
}

#[derive(Serialize)]
pub struct AddResult {
    pub ok: bool,
    pub content: String,
}
