use std::fs;

fn main() {
    let games = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    for game in games.lines() {
        let id = &game[5..game.find(':').unwrap()];
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

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += id.parse::<i32>().unwrap();
        }
    }

    println!("The sum of the matching IDs is {}", sum);
}
