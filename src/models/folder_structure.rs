use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FolderStructure {
    pub folder: Vec<String>,
    pub files: Vec<String>,
}
