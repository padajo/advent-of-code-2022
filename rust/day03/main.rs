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

fn get_intersection(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let unique_a = a.iter().collect::<HashSet<_>>();
    let unique_b = b.iter().collect::<HashSet<_>>();
    let intersection: Vec<char> = unique_a.intersection(&unique_b).map(|x| **x).collect();
    intersection
}

fn main() {
    println!("Advent of code 2022, Day 2");

    let mut priorities: Vec<u16> = Vec::new();

    let mut elf_sacks: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            let chars: Vec<char> = s.chars().collect();
            // split line in half into two char vecs

            elf_sacks.push(chars);

            if elf_sacks.len() < 3 {
                continue;
            }
            // at this point we should have a set of 3 elf sacks...
            let intersection = get_intersection(elf_sacks[0].clone(), elf_sacks[1].clone());
            let intersection2 = get_intersection(intersection.clone(), elf_sacks[2].clone());
            println!("Intersection: {:?}", intersection2);

            let priority = translate_char_to_priority(&intersection2[0]);
            // add the value to the set of priorities
            priorities.push(priority);

            // clear the elf sacks
            elf_sacks.clear();
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
