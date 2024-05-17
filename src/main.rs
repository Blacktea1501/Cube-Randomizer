mod scramble_generator;

use raylib::prelude::*;

fn main() {
    let mut scramble = scramble_generator::generate();

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();

    // set the target FPS
    rl.set_target_fps(144);

    // loading the font
    let font = rl.load_font(&thread, "/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Bold.ttf",).unwrap();

    // main loop
    while !rl.window_should_close() {
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
            scramble = scramble_generator::generate();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text_ex( &font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex( &font, &scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);

        d.draw_rectangle(150, 100, 25, 25, Color::WHITE);
        d.draw_rectangle(150, 130, 25, 25, Color::WHITE);
        d.draw_rectangle(150, 160, 25, 25, Color::WHITE);

        d.draw_rectangle(180, 130, 25, 25, Color::WHITE);
        d.draw_rectangle(180, 100, 25, 25, Color::WHITE);
        d.draw_rectangle(180, 160, 25, 25, Color::WHITE);

        d.draw_rectangle(210, 100, 25, 25, Color::WHITE);
        d.draw_rectangle(210, 130, 25, 25, Color::WHITE);
        d.draw_rectangle(210, 160, 25, 25, Color::WHITE);

        d.draw_rectangle(150, 190, 25, 25, Color::GREEN);
        d.draw_rectangle(150, 220, 25, 25, Color::GREEN);
        d.draw_rectangle(150, 250, 25, 25, Color::GREEN);
        
        d.draw_rectangle(180, 190, 25, 25, Color::GREEN);
        d.draw_rectangle(180, 220, 25, 25, Color::GREEN);
        d.draw_rectangle(180, 250, 25, 25, Color::GREEN);

        d.draw_rectangle(210, 190, 25, 25, Color::GREEN);
        d.draw_rectangle(210, 220, 25, 25, Color::GREEN);
        d.draw_rectangle(210, 250, 25, 25, Color::GREEN);

       d.draw_rectangle(150, 280, 25, 25, Color::YELLOW); 
       d.draw_rectangle(150, 310, 25, 25, Color::YELLOW);
       d.draw_rectangle(150, 340, 25, 25, Color::YELLOW);

       d.draw_rectangle(180, 280, 25, 25, Color::YELLOW);
       d.draw_rectangle(180, 310, 25, 25, Color::YELLOW);
       d.draw_rectangle(180, 340, 25, 25, Color::YELLOW);
       
       d.draw_rectangle(210, 280, 25, 25, Color::YELLOW);
       d.draw_rectangle(210, 310, 25, 25, Color::YELLOW);
       d.draw_rectangle(210, 340, 25, 25, Color::YELLOW);

       d.draw_rectangle(120, 190, 25, 25, Color::ORANGE);
       d.draw_rectangle(120, 220, 25, 25, Color::ORANGE);
       d.draw_rectangle(120, 250, 25, 25, Color::ORANGE);

       d.draw_rectangle(90, 190, 25, 25, Color::ORANGE);
       d.draw_rectangle(90, 220, 25, 25, Color::ORANGE);
       d.draw_rectangle(90, 250, 25, 25, Color::ORANGE);

       d.draw_rectangle(60, 190, 25, 25, Color::ORANGE);
       d.draw_rectangle(60, 220, 25, 25, Color::ORANGE);
       d.draw_rectangle(60, 250, 25, 25, Color::ORANGE);

       d.draw_rectangle(240, 190, 25, 25, Color::RED);
       d.draw_rectangle(240, 220, 25, 25, Color::RED);
       d.draw_rectangle(240, 250, 25, 25, Color::RED);

       d.draw_rectangle(270, 190, 25, 25, Color::RED);
       d.draw_rectangle(270, 220, 25, 25, Color::RED);
       d.draw_rectangle(270, 250, 25, 25, Color::RED);

       d.draw_rectangle(300, 190, 25, 25, Color::RED);
       d.draw_rectangle(300, 220, 25, 25, Color::RED);
       d.draw_rectangle(300, 250, 25, 25, Color::RED);

       d.draw_rectangle(330, 190, 25, 25, Color::BLUE);
       d.draw_rectangle(330, 220, 25, 25, Color::BLUE);
       d.draw_rectangle(330, 250, 25, 25, Color::BLUE);

       d.draw_rectangle(360, 190, 25, 25, Color::BLUE);
       d.draw_rectangle(360, 220, 25, 25, Color::BLUE);
       d.draw_rectangle(360, 250, 25, 25, Color::BLUE);

       d.draw_rectangle(390, 190, 25, 25, Color::BLUE);
       d.draw_rectangle(390, 220, 25, 25, Color::BLUE);
       d.draw_rectangle(390, 250, 25, 25, Color::BLUE);

       d.draw_fps(720, 420); 
    }
}
