mod game;

use game::world::*;
use macroquad::{miniquad::window::quit, prelude::*};

#[macroquad::main("2Dgame")]
async fn main() {
    // set_window_size(1920, 1080);
    set_fullscreen(true);
    show_mouse(false);
    set_cursor_grab(true);
    next_frame().await;
    next_frame().await;
    let mut main_world = World::new().await;
    loop {
        main_world.update();
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
        let delta = get_frame_time();
        draw_text(
            &format!("FPS: {}", 1.0 / delta,),
            5.0,
            20.0,
            30.0,
            Color {
                r: 0.0,
                g: 255.0,
                b: 0.0,
                a: 255.0,
            },
        );
        next_frame().await;
    }
}
