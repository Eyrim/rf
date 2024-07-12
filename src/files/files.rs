use std::fs::{self, DirEntry};

pub fn get_all_listings_in_dir(path: String) -> Vec<DirEntry> {
    match fs::read_dir(path) {
        Ok(entries) => make_valid_entry_list(entries),
        Err(e) => {
            eprintln!("Error: {}", e);

            std::process::exit(1)
        }
    }
}

fn make_valid_entry_list(entries: fs::ReadDir) -> Vec<DirEntry> {
    let mut entry_vec = Vec::new();

    for entry in entries {
        match entry {
            Ok(entry) => entry_vec.push(entry),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    entry_vec
}
