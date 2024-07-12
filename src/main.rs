use std::fs::DirEntry;

use clap::Parser;

mod files;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    directory_name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let files: Vec<DirEntry> = files::files::get_all_listings_in_dir(args.directory_name);

    for dirEntry in files {
        println!("{} is a {}", dirEntry.path().to_str().unwrap(), dirEntry.metadata().unwrap().is_dir());
    }
}
