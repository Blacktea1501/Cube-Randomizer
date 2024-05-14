// This is a rubics cube scrambler
// It generates a random scramble of 20 moves

// For each move should be a follow up move that is not the same as the previos move and does not
// cancel out the previous move (e.g. F F' or F F2)
// also the same move should not be repeated twice in a row (e.g. F F)
// F F2 should be just F'
// I need to keep track of something like this too: F B F' because that is just B

// after every F move should be only U, R, L, B, D moves
// after every U move should be only F, R, L, B, D moves
// after every R move should be only F, U, L, B, D moves
// after every L move should be only F, U, R, B, D moves
// after every B move should be only F, U, R, L, D moves
// after every D move should be only F, U, R, L, B moves

// But after a F and B move should not be a B and F move
// And after a U and D move should not be a D and U move
// And after a R and L move should not be a L and R move

use rand;

fn main() {
    let moves: Vec<&str> = vec!["F", "U", "R", "L", "B", "D"];
    let post_fix: Vec<&str> = vec!["'", "2", ""];

    let amout_of_moves = 20;
    let mut scramble = String::new();
    let mut last_move = "";
    let mut last_move2 = "";

    for _ in 0..amout_of_moves {
        let mut m = moves[rand::random::<usize>() % moves.len()];

        while m == last_move || m == last_move2 {
            m = moves[rand::random::<usize>() % moves.len()];
        }

        scramble.push_str(m);
        scramble.push_str(post_fix[rand::random::<usize>() % post_fix.len()]);
        scramble.push(' ');

        last_move2 = last_move;
        last_move = m;
    }

    println!("{}", scramble);
}
