use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileIndex {
    file_paths: Vec<PathBuf>,
    file_names_and_idx: Vec<(String, usize)>,
}

impl FileIndex {
    pub fn add_folder(mut self, folder_path: &Path) {
        todo!("implement me");
    }
    pub fn add_file(mut self, file_path: &Path) {
        self.file_paths.push(file_path.to_path_buf());
    }
    pub fn create_index(start_dir_path: &Path) -> Self {
        let mut file_index: Vec<PathBuf> = Vec::new();
        let mut folder_stack = Vec::new();
        folder_stack.push(start_dir_path.to_path_buf());

        while !folder_stack.is_empty() {
            if let Some(current_folder) = folder_stack.pop() {
                let files = fs::read_dir(current_folder.as_path()).unwrap();
                for file_path in files {
                    let file_path = file_path.unwrap().path();
                    if file_path.is_dir() {
                        folder_stack.push(file_path);
                        continue;
                    }
                    if file_path.is_file() {
                        file_index.push(
                            file_path
                                .strip_prefix(start_dir_path)
                                .unwrap()
                                .to_path_buf(),
                        );
                    }
                }
            }
        }
        let mut file_names_and_idx = Vec::new();

        file_index.iter().enumerate().for_each(|(idx, file_path)| {
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            file_names_and_idx.push((file_name, idx as usize));
        });
        FileIndex {
            file_paths: file_index,
            file_names_and_idx,
        }
    }

    pub fn search(&self, search_str: &str) -> Option<Vec<String>> {
        let indexes = self
            .file_names_and_idx
            .iter()
            .filter_map(|(name, idx)| (name.contains(search_str)).then(|| *idx))
            .collect::<Vec<_>>();
        let mut found_file_path_names = Vec::new();
        indexes.iter().for_each(|idx| {
            found_file_path_names.push(String::from(self.file_paths[*idx].to_str().unwrap()))
        });
        Some(found_file_path_names)
    }
}
