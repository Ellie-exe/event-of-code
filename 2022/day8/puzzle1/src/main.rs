use std::fs;

fn main() {
    let map = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<i8>> = Vec::new();

    for map_row in map.lines() {
        let mut row: Vec<i8> = Vec::new();

        for height in map_row.chars() {
            row.push(height.to_digit(10).unwrap() as i8);
        }

        grid.push(row);
    }

    let mut num_visible = 0;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            num_visible += row.len();
            continue;
        }

        for (j, height) in row.iter().enumerate() {
            let mut left_visible = true;
            let mut right_visible = true;
            let mut top_visible = true;
            let mut bottom_visible = true;

            if j == 0 || j == row.len() - 1 {
                num_visible += 1;
                continue;
            }

            for other_height in &row[0..j] {
                if *other_height >= *height {
                    left_visible = false;
                    continue;
                }
            }

            for other_height in &row[j + 1..] {
                if *other_height >= *height {
                    right_visible = false;
                    continue;
                }
            }

            for other_row in &grid[0..i] {
                if other_row[j] >= *height {
                    top_visible = false;
                    continue;
                }
            }

            for other_row in &grid[i + 1..] {
                if other_row[j] >= *height {
                    bottom_visible = false;
                    continue;
                }
            }

            if left_visible || right_visible || top_visible || bottom_visible {
                num_visible += 1;
            }
        }
    }

    println!("The number of visible trees is {}", num_visible);
}
