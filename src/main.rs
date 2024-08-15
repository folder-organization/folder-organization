use std::env;
use std::path::Path;
use ignore::WalkBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <dossier>", args[0]);
        return;
    }

    let dossier = &args[1];
    let chemin = Path::new(dossier);

    // Vérifie si le chemin est un dossier
    if !chemin.is_dir() {
        eprintln!("Le chemin spécifié n'est pas un dossier valide : {}", dossier);
        return;
    }

    let mut subfolders = Vec::new();
    let walker = WalkBuilder::new(chemin)
        .max_depth(Some(1))
        .git_ignore(true)
        .build();

    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.is_dir() && path != chemin {
                if let Some(nom) = path.file_name() {
                    if let Some(nom_str) = nom.to_str() {
                        subfolders.push(nom_str.to_string());
                    }
                }
            }
        }
    }

    let mut result = String::new();

    for folder in subfolders {
        result.push_str(&format!("[`📂 {}`]({})\n> todo\n\n", folder, folder));
    }

    println!("{}", result);
}
