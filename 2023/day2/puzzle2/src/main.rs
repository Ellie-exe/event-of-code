use std::fs;

fn main() {
    let games = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    for game in games.lines() {
        let pulls = &game[game.find(':').unwrap() + 2..];

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for pull in pulls.split("; ") {
            for color in pull.split(", ") {
                let color_and_value = color.split(' ').collect::<Vec<&str>>();
                let value = color_and_value[0].parse::<i32>().unwrap();

                match color_and_value[1] {
                    "red" => {
                        if value > max_red {
                            max_red = value;
                        }
                    },

                    "green" => {
                        if value > max_green {
                            max_green = value;
                        }
                    },

                    "blue" => {
                        if value > max_blue {
                            max_blue = value;
                        }
                    },

                    _ => {}
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }

    println!("The sum of the matching IDs is {}", sum);
}
