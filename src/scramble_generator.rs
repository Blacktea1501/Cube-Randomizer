use rand;

pub fn generate() -> String {
    let mut moves: Vec<&str> = vec!["F", "U", "R", "L", "B", "D"];
    let post_fix: Vec<&str> = vec!["'", "2", ""];
    let number_of_moves = 15;
    let mut scramble = String::new();

    for _ in 0..number_of_moves {
        let m = moves[rand::random::<usize>() % moves.len()];

        match m {
            "U" => {
                moves.retain(|&x| x != "U");
                for x in &["F", "R", "L", "B"] {
                    moves.push(x);
                }
            }
            "F" => {
                moves.retain(|&x| x != "F");
                for x in &["U", "R", "L", "D"] {
                    moves.push(x);
                }
            }
            "R" => {
                moves.retain(|&x| x != "R");
                for x in &["U", "F", "B", "D"] {
                    moves.push(x);
                }
            }
            "D" => {
                moves.retain(|&x| x != "D");
                for x in &["F", "R", "L", "B"] {
                    moves.push(x);
                }
            }
            "B" => {
                moves.retain(|&x| x != "B");
                for x in &["U", "R", "L", "D"] {
                    moves.push(x);
                }
            }
            "L" => {
                moves.retain(|&x| x != "L");
                for x in &["U", "F", "B", "D"] {
                    moves.push(x);
                }
            }
            _ => {}
        }

        scramble.push_str(m);
        scramble.push_str(post_fix[rand::random::<usize>() % post_fix.len()]);
        scramble.push(' ');
    }

    scramble
}
