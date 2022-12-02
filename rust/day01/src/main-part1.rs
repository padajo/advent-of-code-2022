use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    let mut calories: i32 = 0;
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let elf = s.split("\n\n");
            let mut i: i32 = 1;
            for cals in elf {
                let elfcals = cals.trim().split("\n");
                let mut sum = 0;
                for calorie in elfcals {
                    let num = calorie.parse::<i32>().unwrap();
                    sum += num;
                }
                println!("elf {}: {}", i, sum);
                i += 1;
                if sum > calories {
                    calories = sum;
                    println!("**** new record: {} ****", calories);
                }
            }
        }
    }

    println!("Calories: {}", calories);
}
