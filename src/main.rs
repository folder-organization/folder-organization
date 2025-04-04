mod subfolders;
mod markdown;
mod json;

use std::path::Path;
use std::env;
use crate::json::{build_folder_structure, save_structure_to_json, save_to_json};
use crate::markdown::update_readme;
use crate::subfolders::{get_subfolders_name, get_subfolders_path};


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

    // let r = get_subfolders_name(subfolders);

    println!();
    // println!("{:?}", r);
    // save_to_json(&r, "output.json").unwrap();

    let structure = build_folder_structure(subfolders);

    save_structure_to_json(&structure, "structure.json").unwrap();

    // for (folder, subfolders) in r {
    //     update_readme(&folder, &subfolders).expect("TODO: panic message");
    // }
}
