use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

struct Rope {
    knots: Vec<Point>, // model as Vec of connected Points
    history: Vec<Vec<Point>>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: vec![Point::new(0, 0); size],
            history: vec![vec![Point::new(0, 0)]; size],
        }
    }

    fn move_knots(&mut self, direction: &String, distance: &u32) {
        for _i in 0..*distance {
            self.move_head(direction);
        }
    }

    fn move_head(&mut self, direction: &String) {
        // only move first Point in Rope
        // then propagate if it needs to move

        let head_position = &self.knots[0];
        let x: i32 = head_position.x;
        let y: i32 = head_position.y;

        // move the head to the new position
        match direction.as_str() {
            "U" => {
                self.knots[0] = Point::new(x, y + 1);
            }
            "D" => {
                self.knots[0] = Point::new(x, y - 1);
            }
            "L" => {
                self.knots[0] = Point::new(x - 1, y);
            }
            "R" => {
                self.knots[0] = Point::new(x + 1, y);
            }
            _ => {
                println!("Invalid direction");
            }
        }
        self.history[0].push(self.knots[0]);

        // print_grid(
        //     &self,
        //     Point::new(-5, -5),
        //     Point::new(10, 10),
        //     &self.get_unique_visited(1),
        //     false,
        //     50,
        // );
        if self.knots.len() > 1 {
            self.move_next(1);
            // print_grid(
            //     &self,
            //     Point::new(-5, -5),
            //     Point::new(10, 10),
            //     &self.get_unique_visited(1),
            //     false,
            //     50,
            // );
        }
    }

    fn move_next(&mut self, knot_index: usize) {
        // move the next knot in the rope
        // if it needs to move
        let knots = &mut self.knots;
        let k_pos = knots[knot_index];

        let ahead_index = knot_index - 1;
        let x: i32 = k_pos.x;
        let y: i32 = k_pos.y;
        let ak_pos = &self.knots[ahead_index];
        let ax: i32 = ak_pos.x;
        let ay: i32 = ak_pos.y;

        // this is the key to the problem
        if (ax - x).abs() > 1 || (ay - y).abs() > 1 {
            // move it to the ahead knot's previous position
            let ak_pre_pos = self.history[ahead_index][self.history[ahead_index].len() - 2];
            let p = Point::new(ak_pre_pos.x, ak_pre_pos.y);
            self.knots[knot_index] = p;
            self.history[knot_index].push(p);
        }

        // move the next knot if there is one
        if knot_index < self.knots.len() - 1 {
            self.move_next(knot_index + 1);
        }
    }

    fn get_unique_visited(&self, knot_index: usize) -> HashSet<String> {
        let mut visited = HashSet::new();
        for knot in &self.history[knot_index] {
            visited.insert(format!("{}", knot));
        }
        visited
    }
}

fn print_grid(
    rope: &Rope,
    min_point: Point,
    max_point: Point,
    tail_visited: &HashSet<String>,
    final_grid: bool,
    millis: u64,
) -> () {
    print!("\x1B[2J\x1B[1;1H");
    let max_i = max_point.x;
    let max_j = max_point.y;
    let min_i = min_point.x;
    let min_j = min_point.y;
    let tail_index = rope.knots.len() - 1;
    let hx = rope.knots[0].x.clone();
    let hy = rope.knots[0].y.clone();
    let tx = rope.knots[tail_index].x.clone();
    let ty = rope.knots[tail_index].y.clone();

    for j in min_j..max_j {
        for i in min_i..max_i {
            let x = i; // this is the x axis value
            let y = max_j - j - 1 - min_j.abs(); // this is the y axis value
            if hx == x && hy == y && !final_grid {
                print!("H");
            } else if tx == x && ty == y && !final_grid {
                print!("T");
            } else if tail_visited.contains(&format!("{},{}", x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    thread::sleep(std::time::Duration::from_millis(millis));
}

fn main() {
    println!("Advent of code 2022, Day 9 Part 1\n");

    let mut filename = "./test-data.txt".to_string();
    match std::env::args().nth(1) {
        Some(arg) => {
            filename = arg;
        }
        None => {
            println!("No filename given, using test data\n");
        }
    }

    let moves: Vec<(String, u32)> = read_lines(filename)
        .unwrap()
        .map(|line| {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            let data: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
            let count = data[1].parse::<u32>().unwrap();
            (data[0].clone(), count)
        })
        .collect();

    let mut rope = Rope::new(2);

    // run the commands
    for command in moves {
        rope.move_knots(&command.0, &command.1);
    }

    // print_grid(
    //     &rope,
    //     Point::new(-5, -5),
    //     Point::new(10, 10),
    //     &rope.get_unique_visited(1),
    //     true,
    //     50,
    // );

    let tail_history = &rope.history[1];

    // println!("Tail history: {:?}", tail_history);
    let mut tail_visited: HashSet<String> = HashSet::new();
    for point in tail_history {
        tail_visited.insert(point.to_string());
    }
    // println!("Tail visited: {:?}", tail_visited);
    println!("Tail visited count: {}", tail_visited.len());
}
