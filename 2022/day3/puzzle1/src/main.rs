use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("input.txt").unwrap();
    let mut priority_sum = 0;

    for rucksack in rucksacks.lines() {
        let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

        for char in compartment1.chars() {
            if !compartment2.contains(char) { continue; }
            priority_sum += if char.is_lowercase() { char as i32 - 96 } else { char as i32 - 38 };

            break;
        }
    }

    println!("The priority sum is {}", priority_sum);
}
