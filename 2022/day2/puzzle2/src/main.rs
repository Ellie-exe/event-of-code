use std::fs;

fn main() {
    let guide = fs::read_to_string("input.txt").unwrap();
    let mut total_score = 0;

    for round in guide.lines() {
        let (opponent_move, outcome) = round.split_once(' ').unwrap();
        let mut score;

        match outcome {
            "X" => {
                score = 0;

                match opponent_move {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => panic!("Invalid move"),
                }
            },

            "Y" => {
                score = 3;

                match opponent_move {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => panic!("Invalid move"),
                }
            },

            "Z" => {
                score = 6;

                match opponent_move {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => panic!("Invalid move"),
                }
            },

            _ => panic!("Invalid move"),
        }

        total_score += score;
    }

    println!("Total score: {}", total_score);
}
