use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Recursively retrieves the paths of all subdirectories within a given folder.
///
/// This function uses [`WalkBuilder`] from the [`ignore`] crate to walk through the specified
/// folder, respecting `.gitignore` rules, and returns a list of all discovered subdirectories,
/// including the root folder if it is a directory.
///
/// # Arguments
///
/// * `folder_path` - A reference to a [`Path`] representing the folder to scan.
///
/// # Returns
///
/// A `Vec<PathBuf>` containing the paths of all subdirectories.
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use my_crate::get_subfolders_path; // Replace `my_crate` with your actual crate/module name
///
/// let subfolders = get_subfolders_path(Path::new("./src"));
/// for folder in subfolders {
///     println!("{:?}", folder);
/// }
/// ```
///
/// ```rust
/// use std::path::Path;
/// use my_crate::get_subfolders_path;
///
/// let subfolders = get_subfolders_path(Path::new("./src"));
/// assert!(subfolders.iter().any(|p| p.ends_with("src")));
/// ```
///
/// # Notes
///
/// - The root folder passed as input will be included in the output if it's a directory.
/// - Folders ignored by `.gitignore` will be excluded.
/// - Returned paths will be relative or absolute depending on the input path.
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
