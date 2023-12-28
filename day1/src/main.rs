use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // file example.txt must exist in current path
    if let Ok(lines) = read_lines("./example.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                // create a string that can hold the two digits
                let mut num_string = String::with_capacity(2);
                for char in l.chars() {
                    if char.is_numeric() {
                        num_string.insert(0, char);
                    }
                }
                for char in l.chars().rev() {
                    if char.is_numeric() {
                        num_string.insert(1, char);
                    }
                }
                let num = num_string.parse::<i32>().unwrap();
                println!("Number parsed from line: {}", num);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
