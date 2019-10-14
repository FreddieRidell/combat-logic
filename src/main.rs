extern crate rpg_engine;

use rpg_engine::*;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let mut item_tome = rpg_engine::Tome::<ItemSpec, ItemInstance>::new("tome/items");

    for entry in WalkDir::new("tome/items") {
        let entry = entry.unwrap();
        if (entry.file_type().is_file()) {
            let full_path_name: &str =
                &entry.path().to_str().expect("couldn't read path as string");
            let path_name = &full_path_name[11..full_path_name.len() - 5];

            println!("check {} ", &full_path_name);
            item_tome.get_instance(path_name);
            println!("(ok)");
        }
    }
}
