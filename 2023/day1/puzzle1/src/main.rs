use std::fs;

fn main() {
    let document = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    for line in document.lines() {
        let mut values = vec![];

        for character in line.chars() {
            if character.is_ascii_digit() {
                values.push(character.to_digit(10).unwrap());
            }
        }

        sum += values.first().unwrap() * 10 + values.last().unwrap();
    }

    println!("The sum of all the numbers in the document is {}", sum);
}
