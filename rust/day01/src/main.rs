use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
struct Elf {
    i: i16,
    calories: i32,
}

impl Elf {
    fn sort_field(&self) -> i32 {
        self.calories
    }
}

fn main() {
    println!("Advent of code 2022, Day 1");
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    // let mut elves: Vec<i32> = Vec::new();
    let mut elves: Vec<Elf> = Vec::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let elf = s.split("\n\n");
            let mut i: i16 = 1;
            for cals in elf {
                let elfcals = cals.trim().split("\n");
                let mut sum = 0;
                for calorie in elfcals {
                    let num = calorie.parse::<i32>().unwrap();
                    sum += num;
                }
                println!("elf {}: {}", i, sum);
                elves.push(Elf {
                    i: i,
                    calories: sum,
                });
                i += 1;
            }
        }
    }

    elves.sort_by_key(|k| k.sort_field());
    for elf in &elves {
        println!("elf[{:03}]: {}", elf.i, elf.calories);
    }

    let last3 = elves.as_slice()[elves.len() - 3..].to_vec();

    println!("---------------\nLast 3:");
    for elf in &last3 {
        println!("elf[{:03}]: {}", elf.i, elf.calories);
    }

    // The answer I think is: 197291
    println!(
        "Calories of last 3 combined: {}",
        last3[0].calories + last3[1].calories + last3[2].calories
    );
}
