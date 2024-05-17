use raylib::prelude::*;

pub fn scramble(mat: Vec<Vec<Vec<Color>>>, scramble: Vec<String>) -> Vec<Vec<Vec<Color>>> {
    let mut new_mat = mat.clone();
    for i in 0..scramble.len() {
        match scramble[i].as_str() {
            "U" => new_mat = rotate_u(new_mat),
            "U'" => new_mat = rotate_u_prime(new_mat),
            "U2" => new_mat = rotate_u2(new_mat),
            "F" => new_mat = rotate_f(new_mat),
            "F'" => new_mat = rotate_f_prime(new_mat),
            "F2" => new_mat = rotate_f2(new_mat),
            "R" => new_mat = rotate_r(new_mat),
            "R'" => new_mat = rotate_r_prime(new_mat),
            "R2" => new_mat = rotate_r2(new_mat),
            "L" => new_mat = rotate_l(new_mat),
            "L'" => new_mat = rotate_l_prime(new_mat),
            "L2" => new_mat = rotate_l2(new_mat),
            "D" => new_mat = rotate_d(new_mat),
            "D'" => new_mat = rotate_d_prime(new_mat),
            "D2" => new_mat = rotate_d2(new_mat),
            "B" => new_mat = rotate_b(new_mat),
            "B'" => new_mat = rotate_b_prime(new_mat),
            "B2" => new_mat = rotate_b2(new_mat),
            _ => (),
        }
    }
    new_mat
}

pub fn draw_cube(d: &mut RaylibDrawHandle, mat: Vec<Vec<Vec<Color>>>, x: i32, y: i32) {
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
        }
    }
}

pub fn generate_new_mat() -> Vec<Vec<Vec<Color>>> {
    let colors = vec![
        Color::WHITE,
        Color::ORANGE,
        Color::GREEN,
        Color::RED,
        Color::BLUE,
        Color::YELLOW,
    ];

    // building the color matrix
    let mut mat = vec![vec![vec![Color::BLACK; 3]; 3]; 6];
    for i in 0..6 {
        for j in 0..3 {
            for k in 0..3 {
                mat[i][j][k] = colors[i];
            }
        }
    }

    mat
}

fn rotate_face(mat: Vec<Vec<Vec<Color>>>, face: usize) -> Vec<Vec<Vec<Color>>> {
    let mut new_mat = mat.clone();

    // swap the corners of the face
    new_mat[face][0][0] = mat[face][2][0];
    new_mat[face][2][0] = mat[face][2][2];
    new_mat[face][2][2] = mat[face][0][2];
    new_mat[face][0][2] = mat[face][0][0];

    // swap the edges of the face
    new_mat[face][0][1] = mat[face][1][0];
    new_mat[face][1][0] = mat[face][2][1];
    new_mat[face][1][2] = mat[face][0][1];
    new_mat[face][2][1] = mat[face][1][2];

    return new_mat;
}

fn rotate_face_prime(mat: Vec<Vec<Vec<Color>>>, face: usize) -> Vec<Vec<Vec<Color>>> {
    let mut new_mat = mat.clone();

    // swap the corners of the face
    new_mat[face][0][0] = mat[face][0][2];
    new_mat[face][0][2] = mat[face][2][2];
    new_mat[face][2][2] = mat[face][2][0];
    new_mat[face][2][0] = mat[face][0][0];

    // swap the edges of the face
    new_mat[face][0][1] = mat[face][1][2];
    new_mat[face][1][2] = mat[face][2][1];
    new_mat[face][1][0] = mat[face][0][1];
    new_mat[face][2][1] = mat[face][1][0];

    new_mat
}

fn rotate_u(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the U face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 0);

    for i in 0..3 {
        new_mat[1][0][i] = mat[2][0][i];
        new_mat[2][0][i] = mat[3][0][i];
        new_mat[3][0][i] = mat[4][0][i];
        new_mat[4][0][i] = mat[1][0][i];
    }
    new_mat
}

fn rotate_u_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    let mut new_mat = rotate_face_prime(mat.clone(), 0);

    for i in 0..3 {
        new_mat[1][0][i] = mat[4][0][i];
        new_mat[2][0][i] = mat[1][0][i];
        new_mat[3][0][i] = mat[2][0][i];
        new_mat[4][0][i] = mat[3][0][i];
    }

    return new_mat;
}

fn rotate_u2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_u(rotate_u(mat));
}


