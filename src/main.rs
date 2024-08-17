use ignore::WalkBuilder;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn get_subfolders_path(folder_path: &Path) -> Vec<PathBuf> {
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

fn get_subfolders_name(folders_path: Vec<PathBuf>) -> HashMap<PathBuf, Vec<String>> {
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

fn default_description(folder_name: &str) -> &str {
    match folder_name {
        "docs" => "Documentation.",
        "src" => "Source code.",
        "tests" => "Unit tests.",
        _ => "todo",
    }
}

fn markdown_content(folder: &Path, subfolders: &[String]) -> (String, String) {
    let folder_name = folder
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_string();

    let header = format!("# {} folder\n\n## Description\n\ntodo\n", folder_name);

    // Définir le texte de la section "Folder Organization"
    let mut new_section = format!(
        "\n## Folder organization\n\n{}\n",
        subfolders
            .iter()
            .map(|name| format!("[`📂 {}`]({})\n> {}", name, name, default_description(name)))
            .collect::<Vec<_>>()
            .join("\n\n")
    );

    if subfolders.is_empty() {
        new_section = "".to_string();
    }

    (header, new_section)
}

fn update_readme(folder: &Path, subfolders: &[String]) -> std::io::Result<()> {
    let readme_path = folder.join("README.md");
    let (header, new_section) = markdown_content(folder, subfolders);

    // Vérifier si README.md existe et le lire
    if readme_path.exists() {
        let readme_content = fs::read_to_string(&readme_path)?;
        // Définir une expression régulière pour capturer la section
        let re = Regex::new(r"(?s)(## Folder organization.*?)(\n## |\z)").unwrap();

        // Vérifier si la section existe
        let updated_content = if let Some(captures) = re.captures(&readme_content) {
            // Remplacer la section capturée par le nouveau contenu
            let section_start = &captures[1];
            readme_content.replacen(section_start, &new_section, 1)
        } else {
            // Ajouter la section à la fin si elle n'existe pas
            format!("{}{}", readme_content, new_section)
        };

        // Écrire le contenu mis à jour dans le fichier
        fs::write(&readme_path, updated_content)?;

        return Ok(());
    }

    // Écrire les modifications dans le fichier README.md
    fs::write(&readme_path, format!("{}{}", header, new_section))?;

    Ok(())
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

    let subfolders = get_subfolders_path(path);
    let mut result = String::new();

    for folder in &subfolders {
        result.push_str(&format!(
            "[`📂 {}`]({})\n> todo\n\n",
            folder.display(),
            folder.display()
        ));
    }

    println!("{}", result);

    let r = get_subfolders_name(subfolders);

    println!();
    println!("{:?}", r);

    for (folder, subfolders) in r {
        update_readme(&folder, &subfolders).expect("TODO: panic message");
    }
}
