use std::fs;

fn main() {
    let guide = fs::read_to_string("input.txt").unwrap();
    let mut total_score = 0;

    for round in guide.lines() {
        let (opponent_move, my_move) = round.split_once(' ').unwrap();
        let mut score;

        match my_move {
            "X" => {
                score = 1;

                match opponent_move {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => panic!("Invalid move"),
                }
            },

            "Y" => {
                score = 2;

                match opponent_move {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => panic!("Invalid move"),
                }
            },

            "Z" => {
                score = 3;

                match opponent_move {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => panic!("Invalid move"),
                }
            },

            _ => panic!("Invalid move"),
        }

        total_score += score;
    }

    println!("Total score: {}", total_score);
}
