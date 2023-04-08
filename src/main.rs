mod args;
use std::{collections::HashMap, path::PathBuf};

use args::Args;
use chrono::{DateTime, Local};
use walkdir::WalkDir;

struct FileInfo {
    name: String,
    path: PathBuf,
    size: u64,
    created: DateTime<Local>,
}

fn main() {
    let Args { path } = Args::get();

    let mut map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    let size_treshold = 100_000_000;

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.metadata().unwrap().len() > size_treshold)
        .for_each(|e| {
            let name = String::from(e.file_name().to_string_lossy());
            let path = e.path().to_owned();
            let metadata = e.metadata().expect("couldn't extract metadata");
            let size = metadata.len();
            let _created = metadata
                .created()
                .expect("Err while getting created timestemp");
            let created = DateTime::<Local>::from(_created);

            let fi = FileInfo {
                name: name.clone(),
                path,
                size,
                created,
            };

            let map_entry = map.entry(name).or_insert(Vec::new());

            map_entry.push(fi);
        });

    map.into_iter().filter(|me| me.1.len() > 1).for_each(|me| {
        println!("############################################");
        let f_name = me.0;
        println!("File: {f_name}\n");

        me.1.iter().for_each(|fi| {
            let date = fi.created.format("%Y-%m-%d %H:%M:%S").to_string();

            println!("Name: {:?}", fi.name);
            println!("Path: {:?}", fi.path.display());
            println!("Created: {:?}", date);
            println!("Bytes: {}", fi.size);
        });
        println!("############################################\n");
    });
}
