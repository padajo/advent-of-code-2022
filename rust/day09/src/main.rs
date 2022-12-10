use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::thread;

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

        let mov = match direction.as_str() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid direction"),
        };

        let new_position = Point::new(x + mov.0, y + mov.1);
        self.knots[0] = new_position;
        self.history[0].push(self.knots[0]);

        // let (min, max) = get_grid_min_max();

        // print_grid(&self, min, max, &self.get_unique_visited(1), false, 50);

        if self.knots.len() > 1 {
            for i in 1..self.knots.len() {
                let a_knot = &self.knots[i - 1];
                let b_knot = &self.knots[i];

                // only move if this is true
                if (a_knot.x - b_knot.x).abs() > 1 || (a_knot.y - b_knot.y).abs() > 1 {
                    let mut p: Point = Point::new(0, 0);

                    // there are 8 places the knot can go to
                    p.x = a_knot.x - b_knot.x;
                    p.y = a_knot.y - b_knot.y;

                    // make sure x and y are only 1 or -1
                    if p.x.abs() > 0 {
                        p.x /= p.x.abs();
                    }
                    if p.y != 0 {
                        p.y /= p.y.abs();
                    }

                    let new_point = Point::new(b_knot.x + p.x, b_knot.y + p.y);
                    self.knots[i] = new_point;
                    self.history[i].push(new_point);
                }
            }

            // print_grid(&self, min, max, &self.get_unique_visited(1), false, 50);
        }
    }

    // fn get_unique_visited(&self, knot_index: usize) -> HashSet<String> {
    //     let mut visited = HashSet::new();
    //     for knot in &self.history[knot_index] {
    //         visited.insert(format!("{}", knot));
    //     }
    //     visited
    // }
}

// fn print_grid(
//     rope: &Rope,
//     min_point: Point,
//     max_point: Point,
//     visited: &HashSet<String>,
//     final_grid: bool,
//     millis: u64,
// ) -> () {
//     print!("\x1B[2J\x1B[1;1H");
//     let max_i = max_point.x;
//     let max_j = max_point.y;
//     let min_i = min_point.x;
//     let min_j = min_point.y;
//     let tail_index = rope.knots.len() - 1;
//     let hx = rope.knots[0].x.clone();
//     let hy = rope.knots[0].y.clone();
//     let tx = rope.knots[tail_index].x.clone();
//     let ty = rope.knots[tail_index].y.clone();

//     for j in min_j..max_j {
//         for i in min_i..max_i {
//             let x = i; // this is the x axis value
//             let y = max_j - j - 1 - min_j.abs(); // this is the y axis value
//             let mut knot_displayed = false;
//             let mut knot_num: String = "".to_string();
//             for n in 0..rope.knots.len() {
//                 let knot = &rope.knots[n];
//                 if knot.x == x && knot.y == y {
//                     knot_num = format!("{}", n);
//                     knot_displayed = true;
//                 }
//             }
//             if knot_displayed {
//                 print!("{}", knot_num);

//             // } else if tail_visited.contains(&format!("{},{}", x, y)) {
//             //     print!("o");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
//     println!("");
//     thread::sleep(std::time::Duration::from_millis(millis));
// }

// fn get_grid_min_max() -> (Point, Point) {
//     (Point::new(-20, -10), Point::new(20, 20))
// }

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

    let mut rope = Rope::new(10);

    // run the commands
    for command in moves {
        rope.move_knots(&command.0, &command.1);
    }

    // let (min, max) = get_grid_min_max();

    // print_grid(&rope, min, max, &rope.get_unique_visited(1), true, 50);

    let tail_history = &rope.history[9];

    // println!("Tail history: {:?}", tail_history);
    let mut tail_visited: HashSet<String> = HashSet::new();
    for point in tail_history {
        tail_visited.insert(point.to_string());
    }

    // println!("Tail visited: {:?}", tail_visited);
    println!("Tail visited count: {}", tail_visited.len());
}
