use ignore::WalkBuilder;
use std::collections::HashMap;
use std::{env, fs};
use std::path::{Path, PathBuf};

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

fn update_readme(folder: &Path, subfolders: &[String]) -> std::io::Result<()> {
    let readme_path = folder.join("README.md");
    let mut content = String::new();

    // Vérifier si README.md existe et le lire
    if readme_path.exists() {
        return Ok(());
    }

    // Définir le texte de la section "Folder Organization"
    let new_section = format!(
        "# Folder Organization\n\n{}\n",
        subfolders
            .iter()
            .map(|name| format!("- {}", name))
            .collect::<Vec<_>>()
            .join("\n")
    );


    // Sinon, ajouter la section à la fin
    if !content.is_empty() {
        content.push_str("\n");
    }
    content.push_str(&new_section);


    // Écrire les modifications dans le fichier README.md
    fs::write(&readme_path, content)?;

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
