use std::collections::HashMap;
use std::fs;

struct Number {
    index: usize,
    value: i32,
}

fn main() {
    let document = fs::read_to_string("input.txt").unwrap();
    let mut numbers = HashMap::new();
    let mut sum = 0;

    numbers.insert("1", 1);
    numbers.insert("2", 2);
    numbers.insert("3", 3);
    numbers.insert("4", 4);
    numbers.insert("5", 5);
    numbers.insert("6", 6);
    numbers.insert("7", 7);
    numbers.insert("8", 8);
    numbers.insert("9", 9);

    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);

    for line in document.lines() {
        let mut first_digits = vec![];
        let mut last_digits = vec![];

        for (number, value) in &numbers {
            match line.find(number) {
                Some(index) => first_digits.push(Number { index, value: *value }),
                None => continue,
            }
        }

        for (number, value) in &numbers {
            match line.rfind(number) {
                Some(index) => last_digits.push(Number { index, value: *value }),
                None => continue,
            }
        }

        first_digits.sort_by(|a, b| a.index.cmp(&b.index));
        last_digits.sort_by(|a, b| b.index.cmp(&a.index));

        sum += first_digits.first().unwrap().value * 10 + last_digits.first().unwrap().value;
    }

    println!("The sum of all the numbers in the document is {}", sum);
}
