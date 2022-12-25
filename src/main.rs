use anyhow::Result;
use clap::{command, Arg, ArgAction};
use itertools::intersperse;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use walkdir::WalkDir;

const SYMBOLS: [char; 29] = [
    '.', ',', '<', '>', '?', '/', '!', '"', '@', '$', '%', '\'', '(', ')', '|', '{', '}', '^', '&',
    '*', '~', '-', '[', ']', '#', '=', '+', ':', '\\',
];

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<String> {
    fn to_string(&self) -> String {
        match self.len() {
            0 => String::from(""),
            1 => self.first().unwrap().to_owned(),
            _ => intersperse(self.clone(), String::from(", ")).collect::<String>(),
        }
    }
}

fn main() {
    let arg_matches = command!()
        .arg(
            Arg::new("extension")
                .long("extension")
                .short('e')
                .action(ArgAction::Append)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .get_matches();

    let root_dir = env::current_dir().expect("error getting current working directory:");
    let extensions = arg_matches
        .get_many("extension")
        .map(|exts| exts.cloned().collect::<Vec<String>>());

    let files = match filter_files(WalkDir::new(&root_dir), &extensions) {
        Some(files) => files,
        None => {
            if let Some(extensions) = extensions {
                let mut extension_word = "extension";
                if extensions.len() > 1 {
                    extension_word = "extensions";
                }
                println!(
                    "No files with the {} '{}' found in '{}'",
                    extension_word,
                    extensions.to_string(),
                    root_dir.to_string_lossy()
                );
            } else {
                println!("No files found in '{}'", root_dir.to_string_lossy());
            }
            exit(0);
        }
    };

    // NOTE: unwrapping here is fine, the clap API guarantees that the flag is always present in
    // the matches when using the 'ArgAction::SetTrue'
    let verbose = *arg_matches.get_one::<bool>("verbose").unwrap();

    let mut symbol_counts: HashMap<char, usize> = HashMap::new();
    for file in files {
        let content = match read_file_to_string(file) {
            Ok(content) => content,
            Err(err) => {
                if verbose {
                    eprintln!("{}", err);
                }
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

fn filter_files(root_dir: WalkDir, extensions: &Option<Vec<String>>) -> Option<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = root_dir
        .into_iter()
        .filter_map(|maybe_entry| maybe_entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|file_entry| file_entry.into_path())
        .collect();

    if let Some(extensions) = extensions {
        files.retain(|file| match file.extension() {
            Some(file_ext) => extensions.contains(&file_ext.to_string_lossy().to_string()),
            None => false,
        });
    }

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

#[cfg(test)]
mod to_string_trait_tests {
    use crate::ToString;

    #[test]
    fn empty_vec_returns_empty_string() {
        assert!(*"" == Vec::new().to_string());
    }

    #[test]
    fn only_one_element_returns_that_element() {
        assert_eq!(
            String::from("first"),
            vec![String::from("first")].to_string()
        );
    }

    #[test]
    fn multiple_elements_get_joined_by_comma_and_space() {
        assert_eq!(
            String::from("first, second"),
            vec![String::from("first"), String::from("second")].to_string()
        );
        assert_eq!(
            String::from("first, second, third"),
            vec![
                String::from("first"),
                String::from("second"),
                String::from("third")
            ]
            .to_string()
        );
    }
}
