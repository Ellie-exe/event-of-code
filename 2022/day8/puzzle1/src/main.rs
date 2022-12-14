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

    for (r, row) in grid.iter().enumerate() {
        if r == 0 || r == grid.len() - 1 {
            num_visible += row.len();
            continue;
        }

        for (c, height) in row.iter().enumerate() {
            let mut left_visible = true;
            let mut right_visible = true;
            let mut top_visible = true;
            let mut bottom_visible = true;

            if c == 0 || c == row.len() - 1 {
                num_visible += 1;
                continue;
            }

            for other_height in &row[0..c] {
                if *other_height >= *height {
                    left_visible = false;
                    break;
                }
            }

            for other_height in &row[c + 1..] {
                if *other_height >= *height {
                    right_visible = false;
                    break;
                }
            }

            for other_row in &grid[0..r] {
                if other_row[c] >= *height {
                    top_visible = false;
                    break;
                }
            }

            for other_row in &grid[r + 1..] {
                if other_row[c] >= *height {
                    bottom_visible = false;
                    break;
                }
            }

            if left_visible || right_visible || top_visible || bottom_visible {
                num_visible += 1;
            }
        }
    }

    println!("The number of visible trees is {}", num_visible);
}
