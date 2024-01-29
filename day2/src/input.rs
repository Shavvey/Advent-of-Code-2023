enum Color {
    BLUE,
    RED,
    GREEN,
}

// each round a given ammount of red, blue, green, cubes are revealed
struct Round {
    green_count: i32,
    red_count: i32,
    blue_count: i32,
}

struct Game {
    id: u32,
    // game is just a series of rounds, stored in this vector
    rounds: Vec<rounds>,
}
