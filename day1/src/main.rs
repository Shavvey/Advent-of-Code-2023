#![allow(dead_code)]

use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    parse_num_digits();
}

fn parse_num_digits() {
    // filename that we willl read from
    let filename = "input.txt";
    let mut num1: i32 = -1;
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
                // create character iterator using each char
                for char in ln.chars() {
                    if char.is_digit(10) {
                        match char.to_digit(10) {
                            Some(val) => {
                                num1 = val as i32;
                                break;
                            }
                            // if a value cannot be matched do nothing
                            None => (),
                        }
                    }
                    num_string.push(char);

                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
                        }
                    }
                    if num_dictionary.is_empty() {
                        // clear out num string
                        num_string.clear();
                        // copy word number dictionary again
                        num_dictionary = word_num.clone();
                    }

                    if num_dictionary.len() == 1 {
                        // this is bad and doesn't work
                        match num_dictionary.iter().last() {
                            Some(dict) => {
                                // if the num_string is matched with dictionary
                                if dict.0.eq(&num_string) {
                                    // save the value in dict as the decoded number
                                    num1 = *dict.1;
                                    // break out of the iteration loop
                                    break;
                                }
                            }
                            // if dictionary cannot be matched yet do nothing
                            None => (),
                        }
                    }
                }
                println!("number parsed from line: {num1}");
            }
        }
    }
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
