extern crate mercer;

use mercer::*;
use std::collections::HashMap;

use std::path::PathBuf;

fn test_all_of_something(label: &str, results: &HashMap<PathBuf, RPGResult<()>>) {
    println!("\n==={}===\n", label);

    let mut ids = results.keys().collect::<Vec<&PathBuf>>();
    ids.sort();

    for id in ids {
        let item = results.get(id).unwrap();

        if let Err(e) = item {
            println!("(err) {}: {}", id.to_str().unwrap(), e);
        } else {
            println!("(ok)  {}", id.to_str().unwrap());
        }
    }
}

fn main() -> RPGResult<()> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut tome_dir = PathBuf::from(&cargo_manifest_dir);
    tome_dir.push("tome");

    let mut tome = mercer::Tome::new(tome_dir.clone());

    test_all_of_something("items", &tome.test_all_items());
    test_all_of_something("spells", &tome.test_all_spells());

    Ok(())
}
