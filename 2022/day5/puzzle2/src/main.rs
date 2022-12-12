use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stacks = vec![vec![]; 9];
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if &line[1..2] == "1" { break; }

        for stack in 0..10 {
            let start = stack * 4;
            let end = start + 3;

            if end > line.len() { break; }
            let supply_crate = &line[start..end];

            if &supply_crate[0..1] != "[" { continue; }
            stacks[stack].insert(0, supply_crate);
        }
    }

    lines.next();

    while let Some(line) = lines.next() {
        let step: Vec<&str> = line.split(" ").collect();

        let quantity = step[1].parse::<i32>().unwrap();
        let from = step[3].parse::<usize>().unwrap() - 1;
        let to = step[5].parse::<usize>().unwrap() - 1;

        let start = stacks[from].len() - quantity as usize;
        let mut supply_crates: Vec<&str> = stacks[from].drain(start..).collect();
        stacks[to].append(&mut supply_crates);
    }

    let mut top_crates = String::new();

    for stack in stacks {
        top_crates += &stack.last().unwrap()[1..2];
    }

    println!("The top crates are {}", top_crates);
}
