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

    let mut size = 0;

    for dir in &dirs {
        if dir.borrow().size <= 100000 {
            size += dir.borrow().size;
        }
    }

    println!("The sum of all dirs smaller than 100000 is {}", size);
}
