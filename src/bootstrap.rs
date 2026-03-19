use macroquad::prelude::*;

use crate::app::{App, AppState};

pub async fn run() {
    rand::srand(miniquad::date::now() as u64);

    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    loop {
        let dt = get_frame_time().min(0.1);

        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            app.update(dt);
            app.draw();
        }))
        .unwrap_or_else(|_| {
            eprintln!("Panic in game loop");
            std::process::exit(1);
        });

        if let AppState::Exiting = app.state {
            app.save.save();
            break;
        }

        next_frame().await
    }
}
