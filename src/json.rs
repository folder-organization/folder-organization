use std::collections::{BTreeMap, HashMap};
use std::path::{PathBuf, Path};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;
use ignore::WalkBuilder;

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct FolderInfo {
    #[serde(rename = "folder-name")]
    folder_name: String,
    title: String,
    children: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct FolderMap {
    folders: BTreeMap<String, FolderInfo>,
}

/// Replace "\\" in path for Windows
fn normalize_path(path: &Path) -> String {
    format!("./{}", path.strip_prefix(".").unwrap_or(path).display())
        .replace("\\", "/")
}

pub(crate) fn build_folder_structure(folders_path: Vec<PathBuf>) -> FolderMap {
    let mut folders = BTreeMap::new();

    for folder in folders_path {
        let mut children: Vec<String> = Vec::new();

        let walker = WalkBuilder::new(&folder)
            .max_depth(Some(1))
            .git_ignore(true)
            .build()
            .flatten();

        for entry in walker {
            let current_path = entry.path();

            if current_path.is_dir() && current_path != folder {
                children.push(normalize_path(current_path));
            }
        }

        // nom du dossier courant (ex: "src")
        let folder_name = folder.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".")
            .to_string();

        // clé du map (ex: "./src")
        let key = normalize_path(&folder);

        let info = FolderInfo {
            folder_name: folder_name.clone(),
            title: format!("{} folder", folder_name),
            children,
        };

        folders.insert(key, info);
    }

    FolderMap { folders }
}

pub(crate) fn save_structure_to_json(structure: &FolderMap, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(structure).expect("Erreur de sérialisation");
    std::fs::write(path, json)?;
    Ok(())
}



pub(crate) fn save_to_json(data: &HashMap<PathBuf, Vec<String>>, path: &str) -> std::io::Result<()> {
    // Convertir les clés PathBuf en String pour la sérialisation
    let converted: HashMap<String, &Vec<String>> = data
        .iter()
        .map(|(k, v)| (k.to_string_lossy().to_string(), v))
        .collect();

    let json = to_string_pretty(&converted).expect("Erreur de sérialisation");

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
