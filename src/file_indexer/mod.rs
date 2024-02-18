/*
 * This file is part of the web_file_exchanger distribution (https://github.com/majorx234/web_file_exchanger ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileIndex {
    file_paths: Vec<PathBuf>,
    file_names_and_idx: Vec<(String, usize)>,
}

impl FileIndex {
    pub fn add_folder(self, _folder_path: &Path) {
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
