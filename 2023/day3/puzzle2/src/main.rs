use std::fs;

#[derive(Copy, Clone)]
struct Star {
    gear_ratio: i32,
    complete: bool,
}

fn main() {
    let schematic = fs::read_to_string("input.txt").unwrap();
    let lines = schematic.lines().collect::<Vec<&str>>();

    let mut stars = [[Star { gear_ratio: 0, complete: false }; 140]; 140];
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut num_to_skip = 0;

        'a: for (x, char) in chars.iter().enumerate() {
            if num_to_skip > 0 {
                num_to_skip -= 1;
                continue;
            }

            if !char.is_ascii_digit() {
                continue;
            }

            let mut num_string = char.to_string();
            let mut next_char = x + 1;

            while next_char < line.len() && line.chars().nth(next_char).unwrap().is_ascii_digit() {
                num_string.push(line.chars().nth(next_char).unwrap());
                next_char += 1;
            }

            let num = num_string.parse::<i32>().unwrap();
            num_to_skip = num_string.len() - 1;

            let num_start = x;
            let num_end = next_char - 1;

            if y > 0 {
                let above_line_start = num_start - if num_start > 0 { 1 } else { 0 };
                let above_line_end = num_end + if num_end < line.len() - 1 { 1 } else { 0 };
                let above_line = &lines[y - 1][above_line_start..above_line_end + 1];

                for (count, above_char) in above_line.chars().enumerate() {
                    if above_char == '*' {
                        let star = &mut stars[y - 1][above_line_start + count];

                        if !star.complete && star.gear_ratio == 0 {
                            star.gear_ratio = num;

                        } else {
                            star.gear_ratio *= num;
                            star.complete = true;
                        }

                        continue 'a;
                    }
                }
            }

            if num_start > 0 && chars[num_start - 1] == '*' {
                let star = &mut stars[y][num_start - 1];

                if !star.complete && star.gear_ratio == 0 {
                    star.gear_ratio = num;

                } else {
                    star.gear_ratio *= num;
                    star.complete = true;
                }

                continue;
            }

            if num_end < line.len() - 1 && chars[num_end + 1] == '*' {
                let star = &mut stars[y][num_end + 1];

                if !star.complete && star.gear_ratio == 0 {
                    star.gear_ratio = num;

                } else {
                    star.gear_ratio *= num;
                    star.complete = true;
                }

                continue;
            }

            if y < lines.len() - 1 {
                let below_line_start = num_start - if num_start > 0 { 1 } else { 0 };
                let below_line_end = num_end + if num_end < line.len() - 1 { 1 } else { 0 };
                let below_line = &lines[y + 1][below_line_start..below_line_end + 1];

                for (count, below_char) in below_line.chars().enumerate() {
                    if below_char == '*' {
                        let star = &mut stars[y + 1][below_line_start + count];

                        if !star.complete && star.gear_ratio == 0 {
                            star.gear_ratio = num;

                        } else {
                            star.gear_ratio *= num;
                            star.complete = true;
                        }

                        continue 'a;
                    }
                }
            }
        }
    }

    for row in stars {
        for star in row {
            if star.complete && star.gear_ratio != 0 {
                sum += star.gear_ratio;
            }
        }
    }

    println!("The sum of all the gear ratios is {}", sum);
}
