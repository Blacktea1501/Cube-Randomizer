mod scramble_generator;
mod scrambler;

use raylib::prelude::*;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();

    // set the target FPS
    rl.set_target_fps(60);
    let font = rl.load_font(&thread, "./src/font/JetBrainsMonoNerdFontMono-Bold.ttf").unwrap();
    
    // initial scramble
    let mut mat = scrambler::generate_new_mat();
    let mut scramble = scramble_generator::generate();
    let s_vec: Vec<String> = scramble.split(" ").map(|s| s.to_string()).collect();
    mat = scrambler::scramble(mat.clone(), s_vec);

    // main loop
    while !rl.window_should_close() {

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            scramble = scramble_generator::generate();
            mat = scrambler::generate_new_mat();
            let s_vec: Vec<String> = scramble.split(" ").map(|s| s.to_string()).collect();
            mat = scrambler::scramble(mat.clone(), s_vec);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text_ex( &font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex( &font, &scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);

        scrambler::draw_cube(&mut d, mat.clone(), 0, 0);

        d.draw_fps(720, 420); 
    }
}
