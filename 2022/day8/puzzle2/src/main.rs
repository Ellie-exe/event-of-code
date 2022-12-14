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

    let mut best_scenic_score = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            let mut left_score = 0;
            let mut right_score = 0;
            let mut top_score = 0;
            let mut bottom_score = 0;

            for other_height in row[0..c].iter().rev() {
                left_score += 1;
                if *other_height >= *height { break; }
            }

            for other_height in &row[c + 1..] {
                right_score += 1;
                if *other_height >= *height { break; }
            }

            for other_row in grid[0..r].iter().rev() {
                top_score += 1;
                if other_row[c] >= *height { break; }
            }

            for other_row in &grid[r + 1..] {
                bottom_score += 1;
                if other_row[c] >= *height { break; }
            }

            let scenic_score = left_score * right_score * top_score * bottom_score;
            if scenic_score > best_scenic_score { best_scenic_score = scenic_score; }
        }
    }

    println!("The best tree has a scenic score of {}", best_scenic_score);
}
