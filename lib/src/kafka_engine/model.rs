
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct FileModel {
    pub file_name: String,
    pub content: String,
}