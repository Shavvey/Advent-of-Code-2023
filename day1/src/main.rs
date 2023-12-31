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
    // file name that we will read from
    let filename = "input.txt";
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    println!("Reading from the file: {filename}");
    // consume the iterator and returns (optionally) a string
    if let Ok(lines) = read_lines(filename) {
        // this hash map is very costly and not a good approach for this problem
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

        let mut sum: i32 = 0;
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
                    //println!("current char: {char}");
                    num_string.push(char);
                    // print out the current 'num_string' for testing purposes
                    println!("current num_string: {num_string}");

                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
                        }
                    }
                    if num_dictionary.len() == 1 {
                        // this is bad and doesn't work
                        match num_dictionary.iter().last() {
                            Some(val) => {
                                if val.0.eq(&num_string) {
                                    // assing value
                                    num1 = *val.1;
                                    // break from loop
                                    break;
                                }
                            }
                            None => (),
                            // if no case if match simply do nothing
                        }
                    }

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
                    if num_dictionary.is_empty() {
                        // clear out num string
                        num_string.clear();
                        // 'num_string' is just now the last character
                        num_string.push(char);
                        // copy word number dictionary again
                        num_dictionary = word_num.clone();
                    }
                }

                println!("first number parsed from line: {num1}");
                num_string.clear();
                num_dictionary = word_num.clone();
                // doing the same thing but now it is reversed
                for char in ln.chars().rev() {
                    println!("current char: {char}");
                    num_string.insert(0, char);
                    println!("current num_string: {num_string}");
                    // updated num_string from iterator char and immediately update dictionary
                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
                        }
                    }

                    if num_dictionary.len() == 1 {
                        // this is bad and doesn't work
                        match num_dictionary.iter().last() {
                            Some(val) => {
                                if val.0.eq(&num_string) {
                                    // assing value
                                    num2 = *val.1;
                                    // break from loop
                                    break;
                                }
                            }
                            None => (),
                            // if no case if match simply do nothing
                        }
                    }
                    if char.is_digit(10) {
                        match char.to_digit(10) {
                            Some(val) => {
                                num2 = val as i32;
                                break;
                            }
                            // if a value cannot be matched do nothing
                            None => (),
                        }
                    }

                    // check if dictionary is current empty
                    if num_dictionary.is_empty() {
                        // clear out num string
                        num_string.clear();
                        num_string.insert(0, char);
                        // copy word number dictionary again
                        num_dictionary = word_num.clone();
                    }
                }
                println!("second number parsed from line: {num2}");
            }
            // sum together to two values found
            sum += (num1 * 10) + num2;
        }
        println!("total sum is: {sum}");
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
