use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

struct Dir {
    dirs: Vec<Rc<RefCell<Dir>>>,
    size: i32
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let root = Rc::new(RefCell::new(Dir {
        dirs: Vec::new(),
        size: 0
    }));

    let mut dirs = vec![root.clone()];
    let mut path = vec![root];

    for line in input.lines() {
        match &line[0..4] {
            "dir " => { continue; },
            "$ ls" => { continue; },

            "$ cd" => {
                if &line[5..] == ".." {
                    path.pop();
                    continue;
                }

                let new_dir = Rc::new(RefCell::new(Dir {
                    dirs: Vec::new(),
                    size: 0
                }));

                path.last().unwrap().borrow_mut().dirs.push(new_dir.clone());
                dirs.push(new_dir.clone());
                path.push(new_dir);
            },

            _ => {
                let size: i32 = line.split_once(' ').unwrap().0.parse().unwrap();

                for dir in &path {
                    dir.borrow_mut().size += size;
                }
            }
        }
    }

    let free_space = 70000000 - dirs[0].borrow().size;
    let space_needed = 30000000 - free_space;

    let mut possible_dirs = Vec::new();

    for dir in &dirs {
        if dir.borrow().size >= space_needed {
            possible_dirs.push(dir.clone());
        }
    }

    let mut smallest_dir = possible_dirs[0].clone();

    for dir in &possible_dirs {
        if dir.borrow().size < smallest_dir.borrow().size {
            smallest_dir = dir.clone();
        }
    }

    println!("The smallest dir we can delete has the size {}", smallest_dir.borrow().size);
}
