use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use walkdir::WalkDir;

const SYMBOLS: [char; 29] = [
    '.', ',', '<', '>', '?', '/', '!', '"', '@', '$', '%', '\'', '(', ')', '|', '{', '}', '^', '&',
    '*', '~', '-', '[', ']', '#', '=', '+', ':', '\\',
];

fn main() -> std::io::Result<()> {
    let dir_path = env::current_dir().expect("error getting current working directory:");
    let dir = WalkDir::new(&dir_path);

    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in dir {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    file_paths.push(entry.path().to_owned())
                }
            }
            Err(err) => {
                eprintln!(
                    "error while enumerating dir {}: {}",
                    dir_path.display(),
                    err
                );
            }
        };
    }

    let mut symbol_counts: HashMap<char, usize> = HashMap::new();
    for path in file_paths {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("error opening file {}: {}", path.display(), err);
                continue;
            }
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("error reading from file {}: {}", path.display(), err);
                continue;
            }
        };

        content.chars().filter(is_symbol).for_each(|symbol| {
            symbol_counts
                .entry(symbol)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    }

    symbol_counts
        .iter()
        .for_each(|count| println!("{}: {}", count.0, count.1));

    Ok(())
}

fn is_symbol(c: &char) -> bool {
    SYMBOLS.contains(c)
}
