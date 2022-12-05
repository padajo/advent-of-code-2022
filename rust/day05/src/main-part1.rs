use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
    name: String,
}

impl Stack {
    fn new(name: String) -> Stack {
        Stack {
            crates: Vec::new(),
            name: name,
        }
    }

    // add a crate to the top (end) of the stack
    fn add_crate(&mut self, c: char) {
        self.crates.push(c);
    }

    // remove the top (last) crate
    fn remove_crate(&mut self) -> Option<char> {
        self.crates.pop()
    }
}

struct ShipYard {
    stacks: Vec<Stack>,
    is_accepting_commands: bool,
}

impl ShipYard {
    fn new() -> ShipYard {
        ShipYard {
            stacks: Vec::new(),
            is_accepting_commands: false,
        }
    }

    fn get_stack(&mut self, stack_num: u16) -> Option<&mut Stack> {
        for x in &mut self.stacks {
            if x.name == stack_num.to_string() {
                return Some(x);
            }
        }
        None
    }

    fn parse_shipyard_data(&mut self, lines: Vec<String>) -> Result<(), &str> {
        // all the data is held at index = (n*4) + 1 e.g. 1, 5, 9 etc
        let mut firstline: bool = true;
        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            let mut i = 1;
            for n in (1..(chars.len() - 1)).step_by(4) {
                // first line contains the number of stacks, so let's create them
                if firstline {
                    let name: String = (i).to_string();
                    self.stacks.push(Stack::new(name));
                } else {
                    // every other line is a stack to display
                    let x = chars[n];
                    if x != ' ' {
                        let c = self.get_stack(i).unwrap();
                        c.add_crate(x);
                    }
                }
                i += 1;
            }
            firstline = false;
        }
        self.is_accepting_commands = true;
        Ok(())
    }

    fn parse_command(&self, command: &str) -> (u16, u16, u16) {
        // command is "move N from S1 to S2"
        // println!("\nCommand: {}", command);
        // self.display_shipyard_status();
        let command_parts: Vec<&str> = command.split(' ').collect::<Vec<&str>>();
        // println!("Command parts: {:?}", command_parts);
        let num_crates = command_parts[1].parse::<u16>().unwrap();
        let from_stack = command_parts[3].parse::<u16>().unwrap();
        let to_stack = command_parts[5].parse::<u16>().unwrap();
        // return the number of crates to move, the from stack, and the to stack
        (num_crates, from_stack, to_stack)
    }

    fn display_top_of_each_stack(&self) {
        let mut s: String = String::new();
        for stack in &self.stacks {
            let top = stack.crates.last();
            match top {
                Some(x) => s.push(*x),
                None => print!("  "),
            }
        }
        println!("Crates at top of each stack (from first to last): {}", s);
    }

    fn display_shipyard_status(&self) {
        println!("Shipyard status:");
        for stack in &self.stacks {
            let mut crate_chars: Vec<String> = Vec::new();
            for c in stack.crates.iter() {
                crate_chars.push(c.to_string().clone());
            }
            println!(
                "{}: [{}] {}",
                stack.name,
                crate_chars.len(),
                crate_chars.join(" ")
            );
        }
        println!("-----");
    }

    fn execute_command(&mut self, command: (u16, u16, u16)) -> Result<(), &str> {
        let (num_crates, from_stack, to_stack) = command;

        // move the crates from the from stack to the to stack
        for _ in 0..num_crates {
            let c = self.get_stack(from_stack).unwrap().remove_crate().unwrap();
            self.get_stack(to_stack).unwrap().add_crate(c);
        }
        Ok(())
    }
}

fn main() {
    println!("Advent of code 2022, Day 5");
    let mut shipyard = ShipYard::new();
    let mut shipyard_data: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            // push data onto shipyard data vec until we hit a blank line or shipyard is accepting commands
            if (!s.trim().is_empty()) && !shipyard.is_accepting_commands {
                shipyard_data.push(s);
                continue;
            } else if !shipyard.is_accepting_commands {
                // start from the bottom of the crates and work up
                shipyard_data.reverse();
                if Ok(()) == shipyard.parse_shipyard_data(shipyard_data.clone()) {
                    // shipyard will now be parsing data
                    println!("*******\nShipyard is accepting commands\n*******");
                    shipyard.display_shipyard_status();
                }
                continue;
            }
            // we get here when the shipyard is accepting commands and the data has been parsed
            let command = shipyard.parse_command(s.as_str());
            if Ok(()) == shipyard.execute_command(command) {
                // use to visually check if the program is doing what we want
                //shipyard.display_shipyard_status();
            }
        }
    }
    println!("*******\nShipyward commands completed\n*******");
    shipyard.display_shipyard_status();
    shipyard.display_top_of_each_stack();
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
