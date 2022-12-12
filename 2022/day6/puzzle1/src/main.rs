use std::fs;

fn main() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut previous = String::new();
    let mut marker = 0;

    for char in buffer.chars() {
        marker += 1;

        if let Some(i) = previous.find(char) {
            previous = previous[i + 1..].to_string() + &char.to_string();
            continue;
        }

        previous.push(char);
        if previous.len() == 4 { break; }
    }

    println!("The first marker is after character: {}", marker);
}
