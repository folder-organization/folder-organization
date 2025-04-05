use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn get_subfolders_path(folder_path: &Path) -> Vec<PathBuf> {
    let mut subfolders_path: Vec<PathBuf> = Vec::new();

    let walker = WalkBuilder::new(folder_path)
        .git_ignore(true)
        .build()
        .flatten();

    for entry in walker {
        let current_path = entry.path();

        if current_path.is_dir() {
            subfolders_path.push(current_path.to_path_buf());
        }
    }

    subfolders_path
}

pub(crate) fn get_subfolders_name(folders_path: Vec<PathBuf>) -> HashMap<PathBuf, Vec<String>> {
    let mut h = HashMap::new();

    for folder in folders_path {
        let mut subfolders_name: Vec<String> = Vec::new();

        let walker = WalkBuilder::new(&folder)
            .max_depth(Some(1))
            .git_ignore(true)
            .build()
            .flatten();

        for entry in walker {
            let current_path = entry.path();

            if current_path.is_dir() && current_path != folder {
                if let Some(name) = current_path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        subfolders_name.push(name_str.to_string());
                    }
                }
            }
        }

        h.insert(folder, subfolders_name);
    }

    h
}