fn rotate_f_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the green face 90 degrees counter clockwise
    let mut new_mat = rotate_face_prime(mat.clone(), 2);

    for i in 0..3 {
        new_mat[1][i][2] = mat[0][2][2-i]; // white to orange
        new_mat[5][0][i] = mat[1][i][2]; // orange to yellow
        new_mat[3][2-i][0] = mat[5][0][i]; // yellow to red
        new_mat[0][2][i] = mat[3][i][0]; // red to white
    }
    new_mat
}

fn rotate_f(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the green face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 2);

    for i in 0..3 {
        new_mat[1][2-i][2] = mat[5][0][2-i]; // yellow to orange
        new_mat[0][2][i] = mat[1][2-i][2]; // orange to white
        new_mat[3][i][0] = mat[0][2][i]; // white to red
        new_mat[5][0][i] = mat[3][2-i][0]; // red to yellow
    }

    new_mat
}

fn rotate_f2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_f(rotate_f(mat));
}

fn rotate_r(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the red face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 3);

    for i in 0..3 {
        new_mat[2][i][2] = mat[5][i][2]; // yellow to green
        new_mat[5][i][2] = mat[4][2 - i][0]; // blue to yellow
        new_mat[0][i][2] = mat[2][i][2]; // green to white
        new_mat[4][i][0] = mat[0][2 - i][2]; // white to blue
    }

    new_mat
}

fn rotate_r_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the red face 90 degrees counter clockwise
    let mut new_mat = rotate_face_prime(mat.clone(), 3);

    for i in 0..3 {
        new_mat[5][i][2] = mat[2][i][2];
        new_mat[4][2 - i][0] = mat[5][i][2];
        new_mat[0][i][2] = mat[4][2 - i][0];
        new_mat[2][i][2] = mat[0][i][2];
    }

    new_mat
}

fn rotate_r2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_r(rotate_r(mat));
}

fn rotate_l(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the orange face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 1);

    for i in 0..3 {
        new_mat[2][i][0] = mat[0][i][0]; // white to green
        new_mat[5][i][0] = mat[2][i][0]; // green to yellow
        new_mat[4][2 - i][2] = mat[5][i][0]; // yellow to blue
        new_mat[0][i][0] = mat[4][2 - i][2]; // blue to white
    }

    new_mat
}

fn rotate_l_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the orange face 90 degrees counter clockwise
    let mut new_mat = rotate_face_prime(mat.clone(), 1);

    for i in 0..3 {
        new_mat[0][i][0] = mat[2][i][0]; // green to white
        new_mat[2][i][0] = mat[5][i][0]; // yellow to green
        new_mat[5][i][0] = mat[4][2 - i][2]; // blue to yellow
        new_mat[4][2 - i][2] = mat[0][i][0]; // white to blue
    }

    new_mat
}

fn rotate_l2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_l(rotate_l(mat));
}

fn rotate_d(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the D face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 5);

    for i in 0..3 {
        new_mat[1][2][i] = mat[4][2][i];
        new_mat[4][2][i] = mat[3][2][i];
        new_mat[3][2][i] = mat[2][2][i];
        new_mat[2][2][i] = mat[1][2][i];
    }

    new_mat
}

fn rotate_d_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the D face 90 degrees counter clockwise
    let mut new_mat = rotate_face_prime(mat.clone(), 5);

    for i in 0..3 {
        new_mat[1][2][i] = mat[2][2][i];
        new_mat[4][2][i] = mat[1][2][i];
        new_mat[3][2][i] = mat[4][2][i];
        new_mat[2][2][i] = mat[3][2][i];
    }

    new_mat
}

fn rotate_d2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_d(rotate_d(mat));
}

fn rotate_b(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the blue face 90 degrees clockwise
    let mut new_mat = rotate_face(mat.clone(), 4);

    for i in 0..3 {
        new_mat[0][0][i] = mat[3][i][2];
        new_mat[1][2 - i][0] = mat[0][0][i];
        new_mat[5][2][2 - i] = mat[1][2 - i][0];
        new_mat[3][2 - i][2] = mat[5][2][i];
    }

    new_mat
}

fn rotate_b_prime(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    // rotate the blue face 90 degrees counter clockwise
    let mut new_mat = rotate_face_prime(mat.clone(), 4);

    for i in 0..3 {
        new_mat[3][i][2] = mat[0][0][i];
        new_mat[0][0][i] = mat[1][2 - i][0];
        new_mat[1][2 - i][0] = mat[5][2][2 - i];
        new_mat[5][2][i] = mat[3][2 - i][2];
    }

    new_mat
}

fn rotate_b2(mat: Vec<Vec<Vec<Color>>>) -> Vec<Vec<Vec<Color>>> {
    return rotate_b(rotate_b(mat));
}
