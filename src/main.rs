use std::fs::File;
use std::io::Read;
use std::path::Path;

const symbols = [";"];

fn main() {
    let filepath = Path::new("./test.txt");
    let mut file: File = match File::open(&filepath) {
        Err(err) => panic!("Error opening file '{}': {}", filepath.display(), err),
        Ok(f) => f,
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .unwrap_or_else(|_| panic!("Error reading file '{}':", filepath.display()));

    for char in buf.chars() {
        if is_symbol(char) {
            println!("{}", char);
        }
    }
}

fn is_symbol(c: char) -> bool {
    false
}
