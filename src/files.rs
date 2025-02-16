use std::{
    fs::{self},
    io,
    path::{Path, PathBuf},
};

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

