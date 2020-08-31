mod engine;
use crate::engine::game::*;

fn main() {
    println!("2048 - The tile combination game.");

    let game: Game = Game::new(2).unwrap();

    println!("Score: {}", game.get_score());
}
