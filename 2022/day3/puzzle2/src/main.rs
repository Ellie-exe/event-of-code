use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("input.txt").unwrap();
    let mut priority_sum = 0;
    let mut lines = rucksacks.lines();

    while let Some(rucksack1) = lines.next() {
        let rucksack2 = lines.next().unwrap();
        let rucksack3 = lines.next().unwrap();

        for char in rucksack1.chars() {
            if !rucksack2.contains(char) { continue; }
            if !rucksack3.contains(char) { continue; }

            priority_sum += if char.is_lowercase() { char as i32 - 96 } else { char as i32 - 38 };
            break;
        }
    }

    println!("The priority sum is {}", priority_sum);
}
