use std::fs;
use std::time::Duration;
use std::thread::sleep;

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut width = 5;
    let mut height = 5;

    let mut grid = vec![vec!['.'; width]; height];

    let mut start = Knot { x: width as i32 / 2, y: height as i32 / 2 };
    let mut head = Knot { x: width as i32 / 2, y: height as i32 / 2 };
    let mut tails = vec![Knot { x: width as i32 / 2, y: height as i32 / 2 }; 9];

    let mut num_visited = 0;

    println!("\x1b[1m");

    for step in input.lines() {
        let (dir, dist) = step.split_once(' ').unwrap();

        for _ in 0..width - 2 {
            print!("-");
        }

        print!(" {}{} ", dir, dist);

        for _ in 0..width - 2 {
            print!("-");
        }

        println!("\n");

        for _ in 0..dist.parse::<u8>().unwrap() {
            match dir {
                "U" => {
                    if head.y == 0 {
                        grid.insert(0, vec!['.'; width]);

                        height += 1;
                        head.y += 1;
                        start.y += 1;

                        for tail in &mut tails {
                            tail.y += 1;
                        }
                    }

                    head.y -= 1;

                    let mut temp_head = head.clone();

                    for tail in &mut tails {
                        match (temp_head.y - tail.y, temp_head.x - tail.x) {
                            (-2, 0) => { tail.y -= 1; },
                            (2, 0) => { tail.y += 1; },
                            (0, -2) => { tail.x -= 1; },
                            (0, 2) => { tail.x += 1; },

                            (-2, -1) | (-1, -2) | (-2, -2) => {
                                tail.y -= 1;
                                tail.x -= 1;
                            },

                            (-2, 1) | (-1, 2) | (-2, 2) => {
                                tail.y -= 1;
                                tail.x += 1;
                            },

                            (2, -1) | (1, -2) | (2, -2) => {
                                tail.y += 1;
                                tail.x -= 1;
                            },

                            (2, 1) | (1, 2) | (2, 2) => {
                                tail.y += 1;
                                tail.x += 1;
                            },

                            _ => {}
                        }

                        temp_head = tail.clone();
                    }
                },

                "D" => {
                    if head.y == height as i32 - 1 {
                        grid.push(vec!['.'; width]);
                        height += 1;
                    }

                    head.y += 1;

                    let mut temp_head = head.clone();

                    for tail in &mut tails {
                        match (temp_head.y - tail.y, temp_head.x - tail.x) {
                            (-2, 0) => { tail.y -= 1; },
                            (2, 0) => { tail.y += 1; },
                            (0, -2) => { tail.x -= 1; },
                            (0, 2) => { tail.x += 1; },

                            (-2, -1) | (-1, -2) | (-2, -2) => {
                                tail.y -= 1;
                                tail.x -= 1;
                            },

                            (-2, 1) | (-1, 2) | (-2, 2) => {
                                tail.y -= 1;
                                tail.x += 1;
                            },

                            (2, -1) | (1, -2) | (2, -2) => {
                                tail.y += 1;
                                tail.x -= 1;
                            },

                            (2, 1) | (1, 2) | (2, 2) => {
                                tail.y += 1;
                                tail.x += 1;
                            },

                            _ => {}
                        }

                        temp_head = tail.clone();
                    }
                },

                "L" => {
                    if head.x == 0 {
                        for row in &mut grid {
                            row.insert(0, '.');
                        }

                        width += 1;
                        head.x += 1;
                        start.x += 1;

                        for tail in &mut tails {
                            tail.x += 1;
                        }
                    }

                    head.x -= 1;

                    let mut temp_head = head.clone();

                    for tail in &mut tails {
                        match (temp_head.y - tail.y, temp_head.x - tail.x) {
                            (-2, 0) => { tail.y -= 1; },
                            (2, 0) => { tail.y += 1; },
                            (0, -2) => { tail.x -= 1; },
                            (0, 2) => { tail.x += 1; },

                            (-2, -1) | (-1, -2) | (-2, -2) => {
                                tail.y -= 1;
                                tail.x -= 1;
                            },

                            (-2, 1) | (-1, 2) | (-2, 2) => {
                                tail.y -= 1;
                                tail.x += 1;
                            },

                            (2, -1) | (1, -2) | (2, -2) => {
                                tail.y += 1;
                                tail.x -= 1;
                            },

                            (2, 1) | (1, 2) | (2, 2) => {
                                tail.y += 1;
                                tail.x += 1;
                            },

                            _ => {}
                        }

                        temp_head = tail.clone();
                    }
                },

                "R" => {
                    if head.x == width as i32 - 1 {
                        for row in &mut grid {
                            row.push('.');
                        }

                        width += 1;
                    }

                    head.x += 1;

                    let mut temp_head = head.clone();

                    for tail in &mut tails {
                        match (temp_head.y - tail.y, temp_head.x - tail.x) {
                            (-2, 0) => { tail.y -= 1; },
                            (2, 0) => { tail.y += 1; },
                            (0, -2) => { tail.x -= 1; },
                            (0, 2) => { tail.x += 1; },

                            (-2, -1) | (-1, -2) | (-2, -2) => {
                                tail.y -= 1;
                                tail.x -= 1;
                            },

                            (-2, 1) | (-1, 2) | (-2, 2) => {
                                tail.y -= 1;
                                tail.x += 1;
                            },

                            (2, -1) | (1, -2) | (2, -2) => {
                                tail.y += 1;
                                tail.x -= 1;
                            },

                            (2, 1) | (1, 2) | (2, 2) => {
                                tail.y += 1;
                                tail.x += 1;
                            },

                            _ => {}
                        }

                        temp_head = tail.clone();
                    }
                },

                _ => {}
            }

            let last_tail = tails.last().unwrap();

            if grid[last_tail.y as usize][last_tail.x as usize] != '#' {
                grid[last_tail.y as usize][last_tail.x as usize] = '#';
                num_visited += 1;
            }

            for (r, row) in grid.iter().enumerate() {
                'a: for (c, char) in row.iter().enumerate() {
                    if r == head.y as usize && c == head.x as usize {
                        print!(" \x1b[38;2;161;116;79mH\x1b[39m");
                        continue;
                    }

                    for (i, tail) in tails.iter().enumerate() {
                        if r == tail.y as usize && c == tail.x as usize {
                            print!(" \x1b[38;2;161;116;79m{}\x1b[39m", i + 1);
                            continue 'a;
                        }
                    }

                    if r == start.y as usize && c == start.x as usize {
                        print!(" \x1b[38;2;40;143;106ms\x1b[39m");

                    } else {
                        print!(" \x1b[38;2;46;123;216m{}\x1b[39m", *char);
                    }
                }

                println!();
            }

            println!();

            sleep(Duration::from_millis(500));
        }
    }

    println!("Done! Visited {} positions", num_visited);
}
