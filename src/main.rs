use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

mod files;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        file_path: String,
        #[arg(short, long)]
        entry_type: EntryType,
    },
}

enum EntryType {
    File,
    Directory,
    SymLink,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::List {
            file_path,
            entry_type,
        }) => visit_dirs(file_path, entry_type),
        None => {}
    }
}

fn visit_dirs(file_path: &str, entry_type: &EntryType) {
    let dirs = files::visit_dirs(Path::new(&file_path)).unwrap();

    let paths: Vec<&str> = dirs
        .iter()
        .filter(filter_for_entry_type(entry_type))
        .map(|path_buf| path_buf.as_path())
        .map(|path| path.to_str().unwrap())
        .collect();

    println!("{:?}", paths);
}

fn filter_for_entry_type(entry_type: &EntryType) -> impl Fn(&PathBuf) -> bool {
    |path: &PathBuf| match entry_type {
        EntryType::File => path.is_file(),
        EntryType::Directory => path.is_dir(),
        EntryType::SymLink => path.is_symlink()
    }
}
