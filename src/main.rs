mod scramble_generator;
mod scrambler;

use raylib::prelude::*;
use std::time::Instant;
fn main() {

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();

    // set the target FPS
    //rl.set_target_fps(60);
    let font = rl.load_font(&thread, "./src/font/JetBrainsMonoNerdFontMono-Bold.ttf").unwrap();
    
    // initial scramble
    let mut scramble = scramble_generator::generate();
    let mut mat = scrambler::scramble(&scramble);


    let mut start: bool = false;
    let mut timer: Instant = Instant::now();

    // main loop
    while !rl.window_should_close() {

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            scramble = scramble_generator::generate();
            mat = scrambler::scramble(&scramble);
        }
        
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_ENTER) {
            start = true;
            timer = Instant::now();
        }
        

        let mut d = rl.begin_drawing(&thread);
    
        d.clear_background(Color::BLACK);
        d.draw_text_ex( &font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex( &font, &scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);

        if start {
            let elapsed = timer.elapsed().as_secs();
            if elapsed < 15{
                let countdown = 15 - elapsed;
                d.draw_text_ex( &font, &countdown.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::WHITE);
            } else {
                start = false;
            }
        }

        scrambler::draw_cube(&mut d, mat.clone(), 0, 0);

        d.draw_fps(720, 420); 
    }
}

