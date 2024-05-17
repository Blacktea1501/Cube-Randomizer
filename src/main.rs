mod scramble_generator;
mod scrambler;

use raylib::prelude::*;

fn draw_cube(d: &mut RaylibDrawHandle, mat: Vec<Vec<Vec<Color>>>, x: i32, y: i32) {
    draw_face(d, mat[0].clone(), 150 + x, 100 + y); // white 0
    draw_face(d, mat[1].clone(), 60  + x, 190 + y); // orange 1
    draw_face(d, mat[2].clone(), 150 + x, 190 + y); // green 2
    draw_face(d, mat[3].clone(), 240 + x, 190 + y); // red 3 
    draw_face(d, mat[4].clone(), 330 + x, 190 + y); // blue 4
    draw_face(d, mat[5].clone(), 150 + x, 280 + y); // yellow 5
}

fn draw_face(d: &mut RaylibDrawHandle, mat: Vec<Vec<Color>>, x: i32, y: i32) {
    for i in 0..3 {
        for j in 0..3 {
            d.draw_rectangle(x + i * 30, y + j * 30, 25, 25, mat[j as usize][i as usize]);
            // draw numbers on the rectangles
            d.draw_text_ex(&d.get_font_default(), &format!("{},{}", i, j), Vector2::new(x as f32 + i as f32 * 30.0, y as f32 + j as f32 * 30.0), 10.0, 0.0, Color::BLACK);
        }
    }
}

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Scramble Generator")
        .build();


    // set the target FPS
    rl.set_target_fps(60);



    // loading the font
    let font = rl.load_font(&thread, "./src/font/JetBrainsMonoNerdFontMono-Bold.ttf").unwrap();
    
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


        draw_cube(&mut d, mat.clone(), 0, 0);

        d.draw_fps(720, 420); 
    }
}
