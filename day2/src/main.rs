fn main() {}

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
