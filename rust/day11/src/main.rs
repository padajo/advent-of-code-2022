use std::collections::VecDeque;
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

struct Monkeys {
    monkeys: Vec<Monkey>,
    divisors: Vec<u16>,
}

impl Monkeys {
    fn new() -> Monkeys {
        Monkeys {
            monkeys: vec![],
            divisors: vec![],
        }
    }

    fn load_monkey_data(&mut self, filename: &str) {
        let mut current_index: usize = 0;

        let mut monkey: Monkey = Monkey {
            index: current_index,
            items: VecDeque::new(),
            operation: ("".to_string(), "".to_string()),
            test: ("".to_string(), 0, 0, 0),
            inspection_count: 0,
            max_modulo: 0,
        };

        let mut divisors: Vec<u16> = vec![];

        // load all the monkeys...
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    if l.trim().len() == 0 {
                        continue;
                    }
                    let mut words: Vec<String> =
                        l.split_whitespace().map(|s| s.to_string()).collect();
                    let start = l[0..6].trim();
                    match start {
                        "Monkey" => {
                            let monkey_index_str = words.get_mut(1).unwrap();
                            monkey_index_str.pop();
                            let monkey_index: usize = monkey_index_str.parse().unwrap();
                            monkey.index = monkey_index;
                            // println!("{:?}", monkey_index);
                        }
                        "Star" => {
                            let items: Vec<u128> = words
                                .split_off(2)
                                .into_iter()
                                .map(|mut s| {
                                    if s.ends_with(",") {
                                        s.pop();
                                    }
                                    s.parse::<u128>().unwrap()
                                })
                                .collect();
                            for item in items {
                                monkey.items.push_back(item);
                            }
                            // println!("{:?}", monkey.items);
                        }
                        "Oper" => {
                            let operation = words.split_off(4);
                            let operation_data = (
                                operation.get(0).unwrap().clone(),
                                operation.get(1).unwrap().clone(),
                            );
                            monkey.operation = operation_data;
                            // println!("{:?}", monkey.operation);
                        }
                        "Test" => {
                            let test_data = words.split_off(1);
                            monkey.test.0 = format!(
                                "{} {}",
                                test_data.get(0).unwrap().clone(),
                                test_data.get(1).unwrap().clone()
                            );
                            let divisor = test_data.get(2).unwrap().parse::<u128>().unwrap();
                            divisors.push(test_data.get(2).unwrap().parse::<u16>().unwrap());
                            monkey.test.1 = divisor;
                            // println!("{:?}", monkey.test);
                        }
                        "If" => {
                            match words.get(1).unwrap().as_str() {
                                "true:" => {
                                    monkey.test.2 = words.get(5).unwrap().parse::<usize>().unwrap();
                                }
                                "false:" => {
                                    monkey.test.3 = words.get(5).unwrap().parse::<usize>().unwrap();
                                    // this is the last line, so add it
                                    self.add_monkey(monkey);
                                    // make a new monkey for the next loop
                                    current_index += 1;
                                    monkey = Monkey {
                                        index: current_index,
                                        items: VecDeque::new(),
                                        operation: ("".to_string(), "".to_string()),
                                        test: ("".to_string(), 0, 0, 0),
                                        inspection_count: 0,
                                        max_modulo: 0,
                                    };
                                }
                                _ => {
                                    println!("Unknown");
                                }
                            }
                            // println!("{:?}", monkey.test);
                        }
                        _ => {
                            println!("Unknown");
                        }
                    }
                }
            }
        }

        self.divisors = divisors;
        let mut n = 1;
        for d in self.divisors.clone() {
            n *= d as u128;
        }
        // set the max number for an item
        for m in self.monkeys.iter_mut() {
            m.max_modulo = n;
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn do_round(&mut self) {
        // go in turn through each monkey
        // inspect each of their items in turn
        let monkey_count = self.monkeys.len();
        for m in 0..monkey_count {
            // loop over the number of items the monkey has
            for _ in 0..self.monkeys[m].items.len() {
                // always take the first one (index 0) as we are removing it
                // later in the loop
                let (_test_passed, monkey_to_send_to_index) =
                    self.monkeys[m].inspect_item_and_test(0);
                // take the first item (index 0), and give it to the new monkey
                let item = self.monkeys[m].items.pop_front().unwrap();
                self.monkeys[monkey_to_send_to_index].items.push_back(item);
            }
        }
        // consolidate here
        // this won't reduce the actual number for "worry" purposes
        // but will for ability to complete the task
        // if the number is a multiple of any of the *square* of the
        // divisors (e.g. 9 is 3 squared or 121 is 11 squared) then
        // you can divide it by that divisor, because they are all
        // prime numbers and you are not affecting the result
        // of any future results
        for m in 0..monkey_count {
            let item_count = self.monkeys[m].items.len();
            for n in 0..item_count {
                let num = self.monkeys[m].items[n].clone();
                let new_num = self.consolidate_item(num.clone());
                self.monkeys[m].items[n] = new_num;
            }
        }
    }

    fn consolidate_item(&self, item: u128) -> u128 {
        // modulo is a wraparound operation.
        // we can simply loop around the product of all the
        // divisors. If the number is larger, we can simply
        // take away the product of all the divisors and the
        // result will stay the same
        let mut n = 1;
        let mut new_value = item.clone();
        for d in self.divisors.clone() {
            n *= d as u128;
        }
        while new_value > n {
            new_value -= n;
        }
        new_value
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: VecDeque<u128>,
    operation: (String, String), // new = (do operation.0) on old with (operation.1) e.g. ("+", 1)
    test: (String, u128, usize, usize), // e.g. ("divisible by", num, monkey to send to if true, monkey to send to if false)
    inspection_count: u32,
    max_modulo: u128,
}

impl Monkey {
    fn inspect_item_and_test(&mut self, item_index: usize) -> (bool, usize) {
        self.do_operation(item_index);
        self.reduce_worry(item_index);
        let test_passed = self.is_test_true(item_index);
        self.inspection_count += 1;
        if test_passed {
            return (true, self.test.2);
        } else {
            return (false, self.test.3);
        }
    }

    fn is_test_true(&mut self, item_index: usize) -> bool {
        let item = &mut self.items[item_index];
        match self.test.0.as_str() {
            "divisible by" => {
                if *item % self.test.1 == 0 {
                    return true;
                }
            }
            _ => {
                println!("Unknown test: {}", self.test.0);
            }
        }
        return false;
    }

    fn reduce_worry(&mut self, _item_index: usize) {
        ()
        // let item = &mut self.items[item_index];
        // let mut b: f32 = *item as f32;
        // b = b / 3.0_f32;
        // *item = b.floor() as u32;
    }

    fn do_operation(&mut self, item_index: usize) {
        let item = &mut self.items[item_index];
        let mut num: u128 = item.clone();

        if !self.operation.1.eq("old") {
            num = self.operation.1.parse().unwrap();
        }
        match self.operation.0.as_str() {
            "+" => {
                *item += num as u128;
            }
            "-" => {
                *item -= num as u128;
            }
            "*" => {
                *item *= num as u128;
            }
            "/" => {
                *item /= num as u128;
            }
            _ => {
                println!("Unknown operation: {}", self.operation.0);
            }
        }
        while *item > self.max_modulo {
            *item -= self.max_modulo;
        }
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

    let mut monkeys = Monkeys::new();

    monkeys.load_monkey_data(&filename);

    // for m in &monkeys.monkeys {
    //     println!("{:?}", m);
    // }

    // println!("{:?}", monkeys.monkeys);

    let monkey_count = monkeys.monkeys.len();
    for i in 1..=10000 {
        monkeys.do_round();
        let mut inspection_count: Vec<u128> = Vec::new();

        for m in 0..monkey_count {
            inspection_count.push(monkeys.monkeys[m].inspection_count as u128);
        }

        if i == 20 || i % 1000 == 0 {
            println!("\nRound {}", i);
            for m2 in 0..monkey_count {
                println!(
                    "Monkey {}: [{:>3}]",
                    monkeys.monkeys[m2].index,
                    monkeys.monkeys[m2].inspection_count,
                    // monkeys.monkeys[m].items
                );
            }
            println!("");
            // now output what data is needed
            inspection_count.sort();
            let highest = inspection_count.pop().unwrap();
            let next_highest = inspection_count.pop().unwrap();
            println!(
                "Monkey Business: {} * {} = {}",
                highest,
                next_highest,
                highest * next_highest
            );
            println!("");
        }
    }
}
