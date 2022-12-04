use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn translate_char_to_priority(c: &char) -> u16 {
    let b = *c as u16;
    if b > 64 && b < 91 {
        b - 64 + 26
    } else {
        b - 96
    }
}

fn main() {
    println!("Advent of code 2022, Day 2");

    let mut priorities: Vec<u16> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            // split line in half into two char vecs
            let mut compartment_1: Vec<char> = line.unwrap().chars().collect();
            let compartment_2: Vec<char> = compartment_1.split_off(compartment_1.len() / 2);
            // find intersection of the two char vecs, which should give only one char
            let unique_a = compartment_1.iter().collect::<HashSet<_>>();
            let unique_b = compartment_2.iter().collect::<HashSet<_>>();
            let intersection: Vec<char> = unique_a.intersection(&unique_b).map(|x| **x).collect();
            // find the value (according to the rules) of the char
            let priority = translate_char_to_priority(&intersection[0]);
            // add the value to the set of priorities
            priorities.push(priority);

            println!(
                "{} - {}: {} : {} -> {:?} -> {}",
                compartment_1.len(),
                compartment_2.len(),
                compartment_1.into_iter().collect::<String>(),
                compartment_2.into_iter().collect::<String>(),
                intersection,
                priority
            );
        }
    }

    println!("Priorities: {:?}", priorities);

    println!("Sum of priorities: {}", priorities.iter().sum::<u16>());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
