use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderStructure {
    pub filename: String,
    pub is_folder: bool,
    pub children: Option<Vec<FolderStructure>>,
}
