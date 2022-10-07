mod constants;
mod domain;
mod models;
mod utils;

use crate::domain::GameProcessor;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut pr: GameProcessor = GameProcessor::new();
    pr.run().await;
}
