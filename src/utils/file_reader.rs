use std::env;
use std::fs::File;
use std::io::{Result, Lines, BufRead, BufReader};
use std::path::Path;

// Example of how to consume the bufReader
pub fn read_file(file_path: &str) {
    if let Ok(lines) = read_lines(file_path) {    
        // Uses the iterator to return each optional line
        for line in lines {
            if let Ok(to_return) = line {
                println!("{}", to_return);
            }
        }
    }
}

pub fn read_lines(path_string: &str) -> Result<Lines<BufReader<File>>> {
    // TODO:: Must be a nicer way of handling relative file paths than this
    let current_exe = env::current_exe()
        .unwrap();
    let current_dir = current_exe.as_path()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let together = format!("{current_dir}{path_string}");

    let path = Path::new(&together);

    read(path)
}

fn read<P>(filename: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        // Return bufReader struct as it's more efficient than returning everything in memory
        Ok(BufReader::new(file).lines())
}
