#![allow(dead_code)]

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
// enum type that represents the possible color the cubes may have
enum Color {
    Blue,
    Green,
    Red,
}

struct Cubes {
    color: Color,
    quantity: u32,
}

struct Game {
    // array that contains cubes revealed in each round
    round_one: [Cubes; 3],
    round_two: [Cubes; 3],
    // how many red, blue, and green cubes are revealed
}
// function to open a file and start reading the lines of it
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    // construct buffer reader from opened file
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Hello, World!");
    // enum type can be declared something like this
    let filename = "parse.txt";
    // get the lines of the textfile we need
    if let Ok(lines) = read_lines(filename) {}
}
