use rand;

pub fn generate() -> String {
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

    scramble
}
