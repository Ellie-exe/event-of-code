use std::fs;

fn main() {
    let inventory = fs::read_to_string("input.txt").unwrap();

    let mut calories: Vec<i32> = Vec::new();
    let mut calorie_sum = 0;

    for line in inventory.lines() {
        if !line.is_empty() {
            calorie_sum += line.parse::<i32>().unwrap();
            continue;
        }

        calories.push(calorie_sum);
        calorie_sum = 0;
    }

    calories.sort();
    let mut total_calories = 0;

    for _ in 0..3 {
        total_calories += calories.pop().unwrap();
    }

    println!("The elves with the most calories have a combined {} calories", total_calories);
}
