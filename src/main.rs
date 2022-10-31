use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

const SYMBOLS: [char; 29] = [
    '.', ',', '<', '>', '?', '/', '!', '"', '@', '$', '%', '\'', '(', ')', '|', '{', '}', '^', '&',
    '*', '~', '-', '[', ']', '#', '=', '+', ':', '\\',
];

fn main() -> std::io::Result<()> {
    let dir = fs::read_dir(".")?;
    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in dir {
        let entry = entry?;
        if entry.path().is_file() {
            file_paths.push(entry.path())
        }
    }

    let mut symbol_counts: HashMap<char, usize> = HashMap::new();
    for path in file_paths {
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

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
