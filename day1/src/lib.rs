#![allow(dead_code)]
// file system libraries
use std::fs::File;
// io and buffer reader libraries
use std::io::{self, BufRead};
// file path libraries
use std::path::Path;
// use the line buffer to parse each digit
fn parse_num_digits() -> i32 {
    // file example.txt must exist in current path
    println!("reading from file");
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                // create a string that can hold the two digits
                let mut num_string = String::with_capacity(2);
                for char in l.chars() {
                    if char.is_digit(10) {
                        num_string.insert(0, char);
                        break;
                    }
                }
                for char in l.chars().rev() {
                    if char.is_digit(10) {
                        num_string.insert(1, char);
                        break;
                    }
                }
                let num = num_string.parse::<i32>().unwrap();
                println!("Number parsed from line: {}", num);
                sum += num;
            }
        }
    }
    println!("Sum of digits found: {}", sum);
    sum
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // if open is okay, create a new filehanlder called `file` to store the result
    let file: File = File::open(filename)?;
    // construct buffer reader from opened file
    Ok(io::BufReader::new(file).lines())
}
