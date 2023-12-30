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
    let filename = "example.txt";
    let mut num1: i32 = -1;
    let mut num2: i32 = -1;
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
                    println!("current char: {char}");
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

                    // update num string
                    num_string.push(char);
                    // print out the current 'num_string' for testing purposes

                    if num_dictionary.len() == 1 {
                        // this is bad and doesn't work
                        for (key, value) in num_dictionary.iter() {
                            // if the num_string is matched with dictionary
                            if key.eq(&num_string) {
                                num_string.clear();
                                // save the value in dict as the decoded number
                                num1 = *value;
                                // break out of the iteration loop
                                num_dictionary = word_num.clone();
                                break;
                            }
                            // if dictionary cannot be matched yet do nothing
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
                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
                        }
                    }
                }

                println!("first number parsed from line: {num1}");
                num_string.clear();
                num_dictionary = word_num.clone();
                // doing the same thing but now it is reversed
                for char in ln.chars().rev() {
                    println!("current char: {char}");
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
                    // insert char at the beginning of the string
                    num_string.insert(0, char);

                    println!("current num_string: {num_string}");
                    for key in word_num.keys() {
                        if !key.contains(&num_string) {
                            num_dictionary.remove(key);
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
                    // handle the dictionary part
                    if num_dictionary.len() == 1 {
                        // this is bad and doesn't work
                        for (key, val) in num_dictionary.iter() {
                            if key.eq(&num_string) {
                                num_string.clear();
                                // assign the number to given value
                                num2 = *val;
                                // consume the last term the dictionary
                                num_dictionary = word_num.clone();
                                // break from current iteration
                                break;
                            }
                        }
                    }
                }
                println!("second number parsed from line {num2}");
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
