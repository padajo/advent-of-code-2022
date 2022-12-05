use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct AssignmentPair {
    a1_start: i32,
    a1_end: i32,
    a2_start: i32,
    a2_end: i32,
}

impl AssignmentPair {
    fn new(a1_start: i32, a1_end: i32, a2_start: i32, a2_end: i32) -> AssignmentPair {
        AssignmentPair {
            a1_start,
            a1_end,
            a2_start,
            a2_end,
        }
    }

    fn is_containing_overlapping_assignments(&self) -> bool {
        let a = get_hashset(&self.a1_start, &self.a1_end);
        let b = get_hashset(&self.a2_start, &self.a2_end);

        // check if a is a subset of b
        if a.is_subset(&b) {
            return true;
        } else {
            // check if b is a subset of a
            return b.is_subset(&a);
        }
    }
}

fn get_hashset(start: &i32, end: &i32) -> HashSet<i32> {
    let mut hs = HashSet::new();
    for i in *start..*end + 1 {
        hs.insert(i);
    }
    hs
}

fn get_assignment_vec(elf_assignment: &str) -> (i32, i32) {
    let (start_str, end_str) = elf_assignment.split_at(elf_assignment.find('-').unwrap());
    let start = start_str.parse::<i32>().unwrap();
    // remove first character ('-') from end_str
    let mut chars = end_str.chars();
    chars.next();
    let end = chars.as_str().parse::<i32>().unwrap();
    (start, end)
}

fn main() {
    println!("Advent of code 2022, Day 4");

    let mut overlapping_count: i32 = 0;

    let mut assignment_pairs: Vec<AssignmentPair> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            // println!("{}", s);
            let assignments: Vec<&str> = s.split(",").collect::<Vec<&str>>();
            let (a1_start, a1_end) = get_assignment_vec(&assignments[0]);
            let (a2_start, a2_end) = get_assignment_vec(&assignments[1]);
            let assignment_pair: AssignmentPair =
                AssignmentPair::new(a1_start, a1_end, a2_start, a2_end);
            assignment_pairs.push(assignment_pair);
            // println!(
            //     "Overlapping: {}\n",
            //     assignment_pair.is_containing_overlapping_assignments()
            // );
            if assignment_pair.is_containing_overlapping_assignments() {
                overlapping_count += 1;
            }
        }
    }

    println!("Overlapping Count: {:?}", overlapping_count);
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
