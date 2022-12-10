use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct CPU {
    x: i32,
    cycle: i32,
    current_instruction: (String, i32),
    signal_strength_data: Vec<(i32, i32, i32)>,
    crt: Vec<Vec<char>>,       // either '.' or '#'
    current_crt_position: i32, // where on the crt_row are we? 0-39
    current_crt_row: i32,      // which row of the crt are we on?
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1, // middle pixel of 3
            cycle: 0,
            current_instruction: ("".to_string(), 0),
            signal_strength_data: vec![],
            crt: vec![vec!['.'; 40]; 6], // 6 rows of 40 pixels filled with '.'
            current_crt_position: 0,
            current_crt_row: 1,
        }
    }

    fn drawCRT(&self) {
        for row in self.crt.iter() {
            println!(
                "\t\t[drawCRT] {:?}",
                row.iter().map(|x| x.to_string()).collect::<String>()
            );
        }
    }

    fn writeCRT(&mut self) {
        let pos = (self.cycle - 1) % 40;
        let row = ((self.cycle - pos) + 1) / 40;
        // println!(
        //     "\t\t\t\t\t[drawCRT] crt_position={}, row={}, x={}",
        //     pos, row, self.x
        // );
        // we know the row, and we know the position of X and current_crt_position
        if (self.x - pos as i32).abs() < 2 {
            // then we add a '#' to current row
            self.crt[row as usize][pos as usize] = '#';
        } else {
            // then we add a '.' to current row
            self.crt[row as usize][pos as usize] = '.';
        }
        // println!(
        //     "\t\t\t\t\t[drawCRT] {:?}",
        //     self.crt[row as usize]
        //         .iter()
        //         .map(|x| x.to_string())
        //         .collect::<String>()
        // );
    }

    fn store_signal_strength(&mut self) {
        self.signal_strength_data
            .push((self.cycle, self.x, self.get_signal_strength()));
    }

    fn get_signal_strength(&self) -> i32 {
        self.x * self.cycle
    }

    fn end_cycle(&mut self) {
        println!("[{:0>4}] END (X={})", self.cycle, self.x);
    }

    fn start_cycle(&mut self) {
        println!("[{:0>4}] START (X={})", self.cycle, self.x);
    }

    fn increment_cycle(&mut self) {
        if self.cycle > 0 {
            self.end_cycle();
        }
        self.cycle += 1;
        self.start_cycle();
    }

    fn run(&mut self, program: &Vec<(String, i32)>) {
        println!("Running program");
        for instruction in program {
            match instruction.0.as_str() {
                "addx" => {
                    self.increment_cycle();
                    println!("\t[addx] Start executing addx {}", instruction.1);
                    self.writeCRT();
                    if (self.cycle - 20) % 40 == 0 {
                        // store the signal strength
                        self.store_signal_strength();
                    }
                    self.increment_cycle();
                    self.writeCRT();
                    if (self.cycle - 20) % 40 == 0 {
                        // store the signal strength
                        self.store_signal_strength();
                    }
                    println!("\t[addx] End executing addx {}", instruction.1);
                    println!("\t[addx] Set X to X + ({})", instruction.1);
                    self.x += instruction.1;
                    println!("\t[addx] X set to {}", self.x);
                }
                "noop" => {
                    self.increment_cycle();
                    println!("\t[noop]");
                    self.writeCRT();
                    if (self.cycle - 20) % 40 == 0 {
                        // store the signal strength
                        self.store_signal_strength();
                    }
                }
                _ => println!("Unknown instruction: {}", instruction.0),
            }
        }
        self.end_cycle();
    }
}

fn main() {
    println!("Advent of code 2022, Day 10 Part 1\n");

    let mut filename = "./test-data.txt".to_string();
    match std::env::args().nth(1) {
        Some(arg) => {
            filename = arg;
        }
        None => {
            println!("No filename given, using test data\n");
        }
    }

    let instructions: Vec<(String, i32)> = read_lines(filename)
        .unwrap()
        .map(|line| {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            let instruction_data: Vec<String> =
                s.split_whitespace().map(|s| s.to_string()).collect();
            let instruction: String = instruction_data[0].clone();
            let mut num: i32 = 0;
            if instruction == "addx" {
                num = match instruction_data[1].parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
            }
            (instruction, num)
        })
        .collect();

    let mut cpu = CPU::new();

    cpu.run(&instructions);

    // println!("signal strength: {:?}", cpu.signal_strength_data);

    // let mut sum: i32 = 0;

    // for (cycle, x, signal_strength) in cpu.signal_strength_data {
    //     sum += signal_strength;
    // }

    // println!("sum of total: {}", sum);

    cpu.drawCRT();
}
