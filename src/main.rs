mod scramble_generator;

use raylib::prelude::*;

fn main() {
    let mut scramble = scramble_generator::generate();

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();

    // set the target FPS
    rl.set_target_fps(60);

    // loading the font
    let font = rl.load_font(&thread, "/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Bold.ttf").unwrap();
    
    // main loop
    while !rl.window_should_close() {

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            scramble = scramble_generator::generate();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex(&font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex(&font, &scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);
        
        // d.draw_fps(700, 420);
    }
}
