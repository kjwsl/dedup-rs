pub mod cli;

use std::{collections::HashMap, fs, path::PathBuf};

pub fn find_dups(path: &str) -> Result<HashMap<String, Vec<PathBuf>>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    let files = list_files_in_dir(path, true)?;

    println!("Found {} files", files.len());

    for file in files {
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let entry = map.entry(file_name).or_insert_with(Vec::new);
        entry.push(file);
    }

    map.retain(|_, v| v.len() > 1);

    Ok(map)
}

fn list_files_in_dir(
    path: &str,
    recursive: bool,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut ret = Vec::new();
    let paths = fs::read_dir(path)?;
    for path in paths {
        let path = path?.path();
        if path.is_dir() && recursive {
            ret.append(&mut list_files_in_dir(path.to_str().unwrap(), true)?)
        } else {
            ret.push(path);
        }
    }
    Ok(ret)
}
