use regex::Regex;
use std::fs;
use std::path::Path;

pub(crate) fn default_description(folder_name: &str) -> &str {
    match folder_name {
        "docs" => "Documentation.",
        "src" => "Source code.",
        "tests" => "Unit tests.",
        _ => "todo",
    }
}

pub(crate) fn markdown_content(folder: &Path, subfolders: &[String]) -> (String, String) {
    let folder_name = folder
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_string();

    let header = format!(
        "# {} folder\n\n## Description\n\n{}\n",
        folder_name,
        default_description(&folder_name)
    );

    // Définir le texte de la section "Folder Organization"
    let mut new_section = format!(
        "## Folder organization\n\n{}\n",
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

pub(crate) fn update_readme(folder: &Path, subfolders: &[String]) -> std::io::Result<()> {
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
            format!("{}\n{}", readme_content, new_section)
        };

        // Écrire le contenu mis à jour dans le fichier
        fs::write(&readme_path, updated_content)?;

        return Ok(());
    }

    // Écrire les modifications dans le fichier README.md
    fs::write(&readme_path, format!("{}\n{}", header, new_section))?;

    Ok(())
}
