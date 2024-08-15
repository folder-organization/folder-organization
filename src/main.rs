use ignore::WalkBuilder;
use std::env;
use std::path::Path;

fn get_subfolders_path(folder_path: &Path) -> Vec<String> {
    let mut subfolders_path: Vec<String> = Vec::new();

    let walker = WalkBuilder::new(folder_path)
        .max_depth(Some(1))
        .git_ignore(true)
        .build();

    for result in walker {
        if let Ok(entry) = result {
            let current_path = entry.path();

            if current_path.is_dir() && current_path != folder_path {
                if let Some(nom) = current_path.file_name() {
                    if let Some(nom_str) = nom.to_str() {
                        subfolders_path.push(nom_str.to_string());
                    }
                }
            }
        }
    }

    subfolders_path
}
fn get_subfolders_name(folder_path: &Path) -> Vec<String> {
    let mut subfolders_name: Vec<String> = Vec::new();

    let walker = WalkBuilder::new(folder_path)
        .max_depth(Some(1))
        .git_ignore(true)
        .build();

    for result in walker {
        if let Ok(entry) = result {
            let current_path = entry.path();

            if current_path.is_dir() && current_path != folder_path {
                if let Some(nom) = current_path.file_name() {
                    if let Some(nom_str) = nom.to_str() {
                        subfolders_name.push(nom_str.to_string());
                    }
                }
            }
        }
    }

    subfolders_name
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <dossier>", args[0]);
        return;
    }

    let folder = &args[1];
    let path = Path::new(folder);

    if !path.is_dir() {
        eprintln!("Path specified is not a valid folder: {}", folder);
        return;
    }

    // firstly, get all folders path as a Vector, e.g.
    // ["folder-organization", "folder-organization/src", "folder-organization/tests", "folder-organization/tests/sub1"]

    // secondly, get direct subfolders of each folder
    // {
    //  "folder-organization": ["src", "tests"],
    //  "folder-organization/src": [],
    //  "folder-organization/tests": ["sub1"],
    //  "folder-organization/tests/sub1": [],
    // }

    let subfolders = get_subfolders_name(path);
    let mut result = String::new();

    for folder in subfolders {
        result.push_str(&format!("[`📂 {}`]({})\n> todo\n\n", folder, folder));
    }

    println!("{}", result);
}
