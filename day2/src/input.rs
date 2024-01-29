#![allow(dead_code)]
enum Color {
    BLUE,
    RED,
    GREEN,
}

// each round a given ammount of red, blue, green, cubes are revealed
#[derive(Debug, PartialEq)]
struct Round {
    green_count: u32,
    red_count: u32,
    blue_count: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    // game is just a series of rounds, stored in this vector
    rounds: Vec<Round>,
}

// implement methods for Round
impl Round {
    fn is_possible(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        self.red_count <= red_max && self.blue_count <= blue_max && self.green_count <= green_max

        // checks if struct against the maximum found ammount of red, blue, and green cubes
        // that could be created
    }
}
// implement methods for the game
impl Game {
    fn game_is_possible(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        // iterate over collection of rounds and determine if each are possible
        self.rounds
            .iter()
            .all(|round| round.is_possible(red_max, green_max, blue_max))
        // if one round is not possible, all will return false and stop processing the set
    }
}
