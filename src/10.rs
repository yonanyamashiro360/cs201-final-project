use std::{collections::HashMap, fs};

fn main() {
    let mut map = HashMap::new();
    for file in fs::read_dir(".").unwrap().filter_map(|f| f.ok()) {
        if file.path().extension().map_or(false, |ext| ext == "txt") {
            println!("{}", file.file_name().to_string_lossy());
            map.insert(file.path(), "".to_string());
        }
    }
}
