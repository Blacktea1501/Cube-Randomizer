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


    // inspection timer
    let mut start: bool = false;
    let mut timer: Instant = Instant::now();
    let mut greentimer: Instant = Instant::now();
    let mut timer_started = false;
    let mut was_green = false;

    // stopwatch
    let mut is_stopwatch: bool = false;
    let mut stopwatch_timer: Instant = Instant::now();
    let mut stopwatch_time = 0.0;

    // main loop
    while !rl.window_should_close() {

        // check if the 
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
            scramble = scramble_generator::generate();
            mat = scrambler::scramble(&scramble);
        }
        
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) && !start && !is_stopwatch{
            start = true;
            timer = Instant::now();
            timer_started = true;
        }
        

        let mut d = rl.begin_drawing(&thread);
    
        d.clear_background(Color::BLACK);
        d.draw_text_ex( &font, "Scramble Generator", Vector2::new(10.0, 10.0), 20.0, 0.0, Color::WHITE);
        d.draw_text_ex( &font, &scramble, Vector2::new(10.0, 30.0), 20.0, 0.0, Color::WHITE);

        if start {
            let elapsed = timer.elapsed().as_secs();

            if elapsed >= 15{
                start = false;
                timer_started = false;
            } 

            let countdown = 15 - elapsed;

            // if space is hold change the color of the text to red else it's white and if the
            // space bar is hold for more then one second the text will go green
            if d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
                if timer_started == true{
                    greentimer = Instant::now();
                    timer_started = false;
                }
                if greentimer.elapsed().as_secs() >= 1 {
                    d.draw_text_ex( &font, &countdown.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::GREEN);
                    was_green = true;
                } else {
                    d.draw_text_ex( &font, &countdown.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::RED);
                }
            } else {
                d.draw_text_ex( &font, &countdown.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::WHITE);
            }
           
           // if the spacebar is released and the text was green then the timer will stop
           if d.is_key_up(raylib::consts::KeyboardKey::KEY_SPACE) && was_green {
               start = false;
               timer_started = false;
               was_green = false;
               is_stopwatch = true;
               stopwatch_timer = Instant::now();
           }
           
           // this will reset the timer if the spacebar is released and the text was not green
           if d.is_key_up(raylib::consts::KeyboardKey::KEY_SPACE) && !was_green {
               timer_started = true;
           }
        }
        
        // if the spacebar hits it should stop the watch and display the time
        if is_stopwatch {
            let elapsed = stopwatch_timer.elapsed().as_millis() as f64 / 1000.0;
            d.draw_text_ex( &font, &elapsed.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::WHITE);
            if d.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
                is_stopwatch = false;
                stopwatch_time = elapsed;
            }
        }
        
        if !is_stopwatch && start == false{
            d.draw_text_ex( &font, &stopwatch_time.to_string(), Vector2::new(600.0, 225.0), 50.0, 0.0, Color::WHITE);
        }

        scrambler::draw_cube(&mut d, mat.clone(), 0, 0);

        d.draw_fps(720, 420); 
    }
}

