use clap::{command, Arg, ArgAction};
use ignore::Walk;
use std::collections::HashMap;
use std::env;
use std::process::exit;
use symcount::{count_symbols, filter_files, read_file_to_string, ToString};

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

    let files = match filter_files(Walk::new(&root_dir), &extensions) {
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

    let verbose = *arg_matches.get_one::<bool>("verbose").expect(
        "This should never return a 'None', since the 'verbose' argument is created with the
        'ArgAction::SetTrue'. The clap API guarantees that there is always a return value when
        using that particular arg action.",
    );

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
