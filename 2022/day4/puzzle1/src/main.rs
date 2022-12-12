use std::fs;

fn main() {
    let assignments = fs::read_to_string("input.txt").unwrap();
    let mut num_overlaps = 0;

    for assignment in assignments.lines() {
        let (assignment1, assignment2) = assignment.split_once(",").unwrap();
        let (start1, end1) = assignment1.split_once("-").unwrap();
        let (start2, end2) = assignment2.split_once("-").unwrap();

        let start1 = start1.parse::<i32>().unwrap();
        let start2 = start2.parse::<i32>().unwrap();

        let end1 = end1.parse::<i32>().unwrap();
        let end2 = end2.parse::<i32>().unwrap();

        if start1 <= start2 && end1 >= end2 {
            num_overlaps += 1;

        } else if start2 <= start1 && end2 >= end1 {
            num_overlaps += 1;
        }
    }

    println!("There are {} overlapping assignments", num_overlaps);
}
