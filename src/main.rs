mod engine;
use crate::engine::game::*;

fn main() {
    println!("Hello, world!");

    let game: Game = Game::new(2).unwrap();

    println!("Score: {}",game.get_score()); 
}
