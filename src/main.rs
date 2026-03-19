mod app;
mod bootstrap;
mod game;
mod scenes;
mod systems;

#[macroquad::main("Matrix Crushed!")]
async fn main() {
    bootstrap::run().await;
}
