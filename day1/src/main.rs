#![allow(dead_code)]

use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    println!("hello world!");
}

fn parse_num_digits() -> i32 {
    // filename that we willl read from
    let filename = "example.txt";
    let sum: i32 = 0;
    println!("Reading from the file: {filename}");
    // consume the iterator and returns (optionally) a string
    if let Ok(lines) = read_lines(filename) {
        let word_num: HashMap<&str, i32> = HashMap::from([
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        for line in lines {
            // if string is returned parse this line
            if let Ok(ln) = line {
                // print out line for debug reasons
                println!("{ln}");
                // create a mutable reference is string
                let mut num_string = String::new();
                // clone a mutable reference to dictionary
                let mut num_dictionary = word_num.clone();
                let num1: i32;
                let num2: i32;
                // create character iterator using each char
                for char in ln.chars() {
                    num_string.push(char);

                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
                        }
                        if num_dictionary.len() == 1 {}
                    }
                }
            }
        }
    }
    sum
}

// read each line using a buffer reader instead of storing the whole file as
// a string or something
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
