extern crate mercer;

use mercer::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> RPGResult<()> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut tome_dir = PathBuf::from(&cargo_manifest_dir);
    tome_dir.push("tome");

    let mut tome = mercer::Tome::new(tome_dir.clone());

    println!("\n===items===\n");
    for (id, item) in tome.test_all_items() {
        if let Err(e) = item {
            println!("(err) {} : {}", id.to_str().unwrap(), e);
        } else {
            println!("(ok)  {}", id.to_str().unwrap());
        }
    }

    println!("\n===spells===\n");
    for (id, item) in tome.test_all_spells() {
        if let Err(e) = item {
            println!("(err) {} : {}", id.to_str().unwrap(), e);
        } else {
            println!("(ok)  {}", id.to_str().unwrap());
        }
    }

    Ok(())
}
