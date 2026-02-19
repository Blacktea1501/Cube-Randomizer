mod scramble_generator;
mod scrambler;
mod history;
mod app;

use raylib::prelude::*;
use app::App;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();

    let mut app = App::new(&mut rl, &thread);

    // main loop
    while !rl.window_should_close() {
        
        app.update(&rl);

        let mut d = rl.begin_drawing(&thread);
        
        app.draw(&mut d);
    }
}


