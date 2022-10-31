use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

const SYMBOLS: [char; 29] = [
    '.', ',', '<', '>', '?', '/', '!', '"', '@', '$', '%', '\'', '(', ')', '|', '{', '}', '^', '&',
    '*', '~', '-', '[', ']', '#', '=', '+', ':', '\\',
];

fn main() -> std::io::Result<()> {
    let dir_path = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(err) => {
            eprintln!("error getting current directory: {}", err);
            std::process::exit(1);
        }
    };

    let dir = match fs::read_dir(&dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("error reading dir {}: {}", dir_path.display(), err);
            std::process::exit(1);
        }
    };

    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!(
                    "error while enumerating dir {}: {}",
                    dir_path.display(),
                    err
                );
                continue;
            }
        };

        if entry.path().is_file() {
            file_paths.push(entry.path())
        }
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
