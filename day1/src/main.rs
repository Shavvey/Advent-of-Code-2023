#![allow(dead_code)]

use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // parse each string in textfile to produce the two digit of each line
    parse_num_digits();
}
// parse the digits inside the textfile
fn parse_num_digits() {
    // file name that we will read from
    let filename = "example.txt";
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    //println!("Reading from the file: {filename}");
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
                    while num_dictionary.is_empty() {
                        num_string.remove(0);
                        num_dictionary = word_num.clone();
                        for key in word_num.keys() {
                            if !key.contains(&num_string) {
                                num_dictionary.remove(key);
                            }
                        }
                    }

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

                    num_string.push(char);
                    // print out the current 'num_string' for testing purposes
                    println!("current num_string: {num_string}");
                }

                println!("first number parsed from line: {num1}");
                num_string.clear();
                num_dictionary = word_num.clone();
                // doing the same thing but now it is reversed
                for char in ln.chars().rev() {
                    while num_dictionary.is_empty() {
                        num_string.pop();
                        num_dictionary = word_num.clone();
                        for key in word_num.keys() {
                            if !key.contains(&num_string) {
                                num_dictionary.remove(key);
                            }
                        }
                    }

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

                    num_string.insert(0, char);
                    // print out the current 'num_string' for testing purposes
                    println!("current num_string: {num_string}");
                }
                println!("second number parsed from line: {num2}");
            }
            // sum together to two values found
            let decoded = num1 * 10 + num2;
            sum += decoded;
            //println!("decoded number: {decoded}");
            //println!("working sum is: {sum}");
        }
        // total sum using the parsed number
        println!("total sum is: {sum}");
    }
}

// read each line using a buffer reader instead of storing the whole file as
// a string or something
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    // P is a generic that represents a reference to a file path
    P: AsRef<Path>,
{
    // open the file
    let file = File::open(filename)?;
    // if file open succesfully, open a buffer reader of the textfile
    Ok(io::BufReader::new(file).lines())
}
