extern crate rpg_engine;

use rpg_engine::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> RPGResult<()> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut tome_dir = PathBuf::from(&cargo_manifest_dir);
    tome_dir.push("tome");

    let mut tome = rpg_engine::Tome::new(tome_dir.clone());

    tome_dir.push("items");
    let mut tome_dir_string: String = tome_dir.clone().to_str().expect("1").to_owned();
    tome_dir_string.push('/');

    for entry in WalkDir::new(tome_dir) {
        let entry = entry.unwrap();
        if (entry.file_type().is_file()
            && !entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false))
        {
            let full_path_name: &str =
                &entry.path().to_str().expect("couldn't read path as string");

            let path_name: String = full_path_name.replace(&tome_dir_string, "");

            print!("check {} ", &full_path_name[1 + cargo_manifest_dir.len()..]);
            std::io::stdout().flush();

            match tome.create_item_instance(&path_name[..path_name.len()]) {
                Ok(_) => println!(" (ok)"),
                Err(e) => {
                    println!("\n{}", &e);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
