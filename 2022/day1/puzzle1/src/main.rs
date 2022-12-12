use std::fs;

fn main() {
    let inventory = fs::read_to_string("input.txt").unwrap();

    let mut total_calories: Vec<i32> = Vec::new();
    let mut calories = 0;

    for line in inventory.lines() {
        if !line.is_empty() {
            calories += line.parse::<i32>().unwrap();
            continue;
        }

        total_calories.push(calories);
        calories = 0;
    }

    total_calories.sort();
    println!("The elf with the most calories has {} calories", total_calories.last().unwrap());
}
