use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stacks = vec![vec![]; 9];
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if &line[1..2] == "1" { break; }

        for (stack_num, stack) in stacks.iter_mut().enumerate() {
            let start = stack_num * 4;
            let end = start + 3;

            if end > line.len() { break; }
            let supply_crate = &line[start..end];

            if &supply_crate[0..1] != "[" { continue; }
            stack.insert(0, supply_crate);
        }
    }

    lines.next();

    for line in lines {
        let step: Vec<&str> = line.split(' ').collect();

        let quantity = step[1].parse::<i32>().unwrap();
        let from = step[3].parse::<usize>().unwrap() - 1;
        let to = step[5].parse::<usize>().unwrap() - 1;

        for _ in 0..quantity {
            let supply_crate = stacks[from].pop().unwrap();
            stacks[to].push(supply_crate);
        }
    }

    let mut top_crates = String::new();

    for stack in stacks {
        top_crates += &stack.last().unwrap()[1..2];
    }

    println!("The top crates are {}", top_crates);
}
