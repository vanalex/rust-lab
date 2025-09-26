use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("non-existing.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("The file was not found.");
                    return;
                }
                _ => {
                    println!("An error occurred: {:?}", error);
                    return;
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
