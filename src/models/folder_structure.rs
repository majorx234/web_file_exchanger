use crate::models::error::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FolderStructure {
    pub folder: Vec<String>,
    pub files: Vec<String>,
}
