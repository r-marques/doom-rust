use ggez::GameResult;

use doom_rust::game::Game;

fn main() -> GameResult {
    let mut game = Game::new("doom1.wad");

    game.run()
}
