use std::fs;

struct LifoQueue {
    stack: Vec<char>,
    max_size: usize,
}

impl LifoQueue {
    fn new(max_size: usize) -> LifoQueue {
        LifoQueue {
            stack: Vec::new(),
            max_size: max_size,
        }
    }

    fn push(&mut self, c: char) {
        self.stack.push(c);
        if self.len() > self.max_size {
            self.stack.remove(0);
        }
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_unique(&self) -> bool {
        let mut unique = true;
        for c in &self.stack {
            // if the stack contains more than one of this character, it's not unique
            // it's only 14 characters, so we can just check each one
            if self.stack.iter().filter(|&x| x == c).count() > 1 {
                // println!("{} is not unique", c);
                unique = false;
                break;
            }
        }
        unique
    }
}

fn main() {
    println!("Advent of code 2022, Day 6 Part 2");
    let signal_data =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let signal: Vec<char> = signal_data.trim().chars().collect();

    let queue_size = 14;

    let mut q: LifoQueue = LifoQueue::new(queue_size);
    let mut is_found: bool = false;

    for mut _i in 0..signal.len() + 1 {
        q.push(signal[_i]);
        if q.len() == queue_size {
            if q.is_unique() {
                println!("***************************************");
                println!("Unique signal found");
                println!("Stack: {:?}", q.stack);
                println!(
                    "0-Index {} - Position {} - Character {}",
                    _i,
                    _i + 1,
                    signal[_i]
                );
                is_found = true;
                break;
            }
        }
    }

    if !is_found {
        println!("***************************************");
        println!("No unique signal found");
    }
}
