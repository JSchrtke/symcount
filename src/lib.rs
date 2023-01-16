use anyhow::Result;
use ignore::WalkBuilder;
use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use itertools::{intersperse, Itertools};

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<String> {
    fn to_string(&self) -> String {
        match self.len() {
            0 => String::from(""),
            1 => self
                .first()
                .expect("The match experession guarantees that the length is at least 1")
                .to_owned(),
            _ => intersperse(self.clone(), String::from(", ")).collect::<String>(),
        }
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

pub fn filter_files(path: &PathBuf, extensions: &Option<Vec<String>>) -> Option<Vec<PathBuf>> {
    let include_ignored = extensions.is_none();

    let dir_walker = WalkBuilder::new(path)
        .git_ignore(include_ignored)
        .ignore(include_ignored)
        .build();

    let mut files: Vec<PathBuf> = dir_walker
        .into_iter()
        .filter_map(|maybe_entry| maybe_entry.ok())
        .filter(|entry| entry.path().is_file())
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

#[cfg(test)]
mod test_filter_files {
    use crate::filter_files;
    use std::path::PathBuf;

    mod ignores_files_in_ignore_lists {
        use crate::filter_files;
        use std::path::PathBuf;

        #[test]
        fn when_given_no_filter() {
            let expected_files: Option<Vec<std::path::PathBuf>> = Some(vec![
                PathBuf::from("test-dir/file.a"),
                PathBuf::from("test-dir/file.b"),
                PathBuf::from("test-dir/file.c"),
            ]);

            let actual_files = filter_files(&PathBuf::from("test-dir"), &None);

            assert_eq!(expected_files, actual_files);
        }

        #[test]
        fn unless_explicitly_given_those_files() {
            let extensions = Some(vec![String::from("gitignored"), String::from("ignored")]);
            let expected_files: Option<Vec<std::path::PathBuf>> = Some(vec![
                PathBuf::from("test-dir/file.gitignored"),
                PathBuf::from("test-dir/file.ignored"),
            ]);
            let actual_files = filter_files(&PathBuf::from("test-dir"), &extensions);
            assert_eq!(expected_files, actual_files);
        }
    }

    #[test]
    fn only_includes_given_extensions() {
        let extensions = Some(vec![String::from("a"), String::from("b")]);
        let expected_files: Option<Vec<std::path::PathBuf>> = Some(vec![
            PathBuf::from("test-dir/file.a"),
            PathBuf::from("test-dir/file.b"),
        ]);
        let actual_files = filter_files(&PathBuf::from("test-dir"), &extensions);
        assert_eq!(expected_files, actual_files);
    }
}

pub fn read_file_to_string(path: PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub const SYMBOLS: [char; 30] = [
    '.', ',', '<', '>', '?', '/', '!', '"', '@', '$', '%', '\'', '(', ')', '|', '{', '}', '^', '&',
    '*', '~', '-', '[', ']', '#', '=', '+', ':', '\\', ';',
];

pub fn count_symbols(content: String, symbols: &mut HashMap<char, usize>) {
    for symbol in content.chars().filter(|c| SYMBOLS.contains(c)) {
        symbols
            .entry(symbol)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
}

pub fn count_symbol_bigrams(input: &str) -> Option<HashMap<String, u32>> {
    if input.is_empty() {
        return None;
    }

    let chunks = input.chars().chunks(2);
    let mut bigrams = Vec::new();
    for mut chunk in &chunks {
        let a = chunk.next().expect("since we check for empty once, even if just one element is given, this one is always gonna exist");
        if !SYMBOLS.contains(&a) {
            continue;
        }

        let b = match chunk.next() {
            Some(char) => char,
            None => continue,
        };
        if !SYMBOLS.contains(&b) {
            continue;
        }

        if a == b {
            return None;
        }
        let bigram = a.to_string() + &b.to_string();
        bigrams.push(bigram);
    }

    let mut bigram_counts: HashMap<String, u32> = HashMap::new();
    for bigram in bigrams {
        bigram_counts
            .entry(bigram)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    if bigram_counts.is_empty() {
        None
    } else {
        Some(bigram_counts)
    }
}

#[cfg(test)]
mod count_bigrams_tests {
    use std::collections::HashMap;

    use crate::count_symbol_bigrams;

    #[test]
    fn empty_string_contains_no_bigrams() {
        let input = String::from("");
        assert_eq!(None, count_symbol_bigrams(&input));
    }

    #[test]
    fn two_symbols_in_sequence_are_bigram() {
        let input = String::from(";/");
        let mut expected_map: HashMap<String, u32> = HashMap::new();
        expected_map.insert(input.to_owned(), 1);
        let expected = Some(expected_map);
        assert_eq!(expected, count_symbol_bigrams(&input));
    }

    #[test]
    fn multiple_bigrams_in_one_string() {
        let input = String::from(";/;/");
        let mut expected_map: HashMap<String, u32> = HashMap::new();
        expected_map.insert(input[0..2].to_string(), 2);
        let expected = Some(expected_map);
        assert_eq!(expected, count_symbol_bigrams(&input));
    }

    #[test]
    fn repeating_single_symbol_is_no_bigram() {
        let input = String::from(";;");
        assert_eq!(None, count_symbol_bigrams(&input));
    }

    #[test]
    fn symbols_that_are_separated_by_non_symbol_are_no_bigram() {
        let input = String::from(";a;");
        assert_eq!(None, count_symbol_bigrams(&input));
    }
}
