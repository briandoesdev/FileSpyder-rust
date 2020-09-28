use walkdir::WalkDir;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    path: String,
    query: String,
}

fn main() {
    let args = Cli::from_args();
    check_directory(&args.path, &args.query);
}

fn check_directory(path: &str, query: &str) {
    let mut total_files_scanned = 0;
    let mut total_files_found = 0;
    let mut discovered_files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        total_files_scanned += 1;

        if entry.metadata().unwrap().is_file() {
            let filename = entry.file_name();
            if filename.eq(query) {
                total_files_found += 1;
                discovered_files.push(entry);
            }
        }
    }

    for file in discovered_files {
        println!("{}", file.path().display());
    }

    println!();
    println!("Total files found: {}", total_files_found);
    println!("Total files searched: {}", total_files_scanned);
}