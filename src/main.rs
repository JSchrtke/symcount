use anyhow::Result;
use clap::{command, Arg, ArgAction};
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

const EXTENSION_ARG_NAME: &str = "extension";
const EXTENSION_ARG_SHORT_NAME: char = 'e';

fn main() {
    let args = command!()
        .arg(
            Arg::new(EXTENSION_ARG_NAME)
                .long(EXTENSION_ARG_NAME)
                .short(EXTENSION_ARG_SHORT_NAME)
                .action(ArgAction::Append)
                .required(false),
        )
        .get_matches();

    let dir_path = env::current_dir().expect("error getting current working directory:");
    let mut files = filter_files(WalkDir::new(&dir_path))
        .unwrap_or_else(|| panic!("found no files in {}", dir_path.to_string_lossy()));

    if let Some(extensions) = args.get_many::<String>("extension") {
        let extensions = extensions
            .map(|extension| extension.to_string())
            .collect::<Vec<String>>();

        files.retain(|file| {
            if let Some(ext) = file.extension() {
                let ext = ext.to_string_lossy().to_string();
                extensions.contains(&ext)
            } else {
                false
            }
        });
    }

    let mut symbol_counts: HashMap<char, usize> = HashMap::new();
    for file in files {
        let content = match read_file_to_string(file) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        count_symbols(content, &mut symbol_counts);
    }

    let mut counts = Vec::from_iter(symbol_counts.iter());
    counts.sort_unstable_by(|a, b| b.1.cmp(a.1));
    for count in counts {
        println!("{}: {}", count.0, count.1)
    }
}

fn filter_files(dir: WalkDir) -> Option<Vec<PathBuf>> {
    let files: Vec<PathBuf> = dir
        .into_iter()
        .filter_map(|maybe_entry| maybe_entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|file_entry| file_entry.into_path())
        .collect();

    if files.is_empty() {
        None
    } else {
        Some(files)
    }
}

fn read_file_to_string(path: PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn count_symbols(content: String, symbols: &mut HashMap<char, usize>) {
    for symbol in content.chars().filter(|c| SYMBOLS.contains(c)) {
        symbols
            .entry(symbol)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
}
